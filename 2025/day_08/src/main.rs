use core::num;
use std::collections::HashMap;

use lib::*;

const INPUT: &str = include_str!("../input");
const EXAMPLE: &str = include_str!("../example");

fn main() {
    let mut example_boxes = extract_juction_boxes(EXAMPLE);
    // example_boxes.sort();
    let example_distances = calculate_distances(&example_boxes);
    let mut input_boxes = extract_juction_boxes(INPUT);
    // input_boxes.sort();
    let input_distances = calculate_distances(&input_boxes);

    assert_eq!(
        part1(&mut example_boxes.clone(), &example_distances, 10),
        40
    );
    println!(
        "Part 1: {}",
        part1(&mut input_boxes.clone(), &input_distances, 1000)
    );
    assert_eq!(part2(&mut example_boxes, &example_distances), 25272);
    println!("Part 2: {}", part2(&mut input_boxes, &input_distances));
}

fn part1(
    junction_boxes: &mut [JunctionBox],
    distances: &Vec<(u64, usize, usize)>,
    desired_number_of_connections: u32,
) -> usize {
    let mut connections: HashMap<u32, Vec<usize>> = HashMap::new();
    let mut number_of_connections: u32 = 0;
    for &(_, i, j) in distances {
        if number_of_connections >= desired_number_of_connections {
            break;
        }
        if junction_boxes[i].circuit_id.is_none() && junction_boxes[j].circuit_id.is_none() {
            let circuit_id = number_of_connections;
            junction_boxes[i].circuit_id = Some(circuit_id);
            junction_boxes[j].circuit_id = Some(circuit_id);
            connections.insert(circuit_id, vec![i, j]);
        } else if junction_boxes[i].circuit_id.is_some() && junction_boxes[j].circuit_id.is_none() {
            let circuit_id = junction_boxes[i].circuit_id.unwrap();
            junction_boxes[j].circuit_id = Some(circuit_id);
            connections.get_mut(&circuit_id).unwrap().push(j);
        } else if junction_boxes[i].circuit_id.is_none() && junction_boxes[j].circuit_id.is_some() {
            let circuit_id = junction_boxes[j].circuit_id.unwrap();
            junction_boxes[i].circuit_id = Some(circuit_id);
            connections.get_mut(&circuit_id).unwrap().push(i);
        } else if junction_boxes[i].circuit_id.unwrap() != junction_boxes[j].circuit_id.unwrap() {
            let circuit_id_i = junction_boxes[i].circuit_id.unwrap();
            let circuit_id_j = junction_boxes[j].circuit_id.unwrap();
            let boxes_to_move = connections.remove(&circuit_id_j).unwrap();
            for &box_id in &boxes_to_move {
                junction_boxes[box_id].circuit_id = Some(circuit_id_i);
            }
            connections
                .get_mut(&circuit_id_i)
                .unwrap()
                .extend(boxes_to_move);
        }
        number_of_connections += 1;
    }
    println!("Total connections made: {}", number_of_connections);
    let mut score = 1;
    let mut connection_sizes: Vec<usize> = connections.values().map(|c| c.len()).collect();
    connection_sizes.sort_by(|a, b| b.cmp(a));
    for &size in connection_sizes.iter().take(3) {
        println!("Connection with {} boxes", size);
        score *= size;
    }
    score
}

fn part2(junction_boxes: &mut [JunctionBox], distances: &Vec<(u64, usize, usize)>) -> usize {
    let mut connections: HashMap<u32, Vec<usize>> = HashMap::new();
    let mut number_of_connections: u32 = 0;
    let last_connection = 'outer: loop {
        for &(_, i, j) in distances {
            if junction_boxes[i].circuit_id.is_none() && junction_boxes[j].circuit_id.is_none() {
                let circuit_id = number_of_connections;
                junction_boxes[i].circuit_id = Some(circuit_id);
                junction_boxes[j].circuit_id = Some(circuit_id);
                connections.insert(circuit_id, vec![i, j]);
            } else if junction_boxes[i].circuit_id.is_some()
                && junction_boxes[j].circuit_id.is_none()
            {
                let circuit_id = junction_boxes[i].circuit_id.unwrap();
                junction_boxes[j].circuit_id = Some(circuit_id);
                connections.get_mut(&circuit_id).unwrap().push(j);
            } else if junction_boxes[i].circuit_id.is_none()
                && junction_boxes[j].circuit_id.is_some()
            {
                let circuit_id = junction_boxes[j].circuit_id.unwrap();
                junction_boxes[i].circuit_id = Some(circuit_id);
                connections.get_mut(&circuit_id).unwrap().push(i);
            } else if junction_boxes[i].circuit_id.unwrap() != junction_boxes[j].circuit_id.unwrap()
            {
                let circuit_id_i = junction_boxes[i].circuit_id.unwrap();
                let circuit_id_j = junction_boxes[j].circuit_id.unwrap();
                let boxes_to_move = connections.remove(&circuit_id_j).unwrap();
                for &box_id in &boxes_to_move {
                    junction_boxes[box_id].circuit_id = Some(circuit_id_i);
                }
                connections
                    .get_mut(&circuit_id_i)
                    .unwrap()
                    .extend(boxes_to_move);
            }
            number_of_connections += 1;
            if connections.len() == 1
                && connections.values().next().unwrap().len() == junction_boxes.len()
            {
                break 'outer (i, j);
            }
        }
    };
    junction_boxes[last_connection.0].x as usize * junction_boxes[last_connection.1].x as usize
}

fn extract_juction_boxes(input: &str) -> Vec<JunctionBox> {
    input
        .lines()
        .enumerate()
        .map(|(id, line)| {
            let mut numbers = line.split(',').map(|n| n.parse::<u32>().unwrap());
            JunctionBox {
                id: id,
                x: numbers.next().unwrap(),
                y: numbers.next().unwrap(),
                z: numbers.next().unwrap(),
                circuit_id: None,
            }
        })
        .collect()
}

fn calculate_distances(junction_boxes: &[JunctionBox]) -> Vec<(u64, usize, usize)> {
    let mut distances = Vec::new();
    for (i, box1) in junction_boxes.iter().enumerate() {
        for box2 in junction_boxes.iter().skip(i + 1) {
            let distance = ((box2.x as i64 - box1.x as i64).pow(2)
                + (box2.y as i64 - box1.y as i64).pow(2)
                + (box2.z as i64 - box1.z as i64).pow(2)) as u64;
            distances.push((distance, box1.id, box2.id));
        }
    }
    distances.sort();
    distances
}

// create a struct for junction box
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct JunctionBox {
    id: usize,

    x: u32,
    y: u32,
    z: u32,

    circuit_id: Option<u32>,
}
