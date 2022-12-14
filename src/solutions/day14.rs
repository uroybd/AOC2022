// Advent of Code 2022 - Day 14

use std::{
    cmp::{max, min},
    collections::HashSet,
    fs,
};

fn get_rocks(pos1: (usize, usize), pos2: (usize, usize)) -> Vec<(usize, usize)> {
    if pos1.0 == pos2.0 {
        (min(pos1.1, pos2.1)..max(pos1.1, pos2.1) + 1)
            .map(|f| (pos1.0, f))
            .collect()
    } else {
        (min(pos1.0, pos2.0)..max(pos1.0, pos2.0) + 1)
            .map(|f| (f, pos1.1))
            .collect()
    }
}

fn parse_pair(pair_str: &str) -> (usize, usize) {
    let mut parts = pair_str.split(',');
    (
        parts.next().unwrap().parse().unwrap(),
        parts.next().unwrap().parse().unwrap(),
    )
}

fn parse_paths(input: &str) -> HashSet<(usize, usize)> {
    let positions: Vec<(usize, usize)> = input.split(" -> ").map(parse_pair).collect();
    positions
        .windows(2)
        .flat_map(|pair| get_rocks(pair[0], pair[1]))
        .collect()
}

fn next_available_pos(
    v: (usize, usize),
    paths: &HashSet<(usize, usize)>,
) -> Option<(usize, usize)> {
    [v.0, v.0 - 1, v.0 + 1]
        .map(|val| (val, v.1 + 1))
        .into_iter()
        .find(|p| !paths.contains(p))
}

fn find_destination(
    initial: (usize, usize),
    paths: &HashSet<(usize, usize)>,
    bottom: usize,
) -> Option<(usize, usize)> {
    let mut dest = initial;
    while let Some(p) = next_available_pos(dest, paths) {
        dest = p;
        if p.1 == bottom {
            break;
        }
    }
    if dest != initial {
        return Some(dest);
    }
    None
}

pub fn solution_day_14_01(file_path: String) -> Option<usize> {
    let mut paths: HashSet<(usize, usize)> = fs::read_to_string(file_path)
        .unwrap()
        .trim()
        .lines()
        .flat_map(parse_paths)
        .collect();
    let bottom = paths.iter().map(|x| x.1).max().unwrap();

    let mut step = 0;
    let s = (500, 0);
    while let Some(v) = find_destination(s, &paths, bottom) {
        if v.1 == bottom {
            break;
        }
        step += 1;
        paths.insert(v);
    }
    Some(step)
}

pub fn solution_day_14_02(file_path: String) -> Option<usize> {
    let mut paths: HashSet<(usize, usize)> = fs::read_to_string(file_path)
        .unwrap()
        .trim()
        .lines()
        .flat_map(parse_paths)
        .collect();
    let bottom = paths.iter().map(|x| x.1).max().unwrap() + 1;

    let mut step = 0;
    let s = (500, 0);
    loop {
        step += 1;
        if let Some(v) = find_destination(s, &paths, bottom) {
            paths.insert(v);
        } else {
            break;
        }
    }
    Some(step)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_14_01() {
        let file_path: String = String::from("src/inputs/day14e.txt");
        let result = solution_day_14_01(file_path).unwrap();
        assert_eq!(result, 24);
    }

    #[test]
    fn test_day_14_02() {
        let file_path: String = String::from("src/inputs/day14e.txt");
        let result = solution_day_14_02(file_path).unwrap();
        assert_eq!(result, 93);
    }

    #[test]
    #[ignore]
    fn output_day_14_01() {
        let file_path: String = String::from("src/inputs/day14.txt");
        let result = solution_day_14_01(file_path);
        dbg!(result.unwrap());
        assert_eq!(1, 1);
    }

    #[test]
    #[ignore]
    fn output_day_14_02() {
        let file_path: String = String::from("src/inputs/day14.txt");
        let result = solution_day_14_02(file_path);
        dbg!(result.unwrap());
        assert_eq!(1, 1);
    }
}
