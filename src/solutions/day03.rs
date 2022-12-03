// Advent of Code 2022 - Day 03

use crate::utils::read::read_lines;
use std::collections::HashSet;

fn get_common(rucksack: &String) -> char {
    let length = rucksack.len() / 2;
    let comp_1 = &rucksack[..length];
    let comp_2: HashSet<char> = rucksack[length..].chars().collect();
    comp_1
        .chars()
        .find(|c| comp_2.contains(c))
        .expect("No common items")
}

fn get_priority(item: char) -> usize {
    let digit = item as usize;
    if digit < 91 {
        return (26 + digit - 64).try_into().unwrap();
    } else {
        return (digit - 96).try_into().unwrap();
    }
}

fn get_badge(rucksacks: &[String]) -> char {
    let mut rucksacks: Vec<String> = rucksacks.clone().into();
    rucksacks.sort_by(|a, b| a.len().cmp(&b.len()));
    let sack_2: HashSet<char> = rucksacks[1].chars().collect();
    let sack_3: HashSet<char> = rucksacks[2].chars().collect();
    rucksacks[0]
        .chars()
        .find(|r| sack_2.contains(r) && sack_3.contains(r))
        .expect("No common items")
}

fn get_total_priorities(input: Vec<String>) -> usize {
    input.iter().map(|r| get_priority(get_common(r))).sum()
}

fn get_total_badge_priorities(input: Vec<String>) -> usize {
    input
        .chunks(3)
        .map(|rss| get_priority(get_badge(rss)))
        .sum()
}

pub fn solution_day_03_01(file_path: String) -> usize {
    let input = read_lines(file_path);
    get_total_priorities(input)
}

pub fn solution_day_03_02(file_path: String) -> usize {
    let input = read_lines(file_path);
    get_total_badge_priorities(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_03_01() {
        let file_path: String = String::from("src/inputs/day03e.txt");
        let result = solution_day_03_01(file_path);
        assert_eq!(result, 157);
    }

    #[test]
    fn test_day_01_02() {
        let file_path: String = String::from("src/inputs/day03e.txt");
        let result = solution_day_03_02(file_path);
        assert_eq!(result, 70);
    }

    #[test]
    #[ignore]
    fn output_day_03_01() {
        let file_path: String = String::from("src/inputs/day03.txt");
        let result = solution_day_03_01(file_path);
        println!("{:?}", result);
        assert_eq!(1, 1);
    }

    #[test]
    #[ignore]
    fn output_day_01_02() {
        let file_path: String = String::from("src/inputs/day03.txt");
        let result = solution_day_03_02(file_path);
        println!("{:?}", result);
        assert_eq!(1, 1);
    }
}
