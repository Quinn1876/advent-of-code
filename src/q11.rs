use std::fmt::Display;

use regex::{Regex, Captures};

use crate::common::read_file;

type MonkeyId = usize;
type WorryLevel = usize;

#[derive(PartialEq, Debug)]
struct Item {
    worry_level: WorryLevel
}

impl From<WorryLevel> for Item {
    fn from(worry_level: WorryLevel) -> Self {
        Self { worry_level }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.worry_level)
    }
}

impl Item {
    pub fn inspect(&mut self, operation: &Operation, rollover: &WorryLevel) {
        match operation {
            Operation::Add(increment) => {
                self.worry_level += increment;
            },
            Operation::Multiply(factor) => {
                self.worry_level *= factor;
            },
            Operation::SquareOld => {
                self.worry_level *= self.worry_level.clone();
            }
        }
        self.worry_level %= rollover;
    }

    pub fn divisibility_test(&self, test_value: &WorryLevel) -> bool {
        &self.worry_level % test_value == 0
    }
}

#[derive(PartialEq, Debug)]
struct MonkeyTurnResult {
    item: Item,
    new_owner: MonkeyId
}

#[derive(PartialEq, Debug)]
enum Operation {
    Add(WorryLevel),
    Multiply(WorryLevel),
    SquareOld
}

#[derive(PartialEq, Debug)]
struct Monkey {
    id: MonkeyId,
    items: Vec<Item>,
    operation: Operation,
    divisibility_test: WorryLevel,
    on_true: MonkeyId,
    on_false: MonkeyId,
    number_of_inspections: usize
}

impl<'a> From<Captures<'a>> for Monkey {
    fn from(cap: Captures) -> Self {
        Monkey {
            id: cap[1].parse::<MonkeyId>().unwrap(),
            items: cap[2].to_string()
                         .split(", ")
                         .map(|num| num.parse::<WorryLevel>().unwrap().into())
                         .collect(),
            operation: if &cap[4] == "old" {
                Operation::SquareOld
            } else if &cap[3] == "+" {
                Operation::Add(cap[4].parse::<WorryLevel>().unwrap())
            } else {
                Operation::Multiply(cap[4].parse::<WorryLevel>().unwrap())

            },
            divisibility_test: cap[5].parse::<WorryLevel>().unwrap(),
            on_true: cap[6].parse::<MonkeyId>().unwrap(),
            on_false: cap[7].parse::<MonkeyId>().unwrap(),
            number_of_inspections: 0
        }
    }
}

impl Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Monkey {}:", self.id)?;
        if self.items.len() > 0 {
            write!(f, " {}", self.items[0])?;
            for index in 1..self.items.len() {
                write!(f, ", {}", self.items[index])?;
            }
        }
        Ok(())
    }
}

impl Monkey {
    pub fn take_a_turn(&mut self, rollover: &WorryLevel) -> Vec<MonkeyTurnResult> {
        let mut results = Vec::new();
        while let Some(mut item) = self.items.pop() {
            item.inspect(&self.operation, rollover);
            self.number_of_inspections += 1;
            if item.divisibility_test(&self.divisibility_test) {
                results.push(MonkeyTurnResult {
                    item: item,
                    new_owner: self.on_true
                });
            } else {
                results.push(MonkeyTurnResult {
                    item: item,
                    new_owner: self.on_false
                });
            }
        }
        results
    }
    pub fn give_item(&mut self, item: Item) {
        self.items.push(item)
    }

    pub fn get_monkey_business(&self) -> usize {
        self.number_of_inspections
    }

    pub fn get_divisibility_test(&self) -> WorryLevel {
        self.divisibility_test
    }
}

fn parse_input_to_monkeys(input: &str) -> Vec<Monkey> {
    let re = Regex::new(
        r"Monkey (\d{1}):
  Starting items: ([0-9]+(?:, [0-9]+)*)*
  Operation: new = old (\*|\+) (old|[0-9]+)
  Test: divisible by ([0-9]+)
    If true: throw to monkey (\d{1})
    If false: throw to monkey (\d{1})"
            ).unwrap();
    let captures = re.captures_iter(input);
    captures.map(|cap: Captures| {
        cap.into()
    }).collect()
}


struct MonkeyManager {
    monkeys: Vec<Monkey>,
    worry_level_rollover: WorryLevel
}

impl From<&str> for MonkeyManager {
    fn from(input: &str) -> Self {
        let monkeys = parse_input_to_monkeys(input);
        let worry_level_rollover = monkeys.iter().map(|monkey: &Monkey| monkey.get_divisibility_test()).product();
        Self {
            monkeys,
            worry_level_rollover,
        }
    }
}

impl MonkeyManager {
    pub fn play_round(&mut self) {
        for monkey_id in 0..self.monkeys.len() {
            let monkey_results;
            {
                let main_monkey = &mut self.monkeys[monkey_id];
                monkey_results = main_monkey.take_a_turn(&self.worry_level_rollover);
            }
            for monkey_result in monkey_results {
                self.monkeys[monkey_result.new_owner].give_item(monkey_result.item);
            }
        }
    }

    pub fn print_monkeys(&self) {
        for monkey in self.monkeys.iter() {
            println!("{}", monkey);
        }
    }

    pub fn print_inspections(&self) {
        for monkey in self.monkeys.iter() {
            println!("Monkey 0 inspected items {} times.", monkey.get_monkey_business());
        }
    }

    pub fn calculate_monkey_business(&self) -> usize {
        let mut inspections: Vec<usize> = self.monkeys.iter().map(|monkey| monkey.get_monkey_business()).collect();
        inspections.sort();
        inspections[inspections.len()-1]*inspections[inspections.len()-2]
    }
}

pub fn solve_q11() {
    let input = read_file("./puzzle_11.input");
    let mut mm = MonkeyManager::from(&input as &str);
    // for _ in 0..20 { // uncomment for part 1
    for _ in 0..10000 { // part 2
        mm.play_round();
    }
    println!("solution1: {}", mm.calculate_monkey_business());
}

mod test {
    use crate::q11::{parse_input_to_monkeys, WorryLevel};

    use super::{Monkey, MonkeyManager};

    #[test]
    fn test_parse_monkey_0() {
        let input = "
Monkey 0:
  Starting items: 79, 98, 77, 99
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3";
        let expected_output = Monkey {
            divisibility_test: 23,
            id: 0,
            items: vec![79.into(), 98.into(), 77.into(), 99.into()],
            on_false: 3,
            on_true: 2,
            operation: super::Operation::Multiply(19),
            number_of_inspections: 0
        };

        assert_eq!(expected_output, parse_input_to_monkeys(input)[0]);
    }

    #[test]
    fn test_monkey_business() {
        let input = "
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    let mut mm = MonkeyManager::from(input);
    for round in 0..10000 {
        mm.play_round();
        if round+1 % 1000 == 0 {
            println!("=== After round {} ===", round+1);
            mm.print_inspections();
        }
    }
    mm.print_monkeys();
    assert_eq!(2713310158, mm.calculate_monkey_business());
    }
}
