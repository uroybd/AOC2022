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

fn update_size(
    dir_map: &mut HashMap<String, Directory>,
    current_dir: &Option<String>,
    size_cache: usize,
) -> usize {
    if size_cache > 0 {
        let d = dir_map.get_mut(current_dir.as_ref().unwrap()).unwrap();
        d.size += size_cache;
        let mut dp = d.parent.clone();
        while let Some(dp_key) = dp {
            let dpt = dir_map.get_mut(&dp_key).unwrap();
            dpt.size += size_cache;
            dp = dpt.parent.clone();
        }
    }
    0
}

fn run_commands(input: String) -> HashMap<String, Directory> {
    let mut current_dir: Option<String> = None;
    let mut dir_map: HashMap<String, Directory> = HashMap::new();
    dir_map.insert("/".to_string(), Directory::new_with_parent(None));
    let mut size_cache = 0;
    for line in input.lines() {
        if line.starts_with('$') {
            let mut parts = line.split(' ').skip(1);
            if parts.next().unwrap() == "cd" {
                size_cache = update_size(&mut dir_map, &current_dir, size_cache);
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
                        let mut key = pwd.to_string();
                        if current_dir.is_some() {
                            key = format!("{}{}/", current_dir.as_ref().unwrap(), pwd);
                        }
                        current_dir = Some(key);
                    }
                }
            }
        } else {
            let mut parts = line.split(' ');
            match parts.next().unwrap() {
                "dir" => {
                    let dir = parts.next().unwrap();
                    let key = format!("{}{}/", current_dir.as_ref().unwrap(), dir);
                    dir_map
                        .entry(key)
                        .or_insert_with(|| Directory::new_with_parent(current_dir.clone()));
                }
                size => {
                    let s = size.parse::<usize>().unwrap();
                    size_cache += s;
                }
            }
        }
    }
    update_size(&mut dir_map, &current_dir, size_cache);
    dir_map
}

fn get_all_dir_sizes(dir_map: &HashMap<String, Directory>) -> Vec<usize> {
    dir_map.values().map(|dir| dir.size).collect()
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
    sizes.retain(|a| a >= &to_be_reclaimed);
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
