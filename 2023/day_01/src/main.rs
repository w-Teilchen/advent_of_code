use std::{fs};

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
        total += extract_digits(line);
    }
    println!("{}", total);
}

fn extract_digits(line: &str) -> u32 {
    let first_digit = line
        .chars()
        .find(|c| c.is_ascii_digit())
        .and_then(|c| c.to_digit(10))
        .unwrap_or(0);

    let last_digit = line
        .chars()
        .rev()
        .find(|c| c.is_ascii_digit())
        .and_then(|c| c.to_digit(10))
        .unwrap_or(0);
    first_digit * 10 + last_digit
}

fn second_puzzle(input: &str) {
    let contents = fs::read_to_string(input).expect("Something went wrong reading the file");

    let mut total = 0;
    for line in contents.lines() {
        let converted_line = convert(line.to_string());
        total += extract_digits(&converted_line);
    }

    println!("{}", total);
}

fn convert(mut string: String) -> String {
    let mut converted_string = String::new();
    let mappings = vec![
        ("one", "1"),
        ("1", "1"),
        ("two", "2"),
        ("2", "2"),
        ("three", "3"),
        ("3", "3"),
        ("four", "4"),
        ("4", "4"),
        ("five", "5"),
        ("5", "5"),
        ("six", "6"),
        ("6", "6"),
        ("seven", "7"),
        ("7", "7"),
        ("eight", "8"),
        ("8", "8"),
        ("nine", "9"),
        ("9", "9"),
        ("zero", "0"),
        ("0", "0"),
    ];

    'beginning: while !string.is_empty() {
        for (word, digit) in &mappings {
            if string.starts_with(word) {
                let n = word.len();
                string.drain(..n);
                converted_string.push_str(digit);
                break 'beginning;
            }
        }

        let ch = string.chars().next().unwrap();
        converted_string.push(ch);
        let ch_len = ch.len_utf8();
        string.drain(..ch_len);
    }

    converted_string.push_str(&string);
    while !converted_string.is_empty() {
        for (word, digit) in &mappings {
            if converted_string.ends_with(word) {
                return format!("{}{}", converted_string, digit);
            }
        }

        converted_string.pop();
    }

    String::new()
}
