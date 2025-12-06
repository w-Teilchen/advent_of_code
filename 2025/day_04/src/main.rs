const INPUT: &str = include_str!("../input");
const EXAMPLE: &str = include_str!("../example");

fn main() {
    let mut example_grid = convert_to_grid(EXAMPLE);
    let mut grid = convert_to_grid(INPUT);
    assert_eq!(part1(&example_grid), 13);
    println!("Part 1: {}", part1(&grid));
    assert_eq!(part2(&mut example_grid), 43);
    println!("Part 2: {}", part2(&mut grid));
}

fn convert_to_grid(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.bytes()
                .map(|b| match b {
                    b'.' => 0,
                    b'@' => 1,
                    _ => panic!("Unexpected character: {}", b),
                })
                .collect()
        })
        .collect()
}

fn part1(grid: &Vec<Vec<u8>>) -> u32 {
    let mut moveable_rolls = 0;
    for x in 0..grid[0].len() {
        for y in 0..grid.len() {
            if grid[y][x] == 0 {
                continue;
            }
            let neighboring_rolls = count_neighboring_rolls(grid, x, y);
            if neighboring_rolls < 4 {
                moveable_rolls += 1;
            }
        }
    }
    moveable_rolls
}

fn count_neighboring_rolls(grid: &Vec<Vec<u8>>, x: usize, y: usize) -> u32 {
    let mut neighboring_rolls = -1_i32;
    for i in x.saturating_sub(1)..(x + 2).min(grid[0].len()) {
        for j in y.saturating_sub(1)..(y + 2).min(grid.len()) {
            if grid[j][i] == 1 {
                neighboring_rolls += 1;
            }
        }
    }
    neighboring_rolls as u32
}

fn part2(grid: &mut Vec<Vec<u8>>) -> u32 {
    let mut moveable_rolls = 0;
    let mut moved_rolls: Vec<(usize, usize)> = Vec::new();
    moved_rolls.push((5, 7)); // to start while loop
    while moved_rolls.len() != 0 {
        moved_rolls.clear();
        for x in 0..grid[0].len() {
            for y in 0..grid.len() {
                if grid[y][x] == 0 {
                    continue;
                }
                let neighboring_rolls = count_neighboring_rolls(grid, x, y);
                if neighboring_rolls < 4 {
                    moved_rolls.push((x, y));
                }
            }
        }
        moveable_rolls += moved_rolls.len();
        for &(x, y) in moved_rolls.iter() {
            grid[y][x] = 0;
        }
    }
    moveable_rolls as u32
}
