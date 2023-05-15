use std::{io::BufRead, collections::{HashSet, HashMap}, rc::Rc, ops::DerefMut};

use crate::common::{read_file, parse_lines};

enum Direction {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32)
}

impl<T> From<T> for Direction where T : Into<String> {
    fn from(other: T) -> Direction {
        let other_string: String = other.into();
        let other_split: Vec<&str> = other_string.split(' ').collect();
        let num_steps = other_split[1].parse().unwrap();
        match other_split[0] {
            "R" => Direction::Right(num_steps),
            "L" => Direction::Left(num_steps),
            "U" => Direction::Up(num_steps),
            "D" => Direction::Down(num_steps),
            _ => panic!("Unknown Direction")
        }
    }
}

#[derive(Hash, Eq, Clone, Copy)]
struct Position {
    pub x: i32,
    pub y: i32,
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Default for Position {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0
        }
    }
}

struct Grid {
    tail_positions: HashSet<Position>,
    first_knot: Box<Knot>
}

#[derive(Clone)]
struct Knot {
    pos: Position,
    tail: Option<Box<Knot>>
}

impl<'a> Default for Knot {
    fn default() -> Self {
        Self {
            pos: Position::default(),
            tail: None
        }
    }
}

impl Knot {
    fn move_tail(&mut self) -> Option<Position> {
        if let Some(tail) = &mut self.tail {
            let mut moved = false;
            if self.pos.x - tail.pos.x > 1 {
                tail.pos.x += 1;
                tail.pos.y += if self.pos.y > tail.pos.y {
                    1
                } else if self.pos.y < tail.pos.y {
                    -1
                } else {
                    0
                };
                moved = true;
            }
            else if self.pos.x - tail.pos.x < -1 {
                tail.pos.x -= 1;
                tail.pos.y += if self.pos.y > tail.pos.y {
                    1
                } else if self.pos.y < tail.pos.y {
                    -1
                } else {
                    0
                };
                moved = true;
            }
            else if self.pos.y - tail.pos.y > 1 {
                tail.pos.y += 1;
                tail.pos.x += if self.pos.x > tail.pos.x {
                    1
                } else if self.pos.x < tail.pos.x {
                    -1
                } else {
                    0
                };
                moved = true;
            }
            else if self.pos.y - tail.pos.y < -1 {
                tail.pos.y -= 1;
                tail.pos.x += if self.pos.x > tail.pos.x {
                    1
                } else if self.pos.x < tail.pos.x {
                    -1
                } else {
                    0
                };
                moved = true;
            }
            if moved {
                tail.move_tail()
            } else {
                None
            }
        } else {
            Some(self.pos)
        }
    }

    pub fn apply_up(&mut self) -> Option<Position> {
        self.pos.y += 1;
        if self.tail.is_some() {
            return self.move_tail();
        }
        return Some(self.pos);
    }

    pub fn apply_down(&mut self) -> Option<Position> {
        self.pos.y -= 1;
        if self.tail.is_some() {
            return self.move_tail();
        }
        return Some(self.pos);
    }
    pub fn apply_left(&mut self) -> Option<Position> {
        self.pos.x -= 1;
        if self.tail.is_some() {
            return self.move_tail();
        }
        return Some(self.pos);
    }

    pub fn apply_right(&mut self) -> Option<Position> {
        self.pos.x += 1;
        if self.tail.is_some() {
            return self.move_tail();
        }
        return Some(self.pos);
    }

    pub fn add_children(&mut self, n: usize) {
        if n == 0 { return; }
        let mut knot = Knot::default();
        knot.add_children(n - 1);
        self.tail = Some(Box::new(knot));
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::with_n_knots(2)
    }
}

impl Grid {
    pub fn print_grid(&self) {
        let mut positions = Vec::new();
        let mut positions_map = HashMap::new();

        let mut knot: &Knot = &self.first_knot;
        let mut counter = 0;
        let mut looping = true;
        let mut min_x = 0;
        let mut min_y = 0;
        let mut max_x = 0;
        let mut max_y = 0;
        while looping {
            max_x = max_x.max(knot.pos.x);
            min_x = min_x.min(knot.pos.x);
            max_y = max_y.max(knot.pos.y);
            min_y = min_y.min(knot.pos.y);

            positions.push(knot.pos);
            positions_map.insert(knot.pos, counter);
            counter += 1;
            knot = &knot.tail.as_ref().unwrap_or_else(|| {
                looping = false;
                &self.first_knot
            });
        }

        for y in (min_y..max_y+1).rev() {
            for x in min_x..max_x+1 {
                if let Some(&number) = positions_map.get(&Position{ x: x, y:  y }) {
                    if number == 0 {
                        print!("H");
                    } else {
                        print!("{}", number);
                    }
                }
                else if x == 0 && y == 0 {
                    print!("s");
                }
                else {
                    print!(".");
                }
            }
            println!();
        }
        println!()
    }

    pub fn with_n_knots(n: usize) -> Grid {
        let mut tail_positions = HashSet::<Position>::default();
        tail_positions.insert(Position::default());
        let mut first_knot = Knot::default();
        first_knot.add_children(n-1);
        Self {
            first_knot: Box::new(first_knot),
            tail_positions
        }
    }

    fn head(&mut self) -> &mut Knot {
        &mut self.first_knot
    }

    pub fn apply_direction(&mut self, dir: &Direction) {
        match *dir {
            Direction::Up(steps) => self.apply_up(steps),
            Direction::Down(steps) => self.apply_down(steps),
            Direction::Left(steps) => self.apply_left(steps),
            Direction::Right(steps) => self.apply_right(steps),
        }
    }

    fn apply_up(& mut self, steps: i32) {
        if steps == 0 { return; }
        if let Some(pos) = self.head().apply_up() {
            self.tail_positions.insert(pos);
        }
        // self.print_grid();
        self.apply_up(steps-1)
    }

    fn apply_down(& mut self, steps: i32) {
        if steps == 0 { return; }
        if let Some(pos) = self.head().apply_down() {
            self.tail_positions.insert(pos);
        }
        // self.print_grid();
        self.apply_down(steps-1)

    }
    fn apply_left(& mut self, steps: i32) {
        if steps == 0 { return; }
        if let Some(pos) = self.head().apply_left() {
            self.tail_positions.insert(pos);
        }
        // self.print_grid();
        self.apply_left(steps-1)
    }

    fn apply_right(& mut self, steps: i32) {
        if steps == 0 { return; }
        if let Some(pos) = self.head().apply_right() {
            self.tail_positions.insert(pos);
        }
        // self.print_grid();
        self.apply_right(steps-1)
    }

    pub fn get_num_positions(&self) -> usize {
        self.tail_positions.len()
    }
}

pub fn solve_q9() {
    let input = read_file("./puzzle_9.input");
    let directions = parse_lines(&input).into_iter()
        .map(|line| Direction::from(line))
        .collect::<Vec<Direction>>();

    let mut grid = Grid::default();
    for direction in directions.iter() {
        grid.apply_direction(&direction);
    }
    println!("Solution 1: {}", grid.get_num_positions());

    let mut grid = Grid::with_n_knots(10);
    for direction in directions {
        grid.apply_direction(&direction);
    }
    println!("Solution 2: {}", grid.get_num_positions());

}

mod test {
    use crate::common::parse_lines;

    use super::{Direction, Grid};

    #[test]
    fn test_example_problem() {
        let directions = parse_lines("R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2")
    .into_iter()
    .map(|line| Direction::from(line))
    .collect::<Vec<Direction>>();

        let mut grid = Grid::default();
        for direction in directions.iter() {
            grid.apply_direction(&direction);
        }
        assert_eq!(13, grid.get_num_positions());

        let mut grid = Grid::with_n_knots(10);
        for direction in directions.iter() {
            grid.apply_direction(&direction);
            println!();
            grid.print_grid();
        }
        assert_eq!(1, grid.get_num_positions());

    }

    #[test]
    fn test_example_problem_2() {
        let directions = parse_lines("R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20")
    .into_iter()
    .map(|line| Direction::from(line))
    .collect::<Vec<Direction>>();
        let mut grid = Grid::with_n_knots(10);
        for direction in directions.iter() {
            grid.apply_direction(&direction);
            println!("=====");
            grid.print_grid();
        }
        assert_eq!(36, grid.get_num_positions());

    }
}
