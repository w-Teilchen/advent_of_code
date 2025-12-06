const INPUT: &str = include_str!("../input");
const EXAMPLE: &str = include_str!("../example");

fn main() {
    let (fresh_ranges, ingredients) = extract_ingredients(INPUT);
    let (example_fresh_ranges, example_ingredients) = extract_ingredients(EXAMPLE);
    assert_eq!(part1(&example_fresh_ranges, &example_ingredients), 3);
    println!("Part 1: {}", part1(&fresh_ranges, &ingredients));
    assert_eq!(part2(&example_fresh_ranges), 14);
    println!("Part 2: {}", part2(&fresh_ranges));
}

fn extract_ingredients(input: &str) -> (Vec<(usize, usize)>, Vec<usize>) {
    let (fresh_ranges_str, ingredients_str) = input
        .split_once("\r\n\r\n")
        .or_else(|| input.split_once("\n\n"))
        .unwrap();

    let fresh_ranges = fresh_ranges_str
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            (start.parse().unwrap(), end.parse().unwrap())
        })
        .collect();

    let ingredients = ingredients_str
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    (fresh_ranges, ingredients)
}

fn part1(fresh_ranges: &[(usize, usize)], ingredients: &[usize]) -> usize {
    let is_fresh = |ingredient: usize| {
        fresh_ranges
            .iter()
            .any(|&(start, end)| in_fresh_range(ingredient, (start, end)))
    };

    ingredients
        .iter()
        .filter(|&&ingredient| is_fresh(ingredient))
        .count()
}

fn in_fresh_range(ingredient: usize, fresh_range: (usize, usize)) -> bool {
    ingredient >= fresh_range.0 && ingredient <= fresh_range.1
}

fn part2(fresh_ranges: &[(usize, usize)]) -> usize {
    let mut sorted_ranges = fresh_ranges.to_vec();
    sorted_ranges.sort_by_key(|&(start, _)| start);

    let mut number_of_fresh_ingredients = 0;
    let mut current_range = sorted_ranges[0];
    for &(start, end) in &sorted_ranges[1..] {
        if in_fresh_range(start, current_range) {
            current_range.1 = current_range.1.max(end);
        } else {
            number_of_fresh_ingredients += current_range.1 - current_range.0 + 1;
            current_range = (start, end);
        }
    }
    number_of_fresh_ingredients + current_range.1 - current_range.0 + 1
}
