// Advent of Code 2022 - Day 02

use crate::utils::read::read_lines;

fn score_from_string(s: &str) -> u32 {
    match s {
        "A" | "X" => 1,
        "B" | "Y" => 2,
        _ => 3,
    }
}

fn get_score(opponent: u32, me: u32) -> u32 {
    if me == opponent {
        return 3 + me;
    }
    let winning_move = (opponent % 3) + 1;
    if me == winning_move {
        return 6 + me;
    } else {
        return me;
    }
}

fn get_game_score(rounds: &Vec<String>) -> u32 {
    rounds
        .iter()
        .map(|r| {
            let entries: Vec<&str> = r.split(" ").collect();
            get_score(score_from_string(entries[0]), score_from_string(entries[1]))
        })
        .sum()
}

fn get_score_following_strategy(opponent: u32, strategy: &str) -> u32 {
    return match strategy {
        "X" => ((opponent + 1) % 3) + 1,
        "Y" => 3 + opponent,
        _ => 6 + (opponent % 3) + 1,
    };
}

fn get_game_score_following_strategy(rounds: &Vec<String>) -> u32 {
    rounds
        .iter()
        .map(|r| {
            let entries: Vec<&str> = r.split(" ").collect();
            get_score_following_strategy(score_from_string(entries[0]), entries[1])
        })
        .sum()
}

pub fn solution_day_02_01(file_path: String) -> Option<u32> {
    let input = read_lines(file_path);
    Some(get_game_score(&input))
}

pub fn solution_day_02_02(file_path: String) -> Option<u32> {
    let input = read_lines(file_path);
    Some(get_game_score_following_strategy(&input))
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
        let start = std::time::Instant::now();
        let result = solution_day_02_01(file_path);
        eprintln!("elapsed {:?}", start.elapsed().as_secs_f64());
        println!("{:?}", result.unwrap());
        assert_eq!(1, 1);
    }

    #[test]
    #[ignore]
    fn output_day_02_02() {
        let file_path: String = String::from("src/inputs/day02.txt");
        let start = std::time::Instant::now();
        let result = solution_day_02_02(file_path);
        eprintln!("elapsed {:?}", start.elapsed().as_secs_f64());
        println!("{:?}", result.unwrap());
        assert_eq!(1, 1);
    }
}
