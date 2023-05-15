use std::fmt::Debug;

use crate::common::{parse_lines, read_file};

trait PrintOut {
    fn print(&self);
}

impl<T> PrintOut for  Vec<Vec<T>>
where T : Debug {
    fn print(&self) {
        for row in self {
            for item in row {
                print!("{:?} ", item);
            }
            println!();
        }
    }
}

fn parse_puzzle_input_to_tree_grid(input: String) -> Vec<Vec<u32>> {
    let lines = parse_lines(&input);
    let mut tree_grid: Vec<Vec<u32>> = Vec::new();
    for line in lines {
        let line: &[u8] = line.as_bytes();
        let mut tree_row = Vec::new();
        for entry in line {
            tree_row.push((entry - b'0') as u32);
        }
        tree_grid.push(tree_row);
    }
    tree_grid
}

fn get_visible_trees(grid: &Vec<Vec<u32>>) -> u32 {
    let mut grid_vis: Vec<Vec<bool>> = Vec::new();
    for i in 0..grid.len() {
        let mut grid_row = Vec::new();
        for j in 0..grid[0].len() {
            grid_row.push(i == 0 || i == (grid.len()-1) || j == 0 || j == (grid[0].len()-1));
        }
        grid_vis.push(grid_row);
    }

    /* Rows right to left */
    for (row_index, row) in grid.iter().enumerate() {
        let mut max_height = 0;
        for (column_index, &tree_height) in row.iter().enumerate() {
            if tree_height > max_height {
                max_height = tree_height;
                grid_vis[row_index][column_index] = true;
            }
        }
    }

    /* Rows left to right */
    for (row_index, row) in grid.iter().enumerate() {
        let mut max_height = 0;
        for column_index in (0..row.len()).rev() {
            let tree_height = row[column_index];
            if tree_height > max_height {
                max_height = tree_height;
                grid_vis[row_index][column_index] = true;
            }
        }
    }

    for column_index in 0..grid[0].len() {
        let mut max_height = 0;
        for row_index in 0..grid.len() {
            if grid[row_index][column_index] > max_height {
                max_height = grid[row_index][column_index];
                grid_vis[row_index][column_index] = true;
            }
        }
    }

    for column_index in 0..grid[0].len() {
        let mut max_height = 0;
        for row_index in (0..grid.len()).rev() {
            if grid[row_index][column_index] > max_height {
                max_height = grid[row_index][column_index];
                grid_vis[row_index][column_index] = true;
            }
        }
    }

    grid_vis
        .into_iter()
        .map::<u32, _>(|row| row.into_iter().map(|b| (b as u32)).sum())
        .sum()
}

fn get_scenic_score(grid: &Vec<Vec<u32>>, tree_row: usize, tree_col: usize) -> u32 {
    let tree_height = grid[tree_row][tree_col];
    let mut score: u32 = 1;
    let mut counter = 0;

    for i in (0..tree_row).rev() {
        let other_height = grid[i][tree_col];
        counter += 1;
        if other_height >= tree_height {
            break; // View Blocked
        }
    }
    score *= counter;
    counter = 0;

    for i in tree_row+1..grid.len() {
        let other_height = grid[i][tree_col];
        counter += 1;
        if other_height >= tree_height {
            break; // View Blocked
        }
    }

    score *= counter;

    counter = 0;

    for i in tree_col+1..grid.len() {
        let other_height = grid[tree_row][i];
        counter += 1;
        if other_height >= tree_height {
            break; // View Blocked
        }
    }

    score *= counter;


    counter = 0;

    for i in (0..tree_col).rev() {
        let other_height = grid[tree_row][i];
        counter += 1;
        if other_height >= tree_height {
            break; // View Blocked
        }
    }

    score *= counter;

    score
}

fn get_most_scenic_tree(grid: &Vec<Vec<u32>>) -> u32 {
    let mut best_score = 0;
    for row_index in 1..(grid.len() - 1) {
        for col_index in 1..(grid[0].len() - 1) {
            best_score = u32::max(best_score, get_scenic_score(grid, row_index, col_index));
        }
    }
    best_score
}

pub fn solve_q8() {
    let input_file_name = "puzzle_8.input";
    let input_file = read_file(input_file_name);
    let grid = parse_puzzle_input_to_tree_grid(input_file);
    let solution1 = get_visible_trees(&grid);
    let solution2 = get_most_scenic_tree(&grid);

    println!("Solution 1: {}", solution1);
    println!("Solution 2: {}", solution2);
}

mod test {
    use crate::q8::{parse_puzzle_input_to_tree_grid, get_most_scenic_tree, get_scenic_score};

    use super::get_visible_trees;

    #[test]
    fn test_parse_puzzle_input_to_tree_grid() {
        let input =
String::from("30373
25512
65332
33549
35390");
        let expected_output = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0]
        ];
        let output = parse_puzzle_input_to_tree_grid(input);
        assert_eq!(expected_output, output);
    }

    #[test]
    fn test_get_visible_trees() {
        let input: Vec<Vec<u32>> = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0]
        ];

        let expected_out = 21;

        let output = get_visible_trees(&input);
        assert_eq!(expected_out, output);
    }

    #[test]
    fn test_get_most_scenic_tree() {
        let input: Vec<Vec<u32>> = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0]
        ];

        let expected_out = 8;

        let output = get_most_scenic_tree(&input);
        assert_eq!(expected_out, output);
    }

    #[test]
    fn test_get_scenic_score() {
        let input: Vec<Vec<u32>> = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0]
        ];

        assert_eq!(8, get_scenic_score(&input, 3, 2));
        assert_eq!(4, get_scenic_score(&input, 1, 2));
    }
}
