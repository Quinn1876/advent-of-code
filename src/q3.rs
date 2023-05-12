use std::collections::{HashMap, hash_map::RandomState};

use crate::common::{read_file, parse_lines};


fn parse_line(s: &str) -> (&str, &str) {
    s.split_at(s.len() / 2)
}

const ASCII_A: u64 = 65;
const ASCII_Z: u64 = 90;
const ASCII_a: u64 = 97;
const ASCII_z: u64 = 172;
const UPPER_CASE_OFFSET: u64 = ASCII_A - 27;
const LOWER_CASE_OFFSET: u64 = ASCII_a - 1;

fn get_priority(c: char) -> u64 {
    let ascii: u64 = c.try_into().unwrap();
    if ascii >= ASCII_A && ascii <= ASCII_Z {
        ascii - UPPER_CASE_OFFSET
    } else {
        ascii - LOWER_CASE_OFFSET
    }

}

fn get_common_chars(first: &str, second: &str) -> Vec<char> {
    let mut first_chars = HashMap::with_capacity(first.len());
    first.chars().for_each(|c| {
        first_chars.insert(c, true);
    });
    let mut second_chars = HashMap::with_capacity(second.len());
    second.chars().for_each(|c| {
        second_chars.insert(c, true);
    });

    let mut common_chars = Vec::new();
    for key in first_chars.keys() {
        if second_chars.contains_key(key) {
            common_chars.push(*key);
        }
    }
    common_chars
}

fn get_badge(first: &str, second: &str, third: &str) -> char{
    let common12 = HashMap::<_, _, RandomState>::from_iter(get_common_chars(first, second).into_iter().map(|c| { (c, true)}));
    let common23 = get_common_chars(second, third);

    for c in common23 {
        if common12.contains_key(&c) {
            return c
        }
    }
    '\0'
}

pub fn run() {
    let input = "./puzzle_3.input";
    let contents = read_file(input);
    let lines = parse_lines(&contents);
    let mut sum = 0;
    for line in lines {
        let (first, second) = parse_line(line);
        sum += get_priority(*get_common_chars(first, second).first().unwrap());
    }

    println!("The sum of priorities for the common items is {}", sum);
    sum = 0;
    let lines = parse_lines(&contents);

    for set in lines.chunks(3) {
        if set.len() == 3 {
            sum += get_priority(get_badge(set[0], set[1], set[2]))
        }
    }
    println!("The sum of priorities for the badges is {}", sum);
}

mod test {
    use super::{parse_line, parse_lines, get_priority, get_common_chars};

    #[test]
    fn test_parse_line() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let expected_first = "vJrwpWtwJgWr";
        let expected_second = "hcsFMMfFFhFp";
        let (first, second) = parse_line(input);
        assert_eq!(first, expected_first);
        assert_eq!(second, expected_second);
    }

    #[test]
    fn test_get_priority() {
        let inputs = vec!['a', 'b', 'c', 'z', 'A', 'B', 'C', 'Y', 'Z'];
        let expected_outputs = vec![1,2,3,26,27,28,29,51,52];
        let outputs: Vec<u64> = inputs.into_iter().map(|c| {
            get_priority(c)
        }).collect();
        assert_eq!(expected_outputs, outputs);
    }

    #[test]
    fn test_get_common_chars() {
        let first = "jqHRNqRjqzjGDLGL";
        let second = "rsFMfFZSrLrFZsSL";
        let expected_output = 'L';
        assert_eq!(*get_common_chars(first, second).first().unwrap(), expected_output);
    }
}
