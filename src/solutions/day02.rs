// Advent of Code 2022 - Day 02

use crate::utils::read::read_lines;

fn score_from_string(s: &str) -> usize {
    match s {
        "A" | "X" => 1,
        "B" | "Y" => 2,
        "C" | "Z" => 3,
        _ => unreachable!(),
    }
}

fn get_score(opponent: usize, me: usize) -> usize {
    if me == opponent {
        return 3 + me;
    }
    let winning_move = (opponent % 3) + 1;
    if me == winning_move {
        6 + me
    } else {
        me
    }
}

fn get_game_score(rounds: &[String]) -> usize {
    rounds
        .iter()
        .map(|r| {
            let entries: Vec<&str> = r.split(' ').collect();
            get_score(score_from_string(entries[0]), score_from_string(entries[1]))
        })
        .sum()
}

fn get_score_following_strategy(opponent: usize, strategy: &str) -> usize {
    match strategy {
        "X" => ((opponent + 1) % 3) + 1,
        "Y" => 3 + opponent,
        "Z" => 6 + (opponent % 3) + 1,
        _ => unreachable!(),
    }
}

fn get_game_score_following_strategy(rounds: &[String]) -> usize {
    rounds
        .iter()
        .map(|r| {
            let entries: Vec<&str> = r.split(' ').collect();
            get_score_following_strategy(score_from_string(entries[0]), entries[1])
        })
        .sum()
}

pub fn solution_day_02_01(file_path: String) -> Option<usize> {
    let input = read_lines(file_path);
    Some(get_game_score(&input))
}

pub fn solution_day_02_02(file_path: String) -> Option<usize> {
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
    // #[ignore]
    fn output_day_02_01() {
        let file_path: String = String::from("src/inputs/day02.txt");

        let result = solution_day_02_01(file_path);

        dbg!(result.unwrap());
        assert_eq!(1, 1);
    }

    #[test]
    // #[ignore]
    fn output_day_02_02() {
        let file_path: String = String::from("src/inputs/day02.txt");

        let result = solution_day_02_02(file_path);

        dbg!(result.unwrap());
        assert_eq!(1, 1);
    }
}
