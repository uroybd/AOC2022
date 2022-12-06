// Advent of Code 2022 - Day 01

use std::fs;

fn get_calorie_counts(input: String) -> Vec<usize> {
    let mut calories: Vec<usize> = input
        .trim()
        .split("\n\n")
        .map(|inv| {
            inv.split('\n')
                .map(|item| item.parse::<usize>().unwrap())
                .sum()
        })
        .collect();
    calories.sort();
    calories.reverse();
    calories
}

fn get_top_n(calories: &[usize], count: usize) -> usize {
    return calories.iter().take(count).sum();
}

pub fn solution_day_01_01(file_path: String) -> Option<usize> {
    let input = get_calorie_counts(fs::read_to_string(file_path).unwrap());
    Some(get_top_n(&input, 1))
}

pub fn solution_day_01_02(file_path: String) -> Option<usize> {
    let input = get_calorie_counts(fs::read_to_string(file_path).unwrap());
    Some(get_top_n(&input, 3))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_01_01() {
        let file_path: String = String::from("src/inputs/day01e.txt");
        let result = solution_day_01_01(file_path).unwrap();
        assert_eq!(result, 24000);
    }

    #[test]
    fn test_day_01_02() {
        let file_path: String = String::from("src/inputs/day01e.txt");
        let result = solution_day_01_02(file_path).unwrap();
        assert_eq!(result, 45000);
    }

    #[test]
    fn output_day_01_01() {
        let file_path: String = String::from("src/inputs/day01.txt");

        let result = solution_day_01_01(file_path);

        println!("{:?}", result.unwrap());
        assert_eq!(1, 1);
    }

    #[test]
    fn output_day_01_02() {
        let file_path: String = String::from("src/inputs/day01.txt");

        let result = solution_day_01_02(file_path);

        println!("{:?}", result.unwrap());
        assert_eq!(1, 1);
    }
}
