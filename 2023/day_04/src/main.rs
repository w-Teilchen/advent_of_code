use std::collections::HashSet;

const INPUT: &str = include_str!("../input");
const EXAMPLE: &str = include_str!("../example");

fn main() {
    assert_eq!(part1(EXAMPLE), 13);
    println!("Part 1: {}", part1(INPUT));

    assert_eq!(part2(EXAMPLE), 30);
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> u32 {
    let mut sum = 0;

    for line in input.lines() {
        let line = line.split_once(':').unwrap().1;
        let (winning_numbers, actual_numbers) = line.split_once("|").unwrap();

        let winning_numbers: HashSet<_> = winning_numbers.split_whitespace().collect();
        let actual_numbers: HashSet<_> = actual_numbers.split_whitespace().collect();
        let number_of_matches = winning_numbers.intersection(&actual_numbers).count();

        if number_of_matches > 0 {
            sum += 2u32.pow(number_of_matches as u32 - 1);
        }
    }
    sum
}

fn part2(input: &str) -> u32 {
    let number_of_games = input.lines().count() as u32;
    let mut number_of_cards: Vec<u32> = vec![1; number_of_games as usize];

    for line in input.lines() {
        let (description, line) = line.split_once(':').unwrap();
        println!("{description}");
        let card_index = description.split_whitespace()
            .nth(1)
            .unwrap()
            .parse::<u32>()
            .unwrap();


        let (winning_numbers, actual_numbers) = line.split_once("|").unwrap();
        
        let winning_numbers: HashSet<_> = winning_numbers.split_whitespace().collect();
        let actual_numbers: HashSet<_> = actual_numbers.split_whitespace().collect();
        let number_of_matches = winning_numbers.intersection(&actual_numbers).count() as u32;

        for index in card_index..(card_index+number_of_matches).min(number_of_cards.len() as u32){
            number_of_cards[index as usize] += number_of_cards[card_index.saturating_sub(1) as usize];
        }
    }

    number_of_cards.iter().sum()
}