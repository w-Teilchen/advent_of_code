use std::cmp;

const INPUT: &str = include_str!("../input");
const EXAMPLE: &str = include_str!("../example");

fn main() {
    assert_eq!(part1(EXAMPLE), 4361);
    println!("Part 1: {}", part1(INPUT));

    assert_eq!(part2(EXAMPLE), 467835);
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> u32 {
    let mut sum = 0;

    let mut number_of_digits = 0;
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    for (y, row) in matrix.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            // part of an already processed number
            if number_of_digits > 0 {
                number_of_digits -= 1;
                continue;
            }
            // start of a new number (bounds-checked)
            while x + number_of_digits < row.len() && matrix[y][x + number_of_digits].is_digit(10) {
                number_of_digits += 1;
            }
            if number_of_digits == 0 {
                continue;
            }
            if is_adjacent_to_symbol(&matrix, x, y, number_of_digits) {
            let number = &row[x..x + number_of_digits];
                sum += number.iter().collect::<String>().parse::<u32>().unwrap();
            }
        }
    }
    sum
}

fn is_adjacent_to_symbol(matrix: &Vec<Vec<char>>, x: usize, y: usize, length: usize) -> bool {
    let height = matrix.len();
    let width = matrix[0].len();

    let y_start = y.saturating_sub(1);
    let y_end = (y + 2).min(height);
    let x_start = x.saturating_sub(1);
    let x_end = (x + length + 1).min(width);

    for yy in y_start..y_end {
        for xx in x_start..x_end {
            let c = matrix[yy][xx];
            if !c.is_digit(10) && c != '.' {
                return true;
            }
        }
    }
    false
}

fn part2(input: &str) -> u32 {
    let mut sum = 0;
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    for (y, row) in matrix.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == '*' {
                let adjacent_numbers = find_adjacent_numbers(&matrix, x, y);
                if adjacent_numbers.len() == 2 {
                    sum += adjacent_numbers[0] * adjacent_numbers[1];
                }
            }
        }
    }
    sum
}

fn find_adjacent_numbers(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<u32> {
    let height = matrix.len();
    let width = matrix[0].len();
    let mut adjacent_numbers = Vec::new();

    let y_start = y.saturating_sub(1);
    let y_end = (y + 2).min(height);
    let x_start = x.saturating_sub(1);
    let x_end = (x + 2).min(width);

    for yy in y_start..y_end {
        let mut remaining_length_of_previous_number = 0;
        for xx in x_start..x_end {
            if remaining_length_of_previous_number > 0 {
                remaining_length_of_previous_number -= 1;
                continue;
            }
            let mut x = xx;
            let mut has_found_start_of_number = false;
            let mut number = 0;
            while x < width && matrix[yy][x].is_digit(10) {
                if x == 0 {
                    has_found_start_of_number = true;
                    number = matrix[yy][x].to_digit(10).unwrap();
                    x += 1;
                    continue;
                }
                if !has_found_start_of_number {
                    if matrix[yy][x.saturating_sub(1)].is_digit(10) {
                        x -= 1;
                        continue;
                    } else {
                        has_found_start_of_number = true;
                        number = matrix[yy][x].to_digit(10).unwrap();
                        x += 1;
                        continue;
                    }
                }
                number *= 10;
                number += matrix[yy][x].to_digit(10).unwrap();
                x += 1;
            }
            if number > 0 {
                remaining_length_of_previous_number = x - xx;
                println!("Found adjacent number: {}", number);
                adjacent_numbers.push(number);
            }
        }
    }
    adjacent_numbers
}