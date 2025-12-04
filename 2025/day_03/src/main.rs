const INPUT: &str = include_str!("../input");
const EXAMPLE: &str = include_str!("../example");

fn main() {
    assert_eq!(part1(EXAMPLE), 357);
    println!("Part 1: {}", part1(INPUT));
    assert_eq!(part2(EXAMPLE), 3121910778619);
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> u64 {
    let mut summed_joltage = 0;
    for line in input.lines() {
        let joltage = calculate_joltage(line, 2);
        summed_joltage += joltage;
    }
    summed_joltage
}

fn part2(input: &str) -> u64 {
    let mut summed_joltage = 0_u64;
    for line in input.lines() {
        let joltage = calculate_joltage(line, 12);
        summed_joltage += joltage;
    }
    summed_joltage
}

fn calculate_joltage(line: &str, length: usize) -> u64 {
    let digits: Vec<u32> = line
    .chars()
    .map(|c| c.to_digit(10).unwrap())
    .collect();

    let mut joltage_digits: Vec<usize> = (0..length).collect();
    
    for i in 1..(digits.len()) {
        for j in 0..length {
            if (i < j) || (length - j > digits.len() - i) {
                continue;
            }
            if j != 0 && joltage_digits[j - 1] >= i {
                continue;
            }
            if digits[i] > digits[joltage_digits[j]] {
                for k in j..length {
                    joltage_digits[k] = i + k - j;
                }
                break;
            }
        }
    }
    let mut joltage = 0_u64;
    for j in 0..length {
        joltage += 10_u64.pow(j as u32) * digits[joltage_digits[length -1 - j]] as u64;
    }
    joltage
}