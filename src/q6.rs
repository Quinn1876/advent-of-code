use std::collections::{HashSet, hash_map::RandomState};

use crate::common::read_file;

pub fn run() {
    let input = "./puzzle_6.input";
    let data_stream = read_file(input);
    let result: usize = process_data_stream(&data_stream, 4);
    println!("The packet starts at: {}", result);
    let result: usize = process_data_stream(&data_stream, 14);
    println!("The message starts at: {}", result);
}

struct Buffer {
    len: usize,
    max_len: usize,
    vec: Vec<char>,
    head: usize,
}

impl Buffer {
    pub fn new(max_length: usize) -> Buffer {
        let mut v = Vec::new();
        v.resize(max_length, ' ');
        Buffer {
            len: 0,
            max_len: max_length,
            vec: v,
            head: 0
        }
    }

    pub fn push(&mut self, c: char) {
        self.len += 1;
        if self.len > self.max_len {
            self.len = self.max_len;
            self.head += 1;
            self.head %= self.max_len;
        }

        *self.vec.get_mut((self.head + self.len) % self.max_len).unwrap() = c;
    }

    pub unsafe fn get_vec(&mut self) -> &mut Vec<char> {
        &mut self.vec
    }

    pub fn is_unique(&self) -> bool {
        if self.max_len != self.len { return false; }
        let hash: HashSet<&char, RandomState> = HashSet::from_iter(self.vec.iter());
        hash.len() == self.len
    }

}
pub fn process_data_stream(input: &str, num_unique: usize) -> usize {
    let mut buf = Buffer::new(num_unique);
    for (index, character) in input.chars().enumerate() {
        buf.push(character);
        if buf.is_unique() {
            return index + 1;
        }
    }
    0
}

mod test {
    use super::{Buffer, process_data_stream};

    #[test]
    fn test_push() {
        let mut buffer = Buffer::new(4);
        let expected_vec = vec![' ', ' ', ' ', ' '];
        assert_eq!(*unsafe { buffer.get_vec() }, expected_vec);

        buffer.push('A');
        let expected_vec = vec![' ', 'A', ' ', ' '];
        assert_eq!(*unsafe { buffer.get_vec() }, expected_vec);

        buffer.push('B');
        let expected_vec = vec![' ', 'A', 'B', ' '];
        assert_eq!(*unsafe { buffer.get_vec() }, expected_vec);

        buffer.push('C');
        let expected_vec = vec![' ', 'A', 'B', 'C'];
        assert_eq!(*unsafe { buffer.get_vec() }, expected_vec);

        buffer.push('D');
        let expected_vec = vec!['D', 'A', 'B', 'C'];
        assert_eq!(*unsafe { buffer.get_vec() }, expected_vec);

        buffer.push('E');
        let expected_vec = vec!['D', 'E', 'B', 'C'];
        assert_eq!(*unsafe { buffer.get_vec() }, expected_vec);

    }

    #[test]
    fn test_process_data_stream() {
        let input_1 = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let input_2 = "nppdvjthqldpwncqszvftbrmjlhg";
        let input_3 = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let input_4 = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

        let expected_output_1 = 5;
        let expected_output_2 = 6;
        let expected_output_3 = 10;
        let expected_output_4 = 11;

        assert_eq!(process_data_stream(input_1, 4), expected_output_1);
        assert_eq!(process_data_stream(input_2, 4), expected_output_2);
        assert_eq!(process_data_stream(input_3, 4), expected_output_3);
        assert_eq!(process_data_stream(input_4, 4), expected_output_4);
    }
}
