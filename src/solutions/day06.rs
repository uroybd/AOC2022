// Advent of Code 2022 - Day 06

use std::{collections::HashSet, fs};

fn detect_distinct(inp: &str, amount: usize) -> Option<usize> {
    let data: Vec<char> = inp.chars().collect();
    for index in 0..data.len() - amount {
        if amount
            == data[index..index + amount]
                .iter()
                .collect::<HashSet<&char>>()
                .len()
        {
            return Some(index + amount);
        }
    }
    None
}

pub fn solution_day_06_01(file_path: String) -> Option<usize> {
    detect_distinct(fs::read_to_string(file_path).unwrap().trim(), 4)
}

pub fn solution_day_06_02(file_path: String) -> Option<usize> {
    detect_distinct(fs::read_to_string(file_path).unwrap().trim(), 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_distinct() {
        // let file_path: String = String::from("src/inputs/day06e.txt");
        // let result = solution_day_06_01(file_path).unwrap();
        let inp1 = "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string();
        let inp2 = "nppdvjthqldpwncqszvftbrmjlhg".to_string();
        let inp3 = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string();
        let inp4 = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string();
        assert_eq!(detect_distinct(&inp1, 4), Some(5));
        assert_eq!(detect_distinct(&inp2, 4), Some(6));
        assert_eq!(detect_distinct(&inp3, 4), Some(10));
        assert_eq!(detect_distinct(&inp4, 4), Some(11));
    }

    #[test]
    #[ignore]
    fn output_day_06_01() {
        let file_path: String = String::from("src/inputs/day06.txt");
        let start = std::time::Instant::now();
        let result = solution_day_06_01(file_path);
        eprintln!("elapsed {:?}", start.elapsed().as_secs_f64());
        println!("{:?}", result.unwrap());
        assert_eq!(1, 1);
    }

    #[test]
    #[ignore]
    fn output_day_06_02() {
        let file_path: String = String::from("src/inputs/day06.txt");
        let start = std::time::Instant::now();
        let result = solution_day_06_02(file_path);
        eprintln!("elapsed {:?}", start.elapsed().as_secs_f64());
        println!("{:?}", result.unwrap());
        assert_eq!(1, 1);
    }
}