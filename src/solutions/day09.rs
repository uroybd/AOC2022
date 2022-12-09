// Advent of Code 2022 - Day 09

use std::collections::HashSet;

use crate::utils::read::read_lines;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Coordinate {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Rope {
    links: Vec<Coordinate>,
    movement_record: HashSet<Coordinate>,
}

impl Rope {
    fn at_start(tails: usize) -> Self {
        let mut rope = Rope {
            links: vec![Coordinate { x: 0, y: 0 }; tails + 1],
            movement_record: HashSet::new(),
        };
        rope.movement_record.insert(Coordinate { x: 0, y: 0 });
        rope
    }

    fn move_to_direction(&mut self, direction: &str) {
        let diff = match direction {
            "U" => (0, 1),
            "D" => (0, -1),
            "R" => (1, 0),
            "L" => (-1, 0),
            _ => unreachable!(),
        };
        self.links[0].x += diff.0;
        self.links[0].y -= diff.1;

        let length = self.links.len();

        for i in 1..length {
            let dx = self.links[i - 1].x - self.links[i].x;
            let dy = self.links[i - 1].y - self.links[i].y;
            if dx.abs() < 2 && dy.abs() < 2 {
                continue;
            }
            if dx != 0 {
                self.links[i].x += dx / dx.abs();
            }
            if dy != 0 {
                self.links[i].y += dy / dy.abs();
            }
        }
        self.movement_record.insert(self.links[length - 1].clone());
    }

    fn run_instruction(&mut self, instruction: &str) {
        let mut parts = instruction.split_whitespace();
        let (direction, amount) = (
            parts.next().unwrap(),
            parts.next().unwrap().parse::<usize>().unwrap(),
        );
        for _ in 0..amount {
            self.move_to_direction(direction);
        }
    }

    fn count_visited(&self) -> usize {
        self.movement_record.len()
    }
}

pub fn solution_day_09_01(file_path: String) -> Option<usize> {
    let instructions = read_lines(file_path);
    let mut rope = Rope::at_start(1);
    for instruction in instructions.iter() {
        rope.run_instruction(instruction);
    }
    Some(rope.count_visited())
}

pub fn solution_day_09_02(file_path: String) -> Option<usize> {
    let instructions = read_lines(file_path);
    let mut rope = Rope::at_start(9);
    for instruction in instructions.iter() {
        rope.run_instruction(instruction);
    }
    Some(rope.count_visited())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_09_01() {
        let file_path: String = String::from("src/inputs/day09e.txt");
        let result = solution_day_09_01(file_path).unwrap();
        assert_eq!(result, 13);
    }

    #[test]
    fn test_day_09_02() {
        let file_path: String = String::from("src/inputs/day09ee.txt");
        let result = solution_day_09_02(file_path).unwrap();
        assert_eq!(result, 36);
    }

    #[test]
    #[ignore]
    fn output_day_09_01() {
        let file_path: String = String::from("src/inputs/day09.txt");
        let result = solution_day_09_01(file_path);
        dbg!(result.unwrap());
        assert_eq!(1, 1);
    }

    #[test]
    #[ignore]
    fn output_day_09_02() {
        let file_path: String = String::from("src/inputs/day09.txt");
        let result = solution_day_09_02(file_path);
        dbg!(result.unwrap());
        assert_eq!(1, 1);
    }
}
