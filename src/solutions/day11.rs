// Advent of Code 2022 - Day 11

use std::fs;

#[derive(Debug, Clone)]
struct Monkey {
    inventory: Vec<usize>,
    throw_to_if_true: usize,
    throw_to_if_false: usize,
    devisable_by: usize,
    operation: WorryOperation,
    inspected: usize,
}

#[derive(Debug, Clone)]
enum WorryOperation {
    Add(usize),
    Twice,
    Multiply(usize),
    Square,
}

impl WorryOperation {
    fn operate(&self, old: usize) -> usize {
        match self {
            WorryOperation::Add(x) => old + x,
            WorryOperation::Multiply(x) => old * x,
            WorryOperation::Square => old.pow(2),
            WorryOperation::Twice => old * 2,
        }
    }
}

enum StressManagerEnum {
    Modulo(usize),
    Divide(usize),
}

impl StressManagerEnum {
    fn operate(&self, val: usize) -> usize {
        match self {
            StressManagerEnum::Modulo(x) => val % x,
            StressManagerEnum::Divide(x) => val / x,
        }
    }
}

impl Monkey {
    fn from_string(d: &str) -> Self {
        let mut ins = d.lines().skip(1);
        let inventory = ins
            .next()
            .unwrap()
            .split(':')
            .last()
            .unwrap()
            .split(", ")
            .map(|v| v.trim())
            .filter(|v| !v.is_empty())
            .map(|v| v.parse::<usize>().unwrap())
            .collect();
        let op_split: Vec<&str> = ins
            .next()
            .unwrap()
            .split_whitespace()
            .rev()
            .take(2)
            .collect();
        let operation = match op_split.as_slice() {
            ["old", "*"] => WorryOperation::Square,
            [x, "*"] => WorryOperation::Multiply(x.parse::<usize>().unwrap()),
            ["old", "+"] => WorryOperation::Twice,
            [x, "+"] => WorryOperation::Add(x.parse::<usize>().unwrap()),
            _ => unreachable!(),
        };
        let mut next_3 = ins.take(3).map(|ins| {
            ins.split_whitespace()
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap()
        });
        let devisable_by: usize = next_3.next().unwrap();
        let throw_to_if_true: usize = next_3.next().unwrap();
        let throw_to_if_false: usize = next_3.next().unwrap();
        Self {
            inventory,
            operation,
            devisable_by,
            throw_to_if_false,
            throw_to_if_true,
            inspected: 0,
        }
    }

    fn operate(&mut self, stress_manager: &StressManagerEnum) -> Vec<(usize, usize)> {
        self.inventory
            .drain(..)
            .map(|item| {
                let new_worry_level = stress_manager.operate(self.operation.operate(item));
                self.inspected += 1;
                if new_worry_level % self.devisable_by == 0 {
                    (self.throw_to_if_true, new_worry_level)
                } else {
                    (self.throw_to_if_false, new_worry_level)
                }
            })
            .collect()
    }
}

fn run_monkey_game(
    file_path: String,
    round: usize,
    stress_divider: Option<usize>,
) -> Option<usize> {
    let mut monkeys: Vec<Monkey> = fs::read_to_string(file_path)
        .unwrap()
        .split("\n\nMonkey")
        .map(Monkey::from_string)
        .collect();
    let len = monkeys.len();
    let stress_manager = match stress_divider {
        Some(d) => StressManagerEnum::Divide(d),
        None => {
            let modulo: usize = monkeys.iter().map(|m| m.devisable_by).product();
            StressManagerEnum::Modulo(modulo)
        }
    };
    for _ in 0..round {
        for i in 0..len {
            for (next, val) in monkeys[i].operate(&stress_manager).iter() {
                monkeys[*next].inventory.push(*val);
            }
        }
    }
    let mut scores: Vec<usize> = monkeys.iter().map(|m| m.inspected).collect();
    scores.sort();
    Some(scores[len - 2] * scores[len - 1])
}

pub fn solution_day_11_01(file_path: String) -> Option<usize> {
    run_monkey_game(file_path, 20, Some(3))
}

pub fn solution_day_11_02(file_path: String) -> Option<usize> {
    run_monkey_game(file_path, 10_000, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_11_01() {
        let file_path: String = String::from("src/inputs/day11e.txt");
        let result = solution_day_11_01(file_path).unwrap();
        assert_eq!(result, 10605);
    }

    #[test]
    fn test_day_11_02() {
        let file_path: String = String::from("src/inputs/day11e.txt");
        let result = solution_day_11_02(file_path).unwrap();
        assert_eq!(result, 2713310158);
    }

    #[test]
    #[ignore]
    fn output_day_11_01() {
        let file_path: String = String::from("src/inputs/day11.txt");
        let result = solution_day_11_01(file_path);
        dbg!(result.unwrap());
        assert_eq!(1, 1);
    }

    #[test]
    #[ignore]
    fn output_day_11_02() {
        let file_path: String = String::from("src/inputs/day11.txt");
        let result = solution_day_11_02(file_path);
        dbg!(result.unwrap());
        assert_eq!(1, 1);
    }
}
