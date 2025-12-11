use cached::proc_macro::cached;
use std::{sync::Arc, vec};

const INPUT: &str = include_str!("../input");
const EXAMPLE: &str = include_str!("../example");

fn main() {
    assert_eq!(part1(EXAMPLE), 50);
    println!("Part 1: {}", part1(INPUT));
    assert_eq!(part2(EXAMPLE), 24);
    println!("Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    let corners: Vec<(usize, usize)> = input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(',').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect();

    let mut maximum_area = 0;
    for (i, &(x1, y1)) in corners.iter().enumerate() {
        for &(x2, y2) in corners.iter().skip(i + 1) {
            let width = (x2 as i32 - x1 as i32).unsigned_abs() as usize + 1;
            let height = (y2 as i32 - y1 as i32).unsigned_abs() as usize + 1;
            let area = width * height;
            if area > maximum_area {
                maximum_area = area;
            }
        }
    }
    maximum_area
}

fn part2(input: &str) -> usize {
    let mut red_tiles: Vec<(usize, usize)> = input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(',').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect();
    red_tiles.push(red_tiles[0]); // close the loop

    let mut edges: Vec<((usize, usize), (usize, usize))> = Vec::new();
    for window in red_tiles.windows(2) {
        let ((a1, a2), (b1, b2)) = (window[0], window[1]);
        edges.push(((a1, a2), (b1, b2)));
    }
    let edges = Arc::new(edges);

    let mut maximum_area = 0;
    let start_time = std::time::Instant::now();
    for (i, &(x1, y1)) in red_tiles.iter().enumerate() {
        println!(
            "After {} ms starting with corner {i}.",
            start_time.elapsed().as_millis()
        );
        for &(x2, y2) in red_tiles.iter().skip(i + 1) {
            let upper_left_hand_corner = (x1.min(x2), y1.min(y2));
            let lower_right_hand_corner = (x1.max(x2), y1.max(y2));
            let width = (lower_right_hand_corner.0 - upper_left_hand_corner.0) + 1;
            let height = (lower_right_hand_corner.1 - upper_left_hand_corner.1) + 1;
            let area = width * height;
            if area > maximum_area {
                if !rectangle_is_inside_polygon(
                    upper_left_hand_corner,
                    lower_right_hand_corner,
                    edges.clone(),
                ) {
                    continue;
                }
                maximum_area = area;
            }
        }
    }
    maximum_area
}

fn rectangle_is_inside_polygon(
    (x1, y1): (usize, usize),
    (x2, y2): (usize, usize),
    polygon: Arc<Vec<((usize, usize), (usize, usize))>>,
) -> bool {
    let edges = vec![
        ((x1, y1), (x2, y1)),
        ((x2, y1), (x2, y2)),
        ((x2, y2), (x1, y2)),
        ((x1, y2), (x1, y1)),
    ];
    let edges = Arc::new(edges);
    for &rectangle_edge in edges.iter() {
        for &polygon_edge in polygon.iter() {
            if let Some(intersection) = edges_intersect(rectangle_edge, polygon_edge) {
                let mut test_points = Vec::new();
                for x_offset in [-1, 1] {
                    test_points.push((
                        (intersection.0 as isize + x_offset) as usize,
                        intersection.1,
                    ));
                }
                for y_offset in [-1, 1] {
                    test_points.push((
                        intersection.0,
                        (intersection.1 as isize + y_offset) as usize,
                    ));
                }
                for test_point in test_points {
                    if is_inside_polygon(test_point, edges.clone())
                        && !is_inside_polygon(test_point, polygon.clone())
                    {
                        return false;
                    }
                }
            }
        }
    }
    true
}

fn edges_intersect(
    ((x1a, y1a), (x1b, y1b)): ((usize, usize), (usize, usize)),
    ((x2a, y2a), (x2b, y2b)): ((usize, usize), (usize, usize)),
) -> Option<(usize, usize)> {
    let first_edge_is_horizontal = y1a == y1b;
    let second_edge_is_horizontal = y2a == y2b;
    if first_edge_is_horizontal && second_edge_is_horizontal
        || !first_edge_is_horizontal && !second_edge_is_horizontal
    {
        return None;
    }
    if first_edge_is_horizontal {
        // y1a == y1b
        if (x2a >= x1a.min(x1b))
            && (x2a <= x1a.max(x1b))
            && (y1a >= y2a.min(y2b))
            && (y1a <= y2a.max(y2b))
        {
            return Some((x2a, y1a));
        }
    } else if (x1a >= x2a.min(x2b))
            && (x1a <= x2a.max(x2b))
            && (y2a >= y1a.min(y1b))
            && (y2a <= y1a.max(y1b))
    {
        return Some((x1a, y2a));
    }
    None
}

#[cached]
fn is_inside_polygon(
    point: (usize, usize),
    edges: Arc<Vec<((usize, usize), (usize, usize))>>,
) -> bool {
    let (x, y) = point;
    let mut edges_to_the_right = 0_u32;
    let mut edges_to_the_left = 0_u32;
    let mut edges_above = 0_u32;
    let mut edges_below = 0_u32;
    for &((x1, y1), (x2, y2)) in edges.iter() {
        if x1 == x2 {
            // vertical edge
            if y1.min(y2) <= y && y < y1.max(y2) {
                if x == x1 {
                    // on the edge counts as inside
                    return true;
                }
                if x < x1 {
                    edges_to_the_right += 1;
                }
                if x > x1 {
                    edges_to_the_left += 1;
                }
            }
        }
        if y1 == y2 {
            // horizontal edge
            if x1.min(x2) <= x && x < x1.max(x2) {
                if y == y1 {
                    // on the edge counts as inside
                    return true;
                }
                if y < y1 {
                    edges_above += 1;
                }
                if y > y1 {
                    edges_below += 1;
                }
            }
        }
    }
    (edges_to_the_right % 2 == 1)
        && (edges_to_the_left % 2 == 1)
        && (edges_above % 2 == 1)
        && (edges_below % 2 == 1)
}

#[test]
fn test_is_inside_polygon() {
    use std::sync::Arc;

    // Define corners in order
    let corners = [
        (7, 1),
        (11, 1),
        (11, 7),
        (9, 7),
        (9, 5),
        (2, 5),
        (2, 3),
        (7, 3),
    ];

    // Convert corners into edges
    let edges = Arc::new(
        corners
            .iter()
            .zip(corners.iter().cycle().skip(1)) // pair each point with the next, wrapping around
            .map(|(a, b)| (*a, *b))
            .collect::<Vec<_>>(),
    );

    assert!(is_inside_polygon((8, 2), edges.clone()));
    assert!(is_inside_polygon((10, 6), edges.clone()));
    assert!(is_inside_polygon((9, 3), edges.clone()));
    assert!(!is_inside_polygon((0, 0), edges.clone()));
    assert!(!is_inside_polygon((2, 1), edges.clone()));
    assert!(!is_inside_polygon((12, 4), edges.clone()));
}
