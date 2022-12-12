// Advent of Code 2022 - Day 12

use crate::utils::collections::Faux2DArray;
use std::cmp::*;
use std::collections::{BinaryHeap, HashMap};
use std::{collections::HashSet, fs};

#[derive(Debug)]
struct Step {
    id: usize,
    height: u8,
}

#[derive(Eq, Debug)]
struct Connection {
    vertex: usize,
    distance: usize,
}

fn char_to_num(c: char) -> u8 {
    match c {
        'S' => 0,
        'E' => 25,
        _ => c as u8 - 97,
    }
}

fn parse_input(file_path: String) -> (Faux2DArray<Step>, usize, usize) {
    let mut steps = Faux2DArray::new(5);
    let mut goal = 0;
    let mut start = 0;
    fs::read_to_string(file_path)
        .unwrap()
        .trim()
        .lines()
        .enumerate()
        .for_each(|(y, l)| {
            let width = l.len();
            let row: Vec<Step> = l
                .chars()
                .enumerate()
                .map(|(x, c)| {
                    let id = (width * y) + x;
                    if c == 'E' {
                        goal = id;
                    }
                    if c == 'S' {
                        start = id;
                    }
                    Step {
                        height: char_to_num(c),
                        id,
                    }
                })
                .collect();
            steps.width = row.len();
            steps.add_row(row).unwrap();
        });
    (steps, start, goal)
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
        self.distance == other.distance && self.vertex == other.vertex
    }
}

fn calculate_distances(steps: &Faux2DArray<Step>, start: usize, end: usize) -> Option<usize> {
    let mut connections = HashMap::new();
    let mut distances = HashMap::new();
    for y in 0..steps.height() {
        for x in 0..steps.width {
            let mut l_dist = Vec::new();
            let current = steps.at(x, y);
            if let Some(v) = steps.prev_x(x, y) {
                if current.height + 1 >= v.height {
                    l_dist.push(Connection {
                        vertex: v.id,
                        distance: 1,
                    });
                }
            }
            if let Some(v) = steps.prev_y(x, y) {
                if current.height + 1 >= v.height {
                    l_dist.push(Connection {
                        vertex: v.id,
                        distance: 1,
                    });
                }
            }
            if let Some(v) = steps.next_x(x, y) {
                if current.height + 1 >= v.height {
                    l_dist.push(Connection {
                        vertex: v.id,
                        distance: 1,
                    });
                }
            }
            if let Some(v) = steps.next_y(x, y) {
                if current.height + 1 >= v.height {
                    l_dist.push(Connection {
                        vertex: v.id,
                        distance: 1,
                    });
                }
            }
            if !l_dist.is_empty() {
                connections.insert(current.id, l_dist);
            }
            distances.insert(current.id, usize::MAX);
        }
    }
    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new();

    queue.push(Connection {
        vertex: start,
        distance: 0,
    });

    let mut current = start;
    while current != end && !queue.is_empty() {
        let c = queue.pop().unwrap();
        current = c.vertex;
        let distance = c.distance;
        if !visited.contains(&current) {
            visited.insert(current);
            distances.insert(current, distance);
            for v in connections.get(&current).unwrap() {
                if !visited.contains(&v.vertex) {
                    queue.push(Connection {
                        vertex: v.vertex,
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
    calculate_distances(&steps, start, end)
}

pub fn solution_day_12_02(file_path: String) -> Option<usize> {
    let (steps, _, end) = parse_input(file_path);
    let lowest: Vec<usize> = steps
        .items
        .iter()
        .filter(|s| s.height == 0)
        .map(|s| s.id)
        .collect();
    let mut shortest = usize::MAX;
    for s in lowest {
        if let Some(res) = calculate_distances(&steps, s, end) {
            if res < shortest {
                shortest = res;
            }
        }
    }

    Some(shortest)
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
