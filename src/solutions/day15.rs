// Advent of Code 2022 - Day 15

use std::{fs, str::FromStr, string::ParseError};

type Coordinates = (isize, isize);

#[derive(Debug)]
struct Sensor {
    coordinates: Coordinates,
    beacon: Coordinates,
    radius: isize,
}

impl FromStr for Sensor {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(':');
        let coordinates = parse_coordinates(parts.next().unwrap());
        let beacon = parse_coordinates(parts.next().unwrap());
        let sensor = Self {
            coordinates,
            beacon,
            radius: manhattan_distance(coordinates, beacon),
        };
        Ok(sensor)
    }
}

impl Sensor {
    fn in_radius(&self, coords: Coordinates) -> bool {
        if coords == self.beacon {
            false
        } else {
            self.radius >= manhattan_distance(self.coordinates, coords)
        }
    }
}

fn manhattan_distance(coords1: Coordinates, coords2: Coordinates) -> isize {
    (coords1.0.abs_diff(coords2.0) + coords1.1.abs_diff(coords2.1)) as isize
}

pub fn parse_value(inp: &str) -> isize {
    inp.split('=').last().unwrap().parse().unwrap()
}

pub fn parse_coordinates(inp: &str) -> Coordinates {
    let mut parts = inp.split("at ").last().unwrap().split(", ");
    (
        parse_value(parts.next().unwrap()),
        parse_value(parts.next().unwrap()),
    )
}

fn parse_input(file_path: String) -> Vec<Sensor> {
    fs::read_to_string(file_path)
        .unwrap()
        .trim()
        .lines()
        .map(|l| Sensor::from_str(l).unwrap())
        .collect()
}

pub fn solution_day_15_01(file_path: String, y: isize) -> Option<usize> {
    let sensors = parse_input(file_path);
    let x_bound = (
        sensors
            .iter()
            .map(|s| s.coordinates.0 - (s.radius as isize))
            .min()
            .unwrap(),
        sensors
            .iter()
            .map(|s| s.coordinates.0 + (s.radius as isize))
            .max()
            .unwrap(),
    );

    Some(
        (x_bound.0..=x_bound.1)
            .filter(|x| sensors.iter().any(|s| s.in_radius((*x, y))))
            .count(),
    )
}

pub fn solution_day_15_02(file_path: String, bound: isize) -> Option<isize> {
    let sensors = parse_input(file_path);
    sensors.iter().find_map(|s| {
        ((s.coordinates.0 - (s.radius as isize) - 1).max(0)..=s.coordinates.0.min(bound))
            .zip(s.coordinates.1..=(s.coordinates.1 + (s.radius as isize)).min(bound))
            .find_map(|coords| {
                sensors
                    .iter()
                    .all(|s| !s.in_radius(coords))
                    .then_some(coords.0 * 4000000 + coords.1)
            })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_15_01() {
        let file_path: String = String::from("src/inputs/day15e.txt");
        let result = solution_day_15_01(file_path, 10).unwrap();
        assert_eq!(result, 26);
    }

    #[test]
    fn test_day_15_02() {
        let file_path: String = String::from("src/inputs/day15e.txt");
        let result = solution_day_15_02(file_path, 20).unwrap();
        assert_eq!(result, 56000011);
    }

    #[test]
    #[ignore]
    fn output_day_15_01() {
        let file_path: String = String::from("src/inputs/day15.txt");
        let result = solution_day_15_01(file_path, 2000000);
        dbg!(result.unwrap());
        assert_eq!(1, 1);
    }

    #[test]
    #[ignore]
    fn output_day_15_02() {
        let file_path: String = String::from("src/inputs/day15.txt");
        let result = solution_day_15_02(file_path, 4000000);
        dbg!(result.unwrap());
        assert_eq!(1, 1);
    }
}
