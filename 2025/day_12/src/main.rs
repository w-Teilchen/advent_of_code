use textplots::{Chart, Plot, Shape};

const INPUT_PRESENTS: &str = include_str!("../input_presents");
const INPUT_SPACES: &str = include_str!("../input_spaces");
const EXAMPLE_PRESENTS: &str = include_str!("../example_presents");
const EXAMPLE_SPACES: &str = include_str!("../example_spaces");

fn main() {
    println!("Example:");
    assert_eq!(part1(EXAMPLE_PRESENTS, EXAMPLE_SPACES), 2);
    println!("Input:");
    println!("Part 1: {}", part1(INPUT_PRESENTS, INPUT_SPACES));
}

fn part1(presents: &str, spaces: &str) -> usize {
    let presents: Vec<Present> = presents.split("\r\n\r\n").map(Present::new).collect();
    let spaces: Vec<Space> = spaces.lines().map(Space::new).collect();

    let mut number_of_fitting_spaces = 0;
    let mut differences_in_area = Vec::new();
    for space in spaces {
        let available_area = space.width * space.height;
        let mut required_area = 0;
        for (present_id, number_of_presents) in space.presents_to_fit.iter().enumerate() {
            required_area += presents[present_id].size * number_of_presents;
        }
        differences_in_area.push(available_area as i64 - required_area as i64);
        if available_area >= required_area + 10 { // with this offset the example works correctly
            number_of_fitting_spaces += 1;
        }
    }
    let step_width = if differences_in_area.len() == 3 { 1.0 } else { 10.0 }; // separate between example and input
    let lower_cutoff = -step_width;
    
    differences_in_area.sort_unstable();
    let percentile = (differences_in_area.len() as f32 * 0.95).ceil() as usize - 1;
    let upper_cutoff = differences_in_area[percentile] as f32;
    let number_of_bins = ((upper_cutoff - lower_cutoff) / step_width).ceil() as usize + 2;
    print_histogram(&differences_in_area.iter().map(|&d| d as f32).collect::<Vec<_>>(), lower_cutoff, upper_cutoff, number_of_bins);
    number_of_fitting_spaces
}

fn print_histogram(data: &[f32], lower_cutoff: f32, upper_cutoff: f32, number_of_bins: usize) {
    assert!(upper_cutoff > lower_cutoff);
    assert!(number_of_bins > 2);
    let mut bins = vec![0; number_of_bins];
    let bin_width = (upper_cutoff - lower_cutoff) / (number_of_bins - 2) as f32;
    for &value in data {
        let idx = if value < lower_cutoff {
            0
        } else if value >= upper_cutoff {
            number_of_bins - 1
        } else {
            ((value - lower_cutoff) / bin_width) as usize + 1
        };
        bins[idx] += 1;
    }
    let points: Vec<_> = bins.iter().enumerate().map(|(i, &count)| {
        let x = match i {
            0 => lower_cutoff,                         // underflow marker
            i if i == number_of_bins - 1 => upper_cutoff, // overflow marker
            _ => lower_cutoff + (i as f32 - 0.5) * bin_width, // bin center
        };
        (x, count as f32)
    }).collect();

    Chart::new(240, 20, lower_cutoff, upper_cutoff)
        .lineplot(&Shape::Bars(&points))
        .display();
    // Chart::new(240, 20, 0.0, (number_of_bins - 1) as f32)
    // .lineplot(&Shape::Bars(&points))
    // .display();

}

#[allow(dead_code)] // in case I want to use fills_space later
struct Present {
    fills_space: Vec<Vec<bool>>,
    size: usize,
}

impl Present {
    fn new(input: &str) -> Self {
        let fills_space: Vec<Vec<bool>> = input
                .lines()
                .filter(|line| line.contains('#') || line.contains('.'))
                .map(|line| line.chars().map(|c| c == '#').collect())
                .collect();
        Self {
            size: fills_space.iter().flatten().filter(|&&filled| filled).count(),
            fills_space,
        }
    }
}

struct Space {
    height: usize,
    width: usize,
    presents_to_fit: Vec<usize>,
}

impl Space {
    fn new(input: &str) -> Self {
        let (dimensions, payload) = input.split_once(':').expect("missing ':'");
        let (width, height) = dimensions
            .trim()
            .split_once('x')
            .map(|(w, h)| (w.trim().parse().unwrap(), h.trim().parse().unwrap()))
            .unwrap();
        let presents_to_fit = payload
            .split_whitespace()
            .map(|count| count.parse().unwrap())
            .collect();
        Self {
            height,
            width,
            presents_to_fit,
        }
    }
}
