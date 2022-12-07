// Advent of Code 2022 - Day 04

use crate::utils::read::read_lines;

fn parse_ranges(inp: &str) -> [[usize; 2]; 2] {
    let mut iter = inp.split(',').map(|r| {
        let mut parts = r.split('-');
        [
            parts.next().unwrap().parse::<usize>().unwrap(),
            parts.next().unwrap().parse::<usize>().unwrap(),
        ]
    });
    [iter.next().unwrap(), iter.next().unwrap()]
}

fn are_contained(pair: [[usize; 2]; 2]) -> bool {
    if pair[0][0] <= pair[1][0] && pair[0][1] >= pair[1][1] {
        return true;
    }
    if pair[0][0] >= pair[1][0] && pair[0][1] <= pair[1][1] {
        return true;
    }
    false
}

fn are_overlapping(pair: [[usize; 2]; 2]) -> bool {
    if pair[0][1] >= pair[1][0] && pair[1][1] >= pair[0][0] {
        return true;
    }
    false
}

pub fn solution_day_04_01(file_path: String) -> Option<usize> {
    let input = read_lines(file_path);
    let result = input
        .iter()
        .map(|l| are_contained(parse_ranges(l)) as usize)
        .sum();
    Some(result)
}

pub fn solution_day_04_02(file_path: String) -> Option<usize> {
    let input = read_lines(file_path);
    let result = input
        .iter()
        .map(|l| are_overlapping(parse_ranges(l)) as usize)
        .sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_04_01() {
        let file_path: String = String::from("src/inputs/day04e.txt");
        let result = solution_day_04_01(file_path).unwrap();
        assert_eq!(result, 2);
    }

    #[test]
    fn test_day_04_02() {
        let file_path: String = String::from("src/inputs/day04e.txt");
        let result = solution_day_04_02(file_path).unwrap();
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore]
    fn output_day_04_01() {
        let file_path: String = String::from("src/inputs/day04.txt");

        let result = solution_day_04_01(file_path);

        dbg!(result.unwrap());
        assert_eq!(1, 1);
    }

    #[test]
    #[ignore]
    fn output_day_04_02() {
        let file_path: String = String::from("src/inputs/day04.txt");

        let result = solution_day_04_02(file_path);

        dbg!(result.unwrap());
        assert_eq!(1, 1);
    }
}
