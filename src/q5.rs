use std::collections::HashMap;

use crate::common::{parse_lines, read_file};

pub fn run() {
    let input = "./puzzle_5.input";
    let contents = read_file(input);

    let (boxes, instructions) = contents.split_once("\n\n").unwrap();

    let boxes = parse_lines(boxes);
    let mut box_stacks: Vec<Vec<char>> = parse_boxes(boxes);

    let instructions: Vec<Instruction> = parse_lines(instructions).into_iter().map(|line| {
        Instruction::from_line(line)
    }).collect();

    for instruction in instructions {
        for _ in 0..instruction.num_to_move {
            let b = box_stacks.get_mut(instruction.from).unwrap().pop().unwrap();
            box_stacks.get_mut(instruction.to).unwrap().push(b);
        }
    }

    print!("The final box order is: ");
    for stack in box_stacks {
        print!("{}", stack.last().unwrap());
    }
    println!();

}

pub fn run2() {
    let input = "./puzzle_5.input";
    let contents = read_file(input);

    let (boxes, instructions) = contents.split_once("\n\n").unwrap();

    let boxes = parse_lines(boxes);
    let mut box_stacks: Vec<Vec<char>> = parse_boxes(boxes);

    let instructions: Vec<Instruction> = parse_lines(instructions).into_iter().map(|line| {
        Instruction::from_line(line)
    }).collect();

    for instruction in instructions {
        let mut stack: Vec<char> = Vec::new();
        for _ in 0..instruction.num_to_move {
            let b = box_stacks.get_mut(instruction.from).unwrap().pop().unwrap();
            stack.push(b);
        }
        while stack.last().is_some() {
            box_stacks.get_mut(instruction.to).unwrap().push(stack.pop().unwrap());
        }
    }

    print!("The final box order for the 9001 is: ");
    for stack in box_stacks {
        print!("{}", stack.last().unwrap());
    }
    println!();

}

fn parse_boxes(boxes: Vec<&str>) -> Vec<Vec<char>> {
    let mut box_stacks: Vec<Vec<char>> = Vec::new();
    for line in boxes {
        let chunks = line.as_bytes().chunks(4);
        for (stack_number, chunk) in chunks.enumerate() {
            if chunk[1].is_ascii_digit() { continue; }
            if box_stacks.len() == 0 || ((box_stacks.len() - 1) < stack_number)  {
                box_stacks.push(Vec::new());
            }
            box_stacks.get_mut(stack_number).unwrap().push(chunk[1] as char);
        }
    }
    box_stacks
        .into_iter()
        .map(|mut stack| { stack.reverse(); stack })
        .map(|mut stack| { trim_top_of_stack(&mut stack); stack})
        .collect()
}

fn trim_top_of_stack(stack: &mut Vec<char>) {
    while *stack.last().unwrap_or(&'\0') == ' ' {
        stack.pop();
    }
}

struct Instruction {
    pub num_to_move: u32,
    pub from: usize,
    pub to: usize
}

impl Instruction {
    pub fn from_line(line: &str) -> Instruction {
        let parts: Vec<&str> = line.split(' ').collect();
        Instruction {
            num_to_move: parts[1].parse().unwrap(),
            from: parts[3].parse::<usize>().unwrap() - 1,
            to: parts[5].parse::<usize>().unwrap() - 1
        }
    }
}

mod test {
    use super::parse_boxes;

    #[test]
    fn test_parse_boxes() {
        let input = "
[Z]
[N]
[C] [D]
[M] [P] [A]
1   2   3";
        let expected_output = vec![
            vec!['M', 'C',],
            vec!['P', 'D', 'N', 'Z'],
            vec!['A'],
        ];

        assert_eq!(parse_boxes(input.split('\n').collect()), expected_output);
    }
}
