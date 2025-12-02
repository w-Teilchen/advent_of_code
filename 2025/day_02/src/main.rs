const INPUT: &str = include_str!("../input");
const EXAMPLE: &str = include_str!("../example");

fn main() {
    assert_eq!(part1(EXAMPLE), 1227775554);
    println!("Part 1: {}", part1(INPUT));
    assert_eq!(part2(EXAMPLE), 4174379265);
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> u64 {
    let ids = input.split(",");

    let mut sum_of_invalid_ids = 0_u64;

    for range in ids {
        let (start, end) = range.trim().split_once("-").unwrap();
        let start: u64 = start.parse().unwrap();
        let end: u64 = end.parse().unwrap();
        for id in start..(end+1) {
            sum_of_invalid_ids += is_invalid_id(id);
        }
    }
    sum_of_invalid_ids
}

fn is_invalid_id(id: u64) -> u64
{
    let number_of_digits = (id as f64).log10().floor() as u32 + 1;
    if number_of_digits % 2 == 1 {
        return 0;
    }
    if id / 10u64.pow(number_of_digits / 2) == id % 10u64.pow(number_of_digits / 2) {
        return id;
    }
    0
}

fn part2(input: &str) -> u64 {
    let ids = input.split(",");

    let mut sum_of_invalid_ids = 0_u64;

    for range in ids {
        let (start, end) = range.trim().split_once("-").unwrap();
        let start: u64 = start.parse().unwrap();
        let end: u64 = end.parse().unwrap();
        for id in start..(end+1) {
            sum_of_invalid_ids += is_invalid_id_2(id);
        }
    }
    sum_of_invalid_ids
}

fn is_invalid_id_2(id: u64) -> u64
{
    let id_as_string = id.to_string();

    let number_of_digits = (id as f64).log10().floor() as u32 + 1;
    for digits in 1..(number_of_digits/2+1) {
        let start = &id_as_string[0..(digits as usize)];
        let count = id_as_string.matches(start).count() as u32;
        if count * digits == number_of_digits {
            return id;
        }
    }
    0
}