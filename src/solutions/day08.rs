// Advent of Code 2022 - Day 08

use std::fs;

fn get_visible(trees: &Vec<Vec<usize>>) -> usize {
    let (width, height) = (trees[0].len(), trees.len());
    let mut total = (height * 2) + ((width - 2) * 2);
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let current_tree = trees[y][x];
            if !trees[y][0..x].iter().any(|t| t >= &current_tree)
                || !trees[y][(x + 1)..].iter().any(|t| t >= &current_tree)
                || !(0..y).any(|cy| trees[cy][x] >= current_tree)
                || !((y + 1)..height).any(|cy| trees[cy][x] >= current_tree)
            {
                total += 1;
            }
        }
    }
    total
}

fn largest_scenic_score(trees: &Vec<Vec<usize>>) -> usize {
    let (width, height) = (trees[0].len(), trees.len());
    let mut largest = 0;
    for y in 0..height {
        for x in 0..width {
            let current_tree = trees[y][x];
            let score_left = match trees[y][0..x]
                .iter()
                .rev()
                .enumerate()
                .find(|(_, t)| t >= &&current_tree)
            {
                Some((pos, _)) => pos + 1,
                None => x,
            };
            let score_right = match trees[y][(x + 1)..]
                .iter()
                .enumerate()
                .find(|(_, t)| t >= &&current_tree)
            {
                Some((pos, _)) => pos + 1,
                None => width - x - 1,
            };
            let score_top = match (0..y)
                .rev()
                .enumerate()
                .find(|(_, cy)| trees[*cy][x] >= current_tree)
            {
                Some((pos, _)) => pos + 1,
                None => y,
            };
            let score_bottom = match ((y + 1)..height)
                .enumerate()
                .find(|(_, cy)| trees[*cy][x] >= current_tree)
            {
                Some((pos, _)) => pos + 1,
                None => height - y - 1,
            };
            let score = score_left * score_right * score_top * score_bottom;
            if score > largest {
                largest = score
            }
        }
    }
    largest
}

fn parse_input(file_path: String) -> Vec<Vec<usize>> {
    fs::read_to_string(file_path)
        .unwrap()
        .trim()
        .lines()
        .map(|l| {
            l.chars()
                .map(|s| s.to_string().parse::<usize>().unwrap())
                .collect()
        })
        .collect()
}

pub fn solution_day_08_01(file_path: String) -> Option<usize> {
    let trees = parse_input(file_path);
    Some(get_visible(&trees))
}

pub fn solution_day_08_02(file_path: String) -> Option<usize> {
    let trees = parse_input(file_path);
    Some(largest_scenic_score(&trees))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_08_01() {
        let file_path: String = String::from("src/inputs/day08e.txt");
        let result = solution_day_08_01(file_path).unwrap();
        assert_eq!(result, 21);
    }

    #[test]
    fn test_day_08_02() {
        let file_path: String = String::from("src/inputs/day08ee.txt");
        let result = solution_day_08_02(file_path).unwrap();
        assert_eq!(result, 8);
    }

    #[test]
    #[ignore]
    fn output_day_08_01() {
        let file_path: String = String::from("src/inputs/day08.txt");
        let result = solution_day_08_01(file_path);
        dbg!(result.unwrap());
        assert_eq!(1, 1);
    }

    #[test]
    #[ignore]
    fn output_day_08_02() {
        let file_path: String = String::from("src/inputs/day08.txt");
        let result = solution_day_08_02(file_path);
        dbg!(result.unwrap());
        assert_eq!(1, 1);
    }
}
