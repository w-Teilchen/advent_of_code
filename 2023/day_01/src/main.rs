use std::{fs};

static CHARACTER_TO_NUMBER: &[( &str, u32)] = &[
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("0", 0),
    ];

static WORD_TO_NUMBER: &[( &str, u32)] = &[
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("zero", 0),
    ];

fn main() {
    first_puzzle("example");
    first_puzzle("input");
    second_puzzle("example2");
    second_puzzle("input");
}

fn first_puzzle(input: &str) {
    let contents = fs::read_to_string(input).expect("Something went wrong reading the file");

    let mut total = 0;
    for line in contents.lines() {
        total += extract_values(line, &CHARACTER_TO_NUMBER);
    }
    println!("{}", total);
}

fn second_puzzle(input: &str) {
    let contents = fs::read_to_string(input).expect("Something went wrong reading the file");

    let mut total = 0;
    let combined_mappings: Vec<(&str, u32)> = WORD_TO_NUMBER.iter().chain(CHARACTER_TO_NUMBER.iter()).cloned().collect();
    for line in contents.lines() {

        total += extract_values(line, &combined_mappings);
    }
    println!("{}", total);
}

fn extract_values(line: &str, mappings: &[( &str, u32)]) -> u32 {
    return 10 * extract_first_digit(line, mappings) + extract_last_digit(line, mappings);
}

fn extract_first_digit(line: &str, mappings: &[( &str, u32)]) -> u32 {
    let mut line = line.to_string();
    while !line.is_empty() {
        for (word, digit) in mappings {
            if line.starts_with(word) {
                return *digit;
            }
        }
        line.remove(0);
    }
    0
}

fn extract_last_digit(line: &str, mappings: &[( &str, u32)]) -> u32 {
    let mut line = line.to_string();
    while !line.is_empty() {
        for (word, digit) in mappings {
            if line.ends_with(word) {
                return *digit;
            }
        }
        line.pop();
    }
    0
}

