
use crate::common::read_file;

#[derive(Debug, PartialEq, Clone)]
enum Action {
    Rock,
    Paper,
    Scissors,
}

impl Action {
    pub fn to_score(&self) -> u64 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("Unexpected value: ({})", s),
        }
    }

    fn move_that_beats(&self) -> Action {
        match self {
            Action::Paper => Action::Scissors,
            Action::Rock => Action::Paper,
            Action::Scissors => Action::Rock
        }
    }

    fn move_that_draws(&self) -> Action {
        self.clone()
    }

    fn move_that_losses(&self) -> Action {
        self.move_that_beats().move_that_beats()
    }

    pub fn from_input(op_action: &Action, result: &Outcome) -> Action {
        match result {
            Outcome::Draw => op_action.move_that_draws(),
            Outcome::Win => op_action.move_that_beats(),
            Outcome::Loss => op_action.move_that_losses()
        }
    }
}

enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Outcome {
    pub fn to_score(&self) -> u64 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Loss => 0,
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "X" => Self::Loss,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("Unexpected input: {}", s)
        }
    }

    pub fn from_round(round: &Round) -> Self {
        Self::from_actions(&round.my_action, &round.op_action)
    }

    pub fn from_actions(my_action: &Action, op_action: &Action) -> Self {
        match my_action {
            Action::Rock => {
                match op_action {
                    Action::Rock => Outcome::Draw,
                    Action::Paper => Outcome::Loss,
                    Action::Scissors => Outcome::Win,
                }
            },
            Action::Paper => {
                match op_action {
                    Action::Paper => Outcome::Draw,
                    Action::Rock => Outcome::Win,
                    Action::Scissors => Outcome::Loss
                }
            },
            Action::Scissors => {
                match op_action {
                    Action::Scissors => Outcome::Draw,
                    Action::Paper => Outcome::Win,
                    Action::Rock => Outcome::Loss
                }
            }
        }
    }
}

#[derive(Debug)]
struct Round {
    my_action: Action,
    op_action: Action,
}

impl PartialEq for Round {
    fn eq(&self, other: &Self) -> bool {
        self.my_action == other.my_action && self.op_action == other.op_action
    }
}

impl Round {
    pub fn to_score(&self) -> u64 {
        let outcome = Outcome::from_round(self);
        self.my_action.to_score() + outcome.to_score()
    }
}

pub fn run1() {
    let input = read_file("puzzle_2.input");
    let rounds = parse_input(&input);
    let score: u64 = rounds.into_iter().map(|round| { round.to_score()}).sum();

    println!("The score from this input is: {}", score);
}

pub fn run2() {
    let input = read_file("puzzle_2.input");
    let rounds = parse_input_modified(&input);
    let score: u64 = rounds.into_iter().map(|round| { round.to_score()}).sum();

    println!("The score from this input using the modified strategy is: {}", score);
}

fn parse_input(s: &str) -> Vec<Round> {
    s.split('\n')
     .map(|s| {
        s.trim()
     })
     .filter(|&s| {
        s.len() == 3
     })
     .map(|subs| {
        let mut round_string_it = subs.split(' ');
        let op_action = Action::from_str(round_string_it.next().unwrap());
        let my_action = Action::from_str(round_string_it.next().unwrap());
        Round {
            my_action,
            op_action
        }
    } ).collect()
}

fn parse_input_modified(s: &str) -> Vec<Round> {
    s.split('\n')
    .map(|s| {
       s.trim()
    })
    .filter(|&s| {
       s.len() == 3
    })
    .map(|subs| {
       let mut round_string_it = subs.split(' ');
       let op_action = Action::from_str(round_string_it.next().unwrap());
       let result = Outcome::from_str(round_string_it.next().unwrap());
       let my_action = Action::from_input(&op_action, &result);
       Round {
           my_action,
           op_action
       }
   } ).collect()
}
mod test {
    use crate::q2::{Round, Action};

    use super::parse_input;

    #[test]
    fn test_input_parser() {
        let input =
"A X
B Y
C Z

";
        let output = vec![
            Round { my_action: Action::Rock, op_action: Action::Rock },
            Round { my_action: Action::Paper, op_action: Action::Paper },
            Round { my_action: Action::Scissors, op_action: Action::Scissors },
        ];
        assert_eq!(parse_input(input), output);
    }
}
