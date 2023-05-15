use std::collections::VecDeque;

use crate::common::{read_file, parse_lines};

enum Operation {
    Add(i64),
    Noop
}

struct Instruction {
    num_cycles: u32,
    operation: Operation,
}

impl From<&str> for Instruction {
    fn from(line: &str) -> Self {
        let bytes = line.as_bytes();
        if bytes == b"noop" {
            Instruction {
                num_cycles: 1,
                operation: Operation::Noop
            }
        } else {
            let chunks: Vec<&[u8]> = bytes.split(|&c| c == b' ').collect();
            let val = String::from_utf8(chunks[1].to_vec()).unwrap().parse::<i64>().unwrap();
            Instruction {
                num_cycles: 2,
                operation: Operation::Add(val)
            }
        }
    }
}

struct CpuEmulator {
    instructions: VecDeque<Instruction>,
    current_cycle: u32,
    current_instruction: Option<Instruction>,
    register_x: i64,
    signal_strength: i64,
}

impl Default for CpuEmulator {
    fn default() -> Self {
        Self {
            instructions: VecDeque::default(),
            current_cycle: 1,
            current_instruction: None,
            register_x: 1,
            signal_strength: 0
        }
    }
}

impl CpuEmulator {
    pub fn get_current_cycle(&self) -> u32 { self.current_cycle }

    pub fn get_signal_strength(&self) -> i64 {
        self.signal_strength
    }
    pub fn insert_instructions(&mut self, instructions: Vec<Instruction>) {
        self.instructions.append(&mut instructions.into());
        if self.current_instruction.is_none() {
            self.current_instruction = self.instructions.pop_front();
        }
    }

    pub fn has_instructions(&self) -> bool {
        self.instructions.len() > 0
    }

    pub fn execute_clock_cycle(&mut self) {
        let mut next_instruction = false;
        if let Some(instruction) = self.current_instruction.as_mut() {
            instruction.num_cycles -= 1;
            if instruction.num_cycles == 0 {
                match instruction.operation {
                    Operation::Add(amount) => {
                        self.register_x += amount;
                    },
                    Operation::Noop => {}
                }
                next_instruction = true;
            }
        }
        if next_instruction {
            self.current_instruction = self.instructions.pop_front();
        }

        self.current_cycle += 1;
        if self.current_cycle == 20 || (self.current_cycle > 20 && ((self.current_cycle - 20) % 40) == 0) {
            self.signal_strength += self.register_x * (self.current_cycle as i64);
        }
    }

    pub fn print_crt_data(&self) {
        println!("register x: {}, current_cycle: {}", self.register_x, self.current_cycle % 40);
    }

    pub fn draw_crt_pixel(&self) {
        if self.register_x == (((self.current_cycle - 1) % 40) as i64)
        || self.register_x + 1 == (((self.current_cycle - 1) % 40) as i64)
        || self.register_x - 1 == (((self.current_cycle - 1) % 40) as i64) {
            print!("#");
        } else {
            print!(".");
        }
        if (self.current_cycle) % 40 == 0 {
            println!();
        }
    }
}

pub fn solve_q10() {
    let input_data = read_file("./puzzle_10.input");
    let input: Vec<Instruction> = parse_lines(&input_data)
        .into_iter()
        .map(|line| Instruction::from(line))
        .collect();

    let mut emulator = CpuEmulator::default();
    emulator.insert_instructions(input);
    while emulator.has_instructions() && emulator.get_current_cycle() < 220 {
        emulator.execute_clock_cycle();
    }
    println!("");
    println!("Solution1: {}", emulator.get_signal_strength());
    let input: Vec<Instruction> = parse_lines(&input_data)
        .into_iter()
        .map(|line| Instruction::from(line))
        .collect();

    let mut emulator = CpuEmulator::default();
    emulator.insert_instructions(input);
    while emulator.has_instructions() {
        emulator.draw_crt_pixel();
        emulator.execute_clock_cycle();
    }
    println!();
}

mod test {
    use crate::common::parse_lines;

    use super::{Instruction, CpuEmulator};

    #[test]
    fn test_example() {
let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
        let input: Vec<Instruction> = parse_lines(input)
            .into_iter()
            .map(|line| Instruction::from(line))
            .collect();

        let mut emulator = CpuEmulator::default();
        emulator.insert_instructions(input);
        while emulator.has_instructions() && emulator.get_current_cycle() < 220 {
            emulator.draw_crt_pixel();
            emulator.execute_clock_cycle();
            // emulator.print_crt_data();
        }
        assert_eq!(emulator.get_signal_strength(), 13140);
        while emulator.has_instructions() {
            emulator.draw_crt_pixel();
            emulator.execute_clock_cycle();
            // emulator.print_crt_data();

        }
        println!();
    }
}
