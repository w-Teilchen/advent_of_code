use std::collections::HashSet;

const INPUT: &str = include_str!("../input");
const EXAMPLE: &str = include_str!("../example");

fn main() {
    assert_eq!(part1(EXAMPLE), 21);
    println!("Part 1: {}", part1(INPUT));
    assert_eq!(part2(EXAMPLE), 40);
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    let grid = convert_to_grid(input);
    let start = find_start(&grid);

    let mut columns_with_beams: HashSet<usize> = HashSet::new();
    columns_with_beams.insert(start.0);

    let mut number_of_splits = 0;
    for (y, row) in grid.iter().enumerate() {
        let mut updated_columns_with_beams: HashSet<usize> = HashSet::new();
        for &x in columns_with_beams.iter() {
            if row[x] == b'^' {
                number_of_splits += 1;
                if x > 0 {
                    updated_columns_with_beams.insert(x - 1);
                }
                if x < row.len() - 1 {
                    updated_columns_with_beams.insert(x + 1);
                }
            } else {
                updated_columns_with_beams.insert(x);
            }
        }
        columns_with_beams = updated_columns_with_beams;
    }
    number_of_splits
}

fn part2(input: &str) -> usize {
    let grid = Arc::new(convert_to_grid(input));
    let start = find_start(&grid);

    follow_beam(grid, start)
}

use cached::proc_macro::cached;
use std::sync::Arc;
#[cached]
fn follow_beam(grid: Arc<Vec<Vec<u8>>>, position: (usize, usize)) -> usize {
    let (x, y) = position;
    if y >= grid.len() - 1 {
        return 1;
    }
    if grid[y + 1][x] == b'^' {
        if x == 0 {
            return follow_beam(grid.clone(), (x + 1, y + 1));
        } else if x == grid[0].len() - 1 {
            return follow_beam(grid.clone(), (x - 1, y + 1));
        } else {
            return follow_beam(grid.clone(), (x - 1, y + 1))
                + follow_beam(grid.clone(), (x + 1, y + 1));
        }
    } else {
        return follow_beam(grid, (x, y + 1));
    }
}

fn convert_to_grid(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.bytes().collect()).collect()
}

fn find_start(grid: &Vec<Vec<u8>>) -> (usize, usize) {
    // start is in first row with value S
    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == b'S' {
                return (x, y);
            }
        }
    }
    panic!("Start position not found");
}
