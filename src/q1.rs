use crate::common;

pub fn run(n: usize) {
    let input_path = "./puzzle_1.input";
    let numbers = read_file(input_path);

    let mut sum = 0;
    let mut sums = Vec::with_capacity(numbers.len());
    for number in numbers {
        if number == 0 {
            sums.push(sum);
            sum = 0;
        } else {
            sum += number;
        }
    }
    sums.sort_unstable();
    for diff in 0..n {
        let index = (sums.len() - 1) - diff;
        sum += sums.get(index).unwrap();
    }
    println!("The most Calories Raised by {} elf(s) is: {}", n, sum);
}

fn read_file(input: &str) -> Vec<u64>{
    let s = common::read_file(input);

    let numbers: Vec<u64> = parse_input(&s);

    numbers
}

fn parse_input(s: &str) -> Vec<u64> {
    s.split('\n').map(|subs| {
        match subs.trim_matches(' ').parse::<u64>() { Ok(i) => i, _ => 0}
    } ).collect()
}

mod test {
    use super::parse_input;

    #[test]
    fn test_parse_input() {
        let input_1 = "1
                                2
                                3
                                4";
        let expected_output_1 = vec![1,2,3,4];

        assert_eq!(parse_input(input_1), expected_output_1);

        let input_2 = "1
        2
        3

        4

        5

        6";

        let expected_output_2 = vec![1,2,3,0,4,0,5,0,6];

        assert_eq!(parse_input(input_2), expected_output_2);
    }
}
