// Advent of Code 2022 - Day 06

use std::{collections::HashSet, fs};

fn detect_distinct(inp: &str, amount: usize) -> Option<usize> {
    for i in 0..inp.len() - amount {
        if amount == inp[i..i + amount].chars().collect::<HashSet<char>>().len() {
            return Some(i + amount);
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
        let result = solution_day_06_01(file_path);
        println!("{:?}", result.unwrap());
        assert_eq!(1, 1);
    }

    #[test]
    #[ignore]
    fn output_day_06_02() {
        let file_path: String = String::from("src/inputs/day06.txt");
        let result = solution_day_06_02(file_path);
        println!("{:?}", result.unwrap());
        assert_eq!(1, 1);
    }
}
