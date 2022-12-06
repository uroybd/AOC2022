// Advent of Code 2022 - Day 05

use std::fs;

#[derive(Debug)]
struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

fn execute_instructions(stacks: &mut [Vec<char>], instructions: &[Instruction]) {
    instructions.iter().for_each(|ins| {
        let v = stacks[ins.from - 1]
            .drain(0..ins.amount)
            .as_slice()
            .to_owned();
        stacks[ins.to - 1].splice(0..0, v.into_iter().rev());
    });
}

fn execute_instructions_by_new_crane(stacks: &mut [Vec<char>], instructions: &[Instruction]) {
    instructions.iter().for_each(|ins| {
        let v = stacks[ins.from - 1]
            .drain(0..ins.amount)
            .as_slice()
            .to_owned();
        stacks[ins.to - 1].splice(0..0, v.into_iter());
    });
}

fn get_top(stacks: &[Vec<char>]) -> String {
    return stacks.iter().map(|s| s.first().unwrap()).collect();
}

fn parse_input(inp: &str) -> (Vec<Vec<char>>, Vec<Instruction>) {
    let mut parts = inp.split("\n\n");
    let stacks_data: Vec<&str> = parts.next().unwrap().lines().collect();
    let (columns, stacks_data) = stacks_data.split_last().unwrap();
    let columns_count = columns.split("   ").count();
    let mut stacks = vec![vec![]; columns_count];
    for line in stacks_data {
        for (idx, item) in line[1..].chars().step_by(4).enumerate() {
            if item != ' ' {
                stacks[idx].push(item)
            }
        }
    }

    let instructions: Vec<Instruction> = parts
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            let parts: Vec<&str> = l.split(' ').collect();
            Instruction {
                amount: parts[1].parse::<usize>().unwrap(),
                from: parts[3].parse::<usize>().unwrap(),
                to: parts[5].parse::<usize>().unwrap(),
            }
        })
        .collect();
    (stacks, instructions)
}

pub fn solution_day_05_01(file_path: String) -> Option<String> {
    let input = fs::read_to_string(file_path).unwrap();
    let (mut stacks, instructions) = parse_input(input.as_str());
    execute_instructions(&mut stacks, &instructions);
    Some(get_top(&stacks))
}

pub fn solution_day_05_02(file_path: String) -> Option<String> {
    let input = fs::read_to_string(file_path).unwrap();
    let (mut stacks, instructions) = parse_input(input.as_str());
    execute_instructions_by_new_crane(&mut stacks, &instructions);
    Some(get_top(&stacks))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_05_01() {
        let file_path: String = String::from("src/inputs/day05e.txt");
        let result = solution_day_05_01(file_path).unwrap();
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn test_day_05_02() {
        let file_path: String = String::from("src/inputs/day05e.txt");
        let result = solution_day_05_02(file_path).unwrap();
        assert_eq!(result, "MCD");
    }

    #[test]
    // #[ignore]
    fn output_day_05_01() {
        let file_path: String = String::from("src/inputs/day05.txt");

        let result = solution_day_05_01(file_path);

        dbg!(result.unwrap());
        assert_eq!(1, 1);
    }

    #[test]
    // #[ignore]
    fn output_day_05_02() {
        let file_path: String = String::from("src/inputs/day05.txt");

        let result = solution_day_05_02(file_path);

        dbg!(result.unwrap());
        assert_eq!(1, 1);
    }
}
