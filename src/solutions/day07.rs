// Advent of Code 2022 - Day 07

use std::{collections::HashMap, fs};

#[derive(Debug)]
pub struct Directory {
    parent: Option<String>,
    size: usize,
}

impl Directory {
    fn new_with_parent(p: &Option<String>) -> Self {
        Self {
            parent: p.clone(),
            size: 0,
        }
    }
}

fn update_size(
    dir_map: &mut HashMap<String, Directory>,
    current_dir: &Option<String>,
    size_cache: usize,
) -> usize {
    if size_cache > 0 {
        let mut current_dir = current_dir.clone();
        while let Some(dir_key) = current_dir {
            let dir = dir_map.get_mut(&dir_key).unwrap();
            dir.size += size_cache;
            current_dir = dir.parent.clone();
        }
    }
    0
}

fn run_commands(input: String) -> HashMap<String, Directory> {
    let mut current_dir: Option<String> = None;
    let mut dir_map: HashMap<String, Directory> = HashMap::new();
    dir_map.insert("/".to_string(), Directory::new_with_parent(&None));
    let mut size_cache = 0;
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts.as_slice() {
            ["$", "cd", ".."] => {
                size_cache = update_size(&mut dir_map, &current_dir, size_cache);
                if current_dir.is_some() {
                    current_dir = dir_map.get(&current_dir.unwrap()).unwrap().parent.clone();
                }
            }
            ["$", "cd", pwd] => {
                size_cache = update_size(&mut dir_map, &current_dir, size_cache);
                current_dir = Some(match current_dir {
                    Some(cur_dir) => format!("{}{}/", cur_dir, pwd),
                    None => pwd.to_string(),
                });
            }
            ["dir", dir] => {
                let key = format!("{}{}/", current_dir.as_ref().unwrap(), dir);
                dir_map
                    .entry(key)
                    .or_insert_with(|| Directory::new_with_parent(&current_dir));
            }
            ["$", "ls"] => continue,
            [size, _] => size_cache += size.parse::<usize>().unwrap(),
            _ => unreachable!(),
        };
    }
    update_size(&mut dir_map, &current_dir, size_cache);
    dir_map
}

fn get_all_dir_sizes(dir_map: &HashMap<String, Directory>) -> Vec<usize> {
    dir_map.values().map(|dir| dir.size).collect()
}

pub fn solution_day_07_01(file_path: String) -> Option<usize> {
    let input = fs::read_to_string(file_path).unwrap().trim().to_string();
    Some(
        get_all_dir_sizes(&run_commands(input))
            .iter()
            .filter(|s| s <= &&100000)
            .sum(),
    )
}

pub fn solution_day_07_02(file_path: String) -> Option<usize> {
    let input = fs::read_to_string(file_path).unwrap().trim().to_string();
    let mut sizes = get_all_dir_sizes(&run_commands(input));
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
