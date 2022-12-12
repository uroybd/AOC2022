// Advent of Code 2022 - Day 12

use crate::utils::collections::Faux2DArray;
use std::cmp::*;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;

#[derive(Eq, Debug)]
struct Connection {
    pos: usize,
    distance: usize,
}

impl Ord for Connection {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for Connection {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance && self.pos == other.pos
    }
}

fn char_to_num(c: char) -> u8 {
    match c {
        'S' => 0,
        'E' => 25,
        _ => c as u8 - 97,
    }
}

fn parse_input(file_path: String) -> (Faux2DArray<u8>, usize, usize) {
    let mut steps = Faux2DArray::new(5);
    let mut goal = (0, 0);
    let mut start = (0, 0);
    fs::read_to_string(file_path)
        .unwrap()
        .trim()
        .lines()
        .enumerate()
        .for_each(|(y, l)| {
            let row: Vec<u8> = l
                .chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'E' {
                        goal = (x, y);
                    }
                    if c == 'S' {
                        start = (x, y);
                    }
                    char_to_num(c)
                })
                .collect();
            steps.width = row.len();
            steps.add_row(row).unwrap();
        });

    let r_start = steps.absolute_index(start.0, start.1);
    let r_goal = steps.absolute_index(goal.0, goal.1);
    (steps, r_start, r_goal)
}

fn create_path_map(steps: &Faux2DArray<u8>) -> HashMap<usize, Vec<Connection>> {
    let mut connections = HashMap::new();
    for (idx, current) in steps.items.iter().enumerate() {
        let (x, y) = steps.cartesian_index(idx);
        let mut l_dist = Vec::new();
        let mut neighbors = vec![(x + 1, y), (x, y + 1)];
        if x > 0 {
            neighbors.push((x - 1, y));
        }
        if y > 0 {
            neighbors.push((x, y - 1))
        }
        for (x, y) in neighbors {
            if let Some(v) = steps.at(x, y) {
                if current + 1 >= *v {
                    l_dist.push(Connection {
                        pos: steps.absolute_index(x, y),
                        distance: 1,
                    });
                }
            }
        }
        if !l_dist.is_empty() {
            connections.insert(idx, l_dist);
        }
    }
    connections
}

fn shortest_distance(
    connections: &HashMap<usize, Vec<Connection>>,
    size: usize,
    start: usize,
    end: usize,
) -> Option<usize> {
    let mut distances: HashMap<usize, usize> =
        vec![usize::MAX; size].into_iter().enumerate().collect();
    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new();

    queue.push(Connection {
        pos: start,
        distance: 0,
    });

    let mut current = start;
    while current != end && !queue.is_empty() {
        let c = queue.pop().unwrap();
        current = c.pos;
        let distance = c.distance;
        if !visited.contains(&current) {
            visited.insert(current);
            distances.insert(current, distance);
            for v in connections.get(&current).unwrap() {
                if !visited.contains(&v.pos) {
                    queue.push(Connection {
                        pos: v.pos,
                        distance: v.distance + distance,
                    });
                }
            }
        }
    }
    distances.get(&end).copied()
}

pub fn solution_day_12_01(file_path: String) -> Option<usize> {
    let (steps, start, end) = parse_input(file_path);
    let positions = create_path_map(&steps);
    shortest_distance(&positions, steps.items.len(), start, end)
}

pub fn solution_day_12_02(file_path: String) -> Option<usize> {
    let (steps, _, end) = parse_input(file_path);
    let positions = create_path_map(&steps);
    let length = steps.items.len();

    steps
        .items
        .iter()
        .enumerate()
        .filter(|(_, s)| s == &&0)
        .map(|(idx, _)| shortest_distance(&positions, length, idx, end))
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_12_01() {
        let file_path: String = String::from("src/inputs/day12e.txt");
        let result = solution_day_12_01(file_path).unwrap();
        assert_eq!(result, 31);
    }

    #[test]
    fn test_day_12_02() {
        let file_path: String = String::from("src/inputs/day12e.txt");
        let result = solution_day_12_02(file_path).unwrap();
        assert_eq!(result, 29);
    }

    #[test]
    #[ignore]
    fn output_day_12_01() {
        let file_path: String = String::from("src/inputs/day12.txt");
        let result = solution_day_12_01(file_path);
        dbg!(result.unwrap());
        assert_eq!(1, 1);
    }

    #[test]
    #[ignore]
    fn output_day_12_02() {
        let file_path: String = String::from("src/inputs/day12.txt");
        let result = solution_day_12_02(file_path);
        dbg!(result.unwrap());
        assert_eq!(1, 1);
    }
}
