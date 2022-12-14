// Advent of Code 2022 - Day 09

use std::{collections::HashSet, fs};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Coordinate {
    x: isize,
    y: isize,
}

impl Coordinate {
    fn move_by(&mut self, to: (isize, isize)) {
        self.x += to.0;
        self.y += to.1;
    }

    fn move_close_to(&mut self, to: (isize, isize)) {
        let dx = to.0 - self.x;
        let dy = to.1 - self.y;
        if dx.abs() >= 2 || dy.abs() >= 2 {
            self.move_by((dx.signum(), dy.signum()));
        }
    }

    fn coordinate_tuple(&self) -> (isize, isize) {
        (self.x, self.y)
    }
}

#[derive(Debug)]
struct Rope {
    links: Vec<Coordinate>,
    movement_record: HashSet<(isize, isize)>,
}

impl Rope {
    fn at_start(tails: usize) -> Self {
        Rope {
            links: vec![Coordinate { x: 0, y: 0 }; tails + 1],
            movement_record: HashSet::from([(0, 0)]),
        }
    }

    fn move_to_direction(&mut self, direction: &str) {
        let diff = match direction {
            "U" => (0, 1),
            "D" => (0, -1),
            "R" => (1, 0),
            "L" => (-1, 0),
            _ => unreachable!(),
        };
        self.links[0].move_by(diff);

        let length = self.links.len();

        for idx in 1..length {
            let to = self.links[idx - 1].coordinate_tuple();
            self.links[idx].move_close_to(to);
        }
        self.movement_record
            .insert(self.links[length - 1].coordinate_tuple());
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
}

pub fn solution_day_09_01(file_path: String) -> Option<usize> {
    let mut rope = Rope::at_start(1);
    for instruction in fs::read_to_string(file_path).unwrap().trim().lines() {
        rope.run_instruction(instruction);
    }
    Some(rope.movement_record.len())
}

pub fn solution_day_09_02(file_path: String) -> Option<usize> {
    let mut rope = Rope::at_start(9);
    for instruction in fs::read_to_string(file_path).unwrap().trim().lines() {
        rope.run_instruction(instruction);
    }
    Some(rope.movement_record.len())
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
