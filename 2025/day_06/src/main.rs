const INPUT: &str = include_str!("../input");
const EXAMPLE: &str = include_str!("../example");

fn main() {
    assert_eq!(part1(EXAMPLE), 4277556);
    println!("Part 1: {}", part1(INPUT));
    assert_eq!(part2(EXAMPLE), 3263827);
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> u64 {
    let lines = input.lines().collect::<Vec<&str>>();
    let (first_lines, last_line) = lines.split_at(lines.len() - 1);
    let last_line = last_line[0].split_whitespace().collect::<Vec<&str>>();
    let grid: Vec<Vec<u64>> = first_lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect();
    let mut sum = 0;
    for i in 0..last_line.len() {
        let operator = last_line[i];
        let mut result = grid[0][i];
        for number in grid.iter().skip(1) {
            if operator == "+" {
                result += number[i];
            } else if operator == "*" {
                result *= number[i];
            }
        }
        sum += result;
    }
    sum
}

fn part2(input: &str) -> u64 {
    let mut sum = 0;
    let lines = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let mut operator = ' ';
    let mut digits: Vec<char> = vec![' '; lines.len() - 1];
    let mut numbers: Vec<u64> = Vec::new();
    for i in 0..lines[0].len() {
        for j in 0..lines.len() - 1 {
            digits[j] = lines[j][i];
        }
        let relevant_digits = digits.iter().filter(|&&c| c != ' ').collect::<Vec<&char>>();
        if lines[lines.len() - 1][i] != ' ' {
            operator = lines[lines.len() - 1][i];
        }
        let mut result = 0;
        for k in 0..relevant_digits.len() {
            let digit = relevant_digits[k].to_digit(10).unwrap();
            result += digit as u64 * 10u64.pow((relevant_digits.len() - k - 1) as u32);
        }
        if result != 0 {
            numbers.push(result);
        }
        if relevant_digits.is_empty() || i == lines[0].len() - 1 {
            result = numbers[0];
            for number in numbers.iter().skip(1) {
                if operator == '+' {
                    result += number;
                } else if operator == '*' {
                    result *= number;
                }
            }
            numbers.clear();
            sum += result;
        }
    }
    sum
}
