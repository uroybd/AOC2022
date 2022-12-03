// Advent of Code 2022 - Day 01

use std::fs;

fn get_calorie_counts(input: String) -> Vec<u32> {
    let mut calories: Vec<u32> = input
        .trim()
        .split("\n\n")
        .map(|inv| {
            inv.split("\n")
                .map(|item| item.parse::<u32>().unwrap())
                .sum()
        })
        .collect();
    calories.sort();
    calories.reverse();
    calories
}

fn get_top_n(calories: &Vec<u32>, count: usize) -> u32 {
    return calories.iter().take(count).sum();
}

pub fn solution_day_01_01(file_path: String) -> Option<u32> {
    let input = get_calorie_counts(fs::read_to_string(file_path).unwrap());
    Some(get_top_n(&input, 1))
}

pub fn solution_day_01_02(file_path: String) -> Option<u32> {
    let input = get_calorie_counts(fs::read_to_string(file_path).unwrap());
    Some(get_top_n(&input, 3))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_01_01() {
        let file_path: String = String::from("src/inputs/day01e.txt");
        let result = solution_day_01_01(file_path);
        assert_eq!(result.unwrap(), 24000);
    }

    #[test]
    fn test_day_01_02() {
        let file_path: String = String::from("src/inputs/day01e.txt");
        let result = solution_day_01_02(file_path);
        assert_eq!(result.unwrap(), 45000);
    }

    #[test]
    #[ignore]
    fn output_day_01_01() {
        let file_path: String = String::from("src/inputs/day01.txt");
        let start = std::time::Instant::now();
        let result = solution_day_01_01(file_path);
        eprintln!("elapsed {:?}", start.elapsed().as_secs_f64());
        println!("{:?}", result.unwrap());
        assert_eq!(1, 1);
    }

    #[test]
    #[ignore]
    fn output_day_01_02() {
        let file_path: String = String::from("src/inputs/day01.txt");
        let start = std::time::Instant::now();
        let result = solution_day_01_02(file_path);
        eprintln!("elapsed {:?}", start.elapsed().as_secs_f64());
        println!("{:?}", result.unwrap());
        assert_eq!(1, 1);
    }
}
