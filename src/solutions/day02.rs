// Advent of Code 2022 - Day 02

use std::fs;

fn score_from_string(s: &str) -> usize {
    match s {
        "A" | "X" => 1,
        "B" | "Y" => 2,
        "C" | "Z" => 3,
        _ => unreachable!(),
    }
}

fn get_score(opponent: usize, me: usize) -> usize {
    me + if me == opponent {
        3
    } else if me == (opponent % 3) + 1 {
        6
    } else {
        0
    }
}

fn get_score_following_strategy(opponent: usize, strategy: &str) -> usize {
    match strategy {
        "X" => ((opponent + 1) % 3) + 1,
        "Y" => 3 + opponent,
        "Z" => 6 + (opponent % 3) + 1,
        _ => unreachable!(),
    }
}

pub fn solution_day_02_01(file_path: String) -> Option<usize> {
    let result = fs::read_to_string(file_path)
        .expect("File not found")
        .lines()
        .map(|r| {
            let mut entries = r.split(' ');
            get_score(
                score_from_string(entries.next().unwrap()),
                score_from_string(entries.next().unwrap()),
            )
        })
        .sum();
    Some(result)
}

pub fn solution_day_02_02(file_path: String) -> Option<usize> {
    let result = fs::read_to_string(file_path)
        .expect("File not found")
        .lines()
        .map(|r| {
            let mut entries = r.split(' ');
            get_score_following_strategy(
                score_from_string(entries.next().unwrap()),
                entries.next().unwrap(),
            )
        })
        .sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_02_01() {
        let file_path: String = String::from("src/inputs/day02e.txt");
        let result = solution_day_02_01(file_path).unwrap();
        assert_eq!(result, 15);
    }

    #[test]
    fn test_day_02_02() {
        let file_path: String = String::from("src/inputs/day02e.txt");
        let result = solution_day_02_02(file_path).unwrap();
        assert_eq!(result, 12);
    }

    #[test]
    #[ignore]
    fn output_day_02_01() {
        let file_path: String = String::from("src/inputs/day02.txt");

        let result = solution_day_02_01(file_path);

        dbg!(result.unwrap());
        assert_eq!(1, 1);
    }

    #[test]
    #[ignore]
    fn output_day_02_02() {
        let file_path: String = String::from("src/inputs/day02.txt");

        let result = solution_day_02_02(file_path);

        dbg!(result.unwrap());
        assert_eq!(1, 1);
    }
}
