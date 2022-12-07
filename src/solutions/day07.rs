// Advent of Code 2022 - Day 07

use std::{collections::HashMap, fs};

#[derive(Debug)]
pub struct Directory {
    parent: Option<String>,
    size: usize,
}

impl Directory {
    fn new_with_parent(p: Option<String>) -> Self {
        Self { parent: p, size: 0 }
    }
}

fn run_commands(input: String) -> HashMap<String, Directory> {
    let mut current_dir: Option<String> = None;
    let mut dir_map: HashMap<String, Directory> = HashMap::new();
    for line in input.lines() {
        if line.starts_with('$') {
            let mut parts = line.split(' ').skip(1);
            if parts.next().unwrap() == "cd" {
                let pwd = parts.next().unwrap();
                match pwd {
                    ".." => {
                        let mut new_current_dir = None;
                        if current_dir.is_some() {
                            let cur = current_dir.clone().unwrap();
                            new_current_dir = dir_map.get(&cur).unwrap().parent.clone();
                        }
                        current_dir = new_current_dir;
                    }
                    _ => {
                        // dbg!(current_dir.clone());
                        let mut key = pwd.to_string();
                        if current_dir.is_some() {
                            key = format!("{}/{}", current_dir.as_ref().unwrap(), pwd);
                        }
                        dir_map
                            .entry(key.clone())
                            .or_insert_with(|| Directory::new_with_parent(current_dir));
                        current_dir = Some(key);
                    }
                }
            }
        } else {
            let mut parts = line.split(' ');
            match parts.next().unwrap() {
                "dir" => {
                    let dir = parts.next().unwrap();
                    let key = format!("{}/{}", current_dir.as_ref().unwrap(), dir);
                    dir_map
                        .entry(key)
                        .or_insert_with(|| Directory::new_with_parent(current_dir.clone()));
                }
                size => {
                    let d = dir_map.get_mut(current_dir.as_ref().unwrap()).unwrap();
                    d.size += size.parse::<usize>().unwrap()
                }
            }
        }
    }
    dir_map
}

fn calculate_total_size(dir_map: &HashMap<String, Directory>, dirname: String) -> Option<usize> {
    dir_map.get(&dirname).map(|dir| {
        dir.size
            + dir_map
                .iter()
                .filter(|(_, d)| d.parent == Some(dirname.clone()))
                .map(|(name, _)| calculate_total_size(dir_map, name.to_string()))
                .fold(0, |acc, v| match v {
                    Some(v) => v + acc,
                    None => acc,
                })
    })
}

fn get_all_dir_sizes(dir_map: &HashMap<String, Directory>) -> Vec<usize> {
    dir_map
        .keys()
        .filter_map(|key| calculate_total_size(dir_map, key.to_string()))
        .collect()
}

pub fn solution_day_07_01(file_path: String) -> Option<usize> {
    let input = fs::read_to_string(file_path).unwrap().trim().to_string();
    let dir_map = run_commands(input);
    Some(
        get_all_dir_sizes(&dir_map)
            .iter()
            .filter(|s| s <= &&100000)
            .sum(),
    )
}

pub fn solution_day_07_02(file_path: String) -> Option<usize> {
    let input = fs::read_to_string(file_path).unwrap().trim().to_string();
    let dir_map = run_commands(input);
    let mut sizes = get_all_dir_sizes(&dir_map);
    sizes.sort();
    let (total_size, required_space, used) = (70000000, 30000000, sizes.last().unwrap());
    let to_be_reclaimed = required_space - (total_size - used);
    sizes.retain(|a| a >= &&to_be_reclaimed);
    Some(*sizes.first().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_07_01() {
        let file_path: String = String::from("src/inputs/day07e.txt");
        let result = solution_day_07_01(file_path).unwrap();
        assert_eq!(result, 95437);
    }

    #[test]
    fn test_day_07_02() {
        let file_path: String = String::from("src/inputs/day07e.txt");
        let result = solution_day_07_02(file_path).unwrap();
        assert_eq!(result, 24933642);
    }

    #[test]
    #[ignore]
    fn output_day_07_01() {
        let file_path: String = String::from("src/inputs/day07.txt");
        let result = solution_day_07_01(file_path);
        dbg!(result.unwrap());
        assert_eq!(1, 1);
    }

    #[test]
    #[ignore]
    fn output_day_07_02() {
        let file_path: String = String::from("src/inputs/day07.txt");
        let result = solution_day_07_02(file_path);
        dbg!(result.unwrap());
        assert_eq!(1, 1);
    }
}
