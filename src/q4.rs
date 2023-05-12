use crate::common::{read_file, parse_lines};

#[derive(Copy, Clone, PartialEq, Debug)]
struct Region {
    lower: u32,
    upper: u32
}

impl Region {
    pub fn contains(&self, other: &Self) -> bool {
        self.lower <= other.lower && self.upper >= other.upper
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        self.lower <= other.upper && self.lower >= other.lower
    }
}

pub fn run() {
    let input = "./puzzle_4.input";
    let contents = read_file(input);
    let lines = parse_lines(&contents);
    let mut sum = 0;
    for line in lines {
        let (r1, r2) = parse_line(line);
        if contains(&r1, &r2) {
            sum += 1;
        }
    }
    println!("{} pairs have one range fully containing the other", sum);

    let lines = parse_lines(&contents);
    let mut sum = 0;
    for line in lines {
        let (r1, r2) = parse_line(line);
        if overlaps(&r1, &r2) {
            sum += 1;
        }
    }
    println!("{} pairs overlap", sum);
}

fn contains(r1: &Region, r2: &Region) -> bool {
    r1.contains(r2) || r2.contains(r1)
}

fn overlaps(r1: &Region, r2: &Region) -> bool {
    r1.overlaps(r2) || r2.overlaps(r1)
}

fn parse_line(line: &str) -> (Region, Region) {
    let regions: Vec<Region> = line.split(',').map(|range| {
        let range: Vec<&str> = range.split('-').collect();
        Region {
            lower: range[0].parse().unwrap(),
            upper: range[1].parse().unwrap()
        }
    }).collect();

    (regions[0], regions[1])
}

mod test {
    use super::{Region, contains, parse_line};

    #[test]
    fn test_contains() {
        let r1 = Region {
            lower: 4,
            upper: 6
        };
        let r2 = Region {
            lower: 6,
            upper: 6,
        };
        let r3 = Region {
            lower: 5,
            upper: 8
        };

        assert!(contains(&r1, &r2));
        assert!(!contains(&r1, &r3));
        assert!(contains(&r3, &r2));

    }

    #[test]
    fn test_parse_line() {
        let input = "3-5,7-10";
        let expected_output = (
            Region {
                lower: 3, upper: 5
            },
            Region {
                lower: 7,
                upper: 10
            }
        );
        assert_eq!(parse_line(input), expected_output);
    }
}
