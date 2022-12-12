// Advent of Code 2022 - Day 08

use std::fs;

use crate::utils::collections::Faux2DArray;

fn get_visible(trees: &Faux2DArray<usize>) -> usize {
    let (width, height) = (trees.width, trees.height());
    let mut total = (height * 2) + ((width - 2) * 2);
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let current_tree = trees.at(x, y);
            if !trees.to_row_start(x, y).unwrap().any(|t| t >= current_tree)
                || !trees.to_row_end(x, y).unwrap().any(|t| t >= current_tree)
                || !trees.to_col_start(x, y).unwrap().any(|t| t >= current_tree)
                || !trees.to_col_end(x, y).unwrap().any(|t| t >= current_tree)
            {
                total += 1;
            }
        }
    }
    total
}

fn largest_scenic_score(trees: &Faux2DArray<usize>) -> usize {
    let (width, height) = (trees.width, trees.height());
    let mut largest = 0;
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let current_tree = trees.at(x, y);
            let score_left = match trees
                .to_row_start(x, y)
                .unwrap()
                .enumerate()
                .find(|(_, t)| t >= &current_tree)
            {
                Some((pos, _)) => pos + 1,
                None => x,
            };
            if score_left == 0 {
                continue;
            }
            let score_right = match trees
                .to_row_end(x, y)
                .unwrap()
                .enumerate()
                .find(|(_, t)| t >= &current_tree)
            {
                Some((pos, _)) => pos + 1,
                None => width - x - 1,
            };
            if score_right == 0 {
                continue;
            }
            let score_top = match trees
                .to_col_start(x, y)
                .unwrap()
                .enumerate()
                .find(|(_, t)| t >= &current_tree)
            {
                Some((pos, _)) => pos + 1,
                None => y,
            };
            if score_top == 0 {
                continue;
            }
            let score_bottom = match trees
                .to_col_end(x, y)
                .unwrap()
                .enumerate()
                .find(|(_, t)| t >= &current_tree)
            {
                Some((pos, _)) => pos + 1,
                None => height - y - 1,
            };
            if score_bottom == 0 {
                continue;
            }
            let score = score_left * score_right * score_top * score_bottom;
            if score > largest {
                largest = score
            }
        }
    }
    largest
}

fn parse_input(file_path: String) -> Faux2DArray<usize> {
    let mut trees = Faux2DArray::new(5);
    fs::read_to_string(file_path)
        .unwrap()
        .trim()
        .lines()
        .for_each(|l| {
            let row: Vec<usize> = l
                .chars()
                .map(|s| s.to_string().parse::<usize>().unwrap())
                .collect();
            trees.width = row.len();
            trees.add_row(row).unwrap();
        });
    trees
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
