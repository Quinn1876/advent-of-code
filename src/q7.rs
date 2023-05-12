use std::{io::{BufRead, Split}, fmt::Error, slice::Iter};

use crate::common::{parse_lines, self};

#[derive(PartialEq, Debug)]
struct FileSystem {
    pub root_directory: Directory
}

#[derive(PartialEq, Debug)]
struct Directory {
    pub child_directories: Vec<Directory>,
    pub files: Vec<File>,
    pub name: Vec<u8>,
    pub size: u64
}

#[derive(PartialEq, Debug)]
struct File {
    pub size: u64
}

impl FileSystem {
    pub fn new() -> Self {
        Self {
            root_directory: Directory::new()
        }
    }
}

impl Directory {
    pub fn new() -> Self {
        Self {
            child_directories: Vec::new(),
            files: Vec::new(),
            name: "/".as_bytes().to_vec(),
            size: 0
        }
    }

    pub fn with_name(name: Vec<u8>) -> Self {
        let mut d = Self::new();
        d.name = name;
        d
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    DirectoryName(Vec<u8>),
    ListDirectoryLine(Vec<Vec<u8>>)
}

fn into_token_stream(lines: &Vec<&str>) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    for &line in lines {
        let line: &[u8] = line.as_bytes();
        let chunks = line.split(|&c| c == b' ');
        if line[0] == b'$' {
            for chunk in chunks {
                if chunk[0] == b'c' && chunk[1] == b'd' && chunk.len() == 2 {
                    continue;
                }
                if chunk[0] == b'l' && chunk[1] == b's' && chunk.len() == 2 {
                    continue;
                }
                if chunk[0] != b'$' || chunk.len() > 1  {
                    tokens.push(Token::DirectoryName(chunk.into()));
                }
            }
        }
        else {
            let mut chunk_vec: Vec<Vec<u8>> = Vec::new();
            for chunk in chunks {
                chunk_vec.push(chunk.into());
            }
            tokens.push(Token::ListDirectoryLine(chunk_vec));
        }

    }

    tokens
}

impl FileSystem {
    pub fn parse_tokens(tokens: &Vec<Token>) -> FileSystem {
        let mut fs = FileSystem::new();
        let mut current_directory = &mut fs.root_directory;
        let mut token_iter = tokens.into_iter();
        while FileSystem::parse_directory(&mut current_directory, &mut token_iter) {}

        fs
    }
    fn parse_directory(directory: &mut Directory, token_iter: &mut Iter<Token>) -> bool {
        loop {
            let token = token_iter.next();
            if let Some(token) = token {
                match token {
                    Token::DirectoryName(name) => {
                        let ddot: Vec<u8> = b"..".to_vec();
                        let name = name.to_owned();
                        if name == ddot {
                            return false
                        }
                        if name == vec![b'/'] {
                            return true;
                        }
                        let next_dir: Option<&mut Directory> = directory.child_directories.iter_mut().find(|child| child.name == name);
                        let goto_root;
                        if let Some(dir) = next_dir {
                            goto_root = FileSystem::parse_directory(dir, token_iter);

                        } else {
                            let mut next_dir = Directory::with_name(name);
                            goto_root = FileSystem::parse_directory(&mut next_dir, token_iter);
                            directory.child_directories.push(next_dir);
                        }
                        if goto_root {
                            return true
                        }
                    },
                    Token::ListDirectoryLine(line) => {
                        if line.len() < 2 { panic!("Found List directory line with less then 2 entries")}
                        let first = line[0].clone();
                        let second = line[1].clone();

                        let size = String::from_utf8(first).unwrap().parse::<u64>();
                        if let Ok(size) = size {
                            directory.files.push(File{
                                size: size
                            });
                        } else {
                            directory.child_directories.push(Directory::with_name(second))
                        }
                    }
                }
            } else {
                return false;
            }
        }
    }
}

impl Directory {
    pub fn get_size(&self) -> u64 {
        let sum1: u64 = self.files.iter().map(|file| file.size).sum();
        let sum2: u64 = self.child_directories.iter().map(|dir| dir.get_size()).sum();
        sum1 + sum2
    }

    pub fn get_sum_less_than(&self, n: u64) -> u64 {
        let size = self.get_size();
        let sum_less: u64 = self.child_directories.iter().map(|dir| dir.get_sum_less_than(n)).sum();
        if size < n {
            sum_less + size
        } else {
            sum_less
        }
    }

    pub fn find_smallest_child_less_than(&self, n: u64) -> u64 {
        let smallest_child = self.child_directories.iter().map(|child| child.find_smallest_child_less_than(n)).filter(|&size| size > n).min();
        if smallest_child.is_none() || smallest_child.unwrap() < n {
            self.get_size()
        } else {
            smallest_child.unwrap()
        }
    }
}

pub fn solve_q7() {
    let input_file_name = "./puzzle_7.input";
    let file = common::read_file(input_file_name);
    let lines = common::parse_lines(&file);

    let tokens = into_token_stream(&lines);
    let fs = FileSystem::parse_tokens(&tokens);

    let root_dir = fs.root_directory;
    println!("Sum of directories' sizes with individual size less than 100000 is: {}", root_dir.get_sum_less_than(100000));
    let root_dir_size = root_dir.get_size();
    let space_available = 70000000 - root_dir_size;
    let space_required = 30000000 - space_available;
    let answer2 = root_dir.find_smallest_child_less_than(space_required);
    println!("Size of directory to yeet is: {}", answer2);
}


mod test {
    use crate::q7::File;

    use super::{Token, into_token_stream, FileSystem, Directory};
    use std::{iter::zip, vec};

    #[test]
    fn test_into_token_stream() {
        let input = vec![
            "$ cd /",
            "$ ls",
            "dir a",
            "14848514 b.txt",
            "8504156 c.dat",
            "dir d",
        ];

        let expected_output = vec![
            Token::DirectoryName("/".as_bytes().to_vec()),
            Token::ListDirectoryLine(vec!["dir".as_bytes().to_vec(), "a".as_bytes().to_vec()]),
            Token::ListDirectoryLine(vec!["14848514".as_bytes().to_vec(), "b.txt".as_bytes().to_vec()]),
            Token::ListDirectoryLine(vec!["8504156".as_bytes().to_vec(), "c.dat".as_bytes().to_vec()]),
            Token::ListDirectoryLine(vec!["dir".as_bytes().to_vec(), "d".as_bytes().to_vec()]),
        ];

        let output = into_token_stream(&input);

        assert_eq!(output.len(), expected_output.len());
        for cmp in zip(output, expected_output) {
            assert_eq!(cmp.0, cmp.1);
        }
    }

    #[test]
    fn test_parse_token_stream() {

        let input_token_stream = vec![
            Token::DirectoryName("/".as_bytes().to_vec()),
            Token::ListDirectoryLine(vec!["dir".as_bytes().to_vec(), "a".as_bytes().to_vec()]),
            Token::ListDirectoryLine(vec!["14848514".as_bytes().to_vec(), "b.txt".as_bytes().to_vec()]),
            Token::ListDirectoryLine(vec!["8504156".as_bytes().to_vec(), "c.dat".as_bytes().to_vec()]),
            Token::ListDirectoryLine(vec!["dir".as_bytes().to_vec(), "d".as_bytes().to_vec()]),
        ];

        let expected_output = FileSystem {
            root_directory: Directory {
                name: vec![b'/'],
                files: vec![
                    File {
                        size: 14848514
                    },
                    File {
                        size: 8504156
                    }
                ],
                child_directories: vec![
                    Directory::with_name(vec![b'a']),
                    Directory::with_name(vec![b'd']),
                ],
                size: 0
            }
        };

        let fs = FileSystem::parse_tokens(&input_token_stream);

        assert_eq!(fs, expected_output);
    }

    #[test]
    fn test_ddot() {
        let input = vec![
            "$ cd /",
            "$ ls",
            "dir a",
            "14848514 b.txt",
            "8504156 c.dat",
            "dir d",
            "$ cd a",
            "$ ls",
            "dir e",
            "29116 f",
            "2557 g",
            "62596 h.lst",
            "$ cd e",
            "$ ls",
            "584 i",
            "$ cd ..",
            "$ cd ..",
            "$ cd d",
            "$ ls",
            "4060174 j",
            "8033020 d.log",
            "5626152 d.ext",
            "7214296 k"
        ];

        let tokens = into_token_stream(&input);

        let fs = FileSystem::parse_tokens(&tokens);

        let expected_fs = FileSystem {
            root_directory: Directory {
                size: 0,
                child_directories: vec![
                    Directory {
                        size: 0,
                        child_directories: vec![
                            Directory {
                                size: 0,
                                child_directories: vec![],
                                files: vec![
                                    File {
                                        size: 584
                                    }
                                ],
                                name: b"e".to_vec()
                            },
                        ],
                        files: vec![
                            File {
                                size: 29116
                            },
                            File {
                                size: 2557
                            },
                            File {
                                size: 62596
                            }
                        ],
                        name: b"a".to_vec()
                    },
                    Directory {
                        size: 0,
                        child_directories: vec![],
                        files: vec![
                            File {
                                size: 4060174,
                            },
                            File {
                                size: 8033020,
                            },
                            File {
                                size: 5626152,
                            },
                            File {
                                size: 7214296,
                            },
                        ],
                        name: b"d".to_vec()
                    },
                ],
                files: vec![
                    File {
                        size: 14848514,
                    },
                    File {
                        size: 8504156
                    }
                ],
                name: b"/".to_vec()
            }
        };

        assert_eq!(fs, expected_fs);

    }
}
