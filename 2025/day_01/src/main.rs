const INPUT: &str = include_str!("../input");
const EXAMPLE: &str = include_str!("../example");

fn main() {
    assert_eq!(part1(EXAMPLE), 3);
    println!("Part 1: {}", part1(INPUT));
    assert_eq!(part2(EXAMPLE), 6);
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    let orders = input.split_whitespace();

    let mut number_of_times_reaching_zero = 0;

    let mut accumulated_number :i32 = 50;
    for order in orders {
        let (direction,step_size) = parse_order(order);
        if direction == 'L' {
            accumulated_number -= step_size;
        } else {
            accumulated_number += step_size
        }

        if accumulated_number % 100 == 0 {
            number_of_times_reaching_zero += 1;
        }
    }
    number_of_times_reaching_zero
}

fn part2(input: &str) -> usize {
    let orders = input.split_whitespace();

    let mut number_of_times_crossing_zero = 0;

    let mut accumulated_number :i32 = 50;
    for order in orders {
        let (direction,step_size) = parse_order(order);
        if direction == 'L' {
            for _ in 0..step_size {
                accumulated_number -= 1;
                if accumulated_number % 100 == 0 {
                    number_of_times_crossing_zero += 1;
                }
            }
        } else {
            for _ in 0..step_size {
                accumulated_number += 1;
                if accumulated_number % 100 == 0 {
                    number_of_times_crossing_zero += 1;
                }
            }
        }
    }
    number_of_times_crossing_zero
}

fn parse_order(order: &str) -> (char, i32) {
    let (direction, number) = order.split_at(1);
    let direction = direction.chars().next().expect("Empty direction");
    let step_size = number.parse().expect("Failed to parse number");
    (direction, step_size)
}
