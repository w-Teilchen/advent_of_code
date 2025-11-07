use std::fs;

fn main() {
    first_puzzle("example");
    first_puzzle("input");
    second_puzzle("example");
    second_puzzle("input");
}

fn first_puzzle(input: &str) {
    let contents = fs::read_to_string(input).expect("Something went wrong reading the file");

    let mut total = 0;
    for line in contents.lines() {
        if is_valid_game(line) {
            // extract the game index from Game 1:
            let game_index = line.split_once(':').unwrap().0
                .trim_start_matches("Game ")
                .parse::<u32>()
                .unwrap();
            total += game_index;
        }
    }
    println!("{}", total);
}

fn is_valid_game(line: &str) -> bool {
    // bag content: 12 red cubes, 13 green cubes, and 14 blue cubes split by ";"

    // remove Game [some number]: from line:
    let line = line.split_once(':').unwrap().1;
    let showings: Vec<&str> = line.split(';').collect();

    for showing in showings {
        let parts: Vec<&str> = showing.split(',').collect();
        for part in parts {
            // split entries like "3 blue" by whitespace:
            let words: Vec<&str> = part.trim().split_whitespace().collect();
            let (number, color) = match words.as_slice() {
                [num, col, ..] => (*num, *col),
                _ => ("0", ""),
            };

            let n = number.parse::<u32>().unwrap_or(0);
            match color {
                "red" if n > 12 => return false,
                "green" if n > 13 => return false,
                "blue" if n > 14 => return false,
                "red" | "green" | "blue" => { /* within limits */ }
                _ => panic!("Invalid color or number combination {part:?}"),
            }
        }
    }

    true
}

fn second_puzzle(input: &str) {
    let contents = fs::read_to_string(input).expect("Something went wrong reading the file");

    let mut total = 0;
    for line in contents.lines() {
        let (red, green, blue) = minimal_bag_contents(line);
        total += red * green * blue;
    }
    println!("{}", total);
}

fn minimal_bag_contents(line: &str) -> (u32, u32, u32) {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    // remove Game [some number]: from line:
    let line = line.split_once(':').unwrap().1;
    let showings: Vec<&str> = line.split(';').collect();

    for showing in showings {

        let parts: Vec<&str> = showing.split(',').collect();
        for part in parts {
            // split entries like "3 blue" by whitespace:
            let words: Vec<&str> = part.trim().split_whitespace().collect();
            let (number, color) = match words.as_slice() {
                [num, col, ..] => (*num, *col),
                _ => ("0", ""),
            };

            let n = number.parse::<u32>().unwrap_or(0);
            match color {
                "red" => red = red.max(n),
                "green" => green = green.max(n),
                "blue" => blue = blue.max(n),
                _ => panic!("Invalid color \"{color}\" or number \"{number}\" combination {part:?}"),
            }
        }
    }

    (red, green, blue)
}

