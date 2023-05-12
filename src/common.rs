use std::{path::Path, fs::File, io::Read};

/**
 * Read the file from input into a string
 */
pub fn read_file(input: &str) -> String {
    let path = Path::new(input);
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open; {}", why),
        Ok(file) => file,
    };

    let mut s = String::new();
    file.read_to_string(&mut s).expect("unable to read file");

    s
}

/**
 * Split the file on new lines and remove any lines with 0 length
 */
pub fn parse_lines(s: &str) -> Vec<&str> {
    s.split('\n').filter(|s| { s.trim().len() > 0}).collect()
}

mod test {
    use crate::common::parse_lines;

    #[test]
    fn test_parse_lines() {
        let input =
"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";
        let expected_output = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ];

        assert_eq!(parse_lines(input), expected_output);
    }
}
