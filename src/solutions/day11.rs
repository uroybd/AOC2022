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
    AddSelf,
    Multiply(usize),
    MultiplySelf,
}

impl WorryOperation {
    fn operate(&self, old: usize) -> usize {
        match self {
            WorryOperation::Add(x) => old + x,
            WorryOperation::Multiply(x) => old * x,
            WorryOperation::AddSelf => old + old,
            WorryOperation::MultiplySelf => old * old,
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
            ["old", "*"] => WorryOperation::MultiplySelf,
            [x, "*"] => WorryOperation::Multiply(x.parse::<usize>().unwrap()),
            ["old", "+"] => WorryOperation::AddSelf,
            [x, "+"] => WorryOperation::Add(x.parse::<usize>().unwrap()),
            _ => unreachable!(),
        };
        let devisable_by: usize = ins
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let throw_to_if_true: usize = ins
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let throw_to_if_false: usize = ins
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        Self {
            inventory,
            operation,
            devisable_by,
            throw_to_if_false,
            throw_to_if_true,
            inspected: 0,
        }
    }

    fn operate(&self, div_func: impl Fn(usize) -> usize) -> Vec<(usize, usize)> {
        self.inventory
            .iter()
            .map(|item| {
                let new_worry_level = div_func(self.operation.operate(*item));
                if new_worry_level % self.devisable_by == 0 {
                    (self.throw_to_if_true, new_worry_level)
                } else {
                    (self.throw_to_if_false, new_worry_level)
                }
            })
            .collect()
    }
}

pub fn solution_day_11_01(file_path: String) -> Option<usize> {
    let mut monkeys: Vec<Monkey> = fs::read_to_string(file_path)
        .unwrap()
        .split("\n\nMonkey")
        .map(Monkey::from_string)
        .collect();
    let len = monkeys.len();
    for _ in 0..20 {
        for i in 0..len {
            for (next, val) in monkeys[i].clone().operate(|x| x / 3).iter() {
                monkeys[*next].inventory.push(*val);
            }
            monkeys[i].inspected += monkeys[i].inventory.len();
            monkeys[i].inventory.clear();
        }
    }
    let mut scores: Vec<usize> = monkeys.iter().map(|m| m.inspected).collect();
    scores.sort();
    Some(scores[len - 2] * scores[len - 1])
}

pub fn solution_day_11_02(file_path: String) -> Option<usize> {
    let mut monkeys: Vec<Monkey> = fs::read_to_string(file_path)
        .unwrap()
        .split("\n\nMonkey")
        .map(Monkey::from_string)
        .collect();
    let modulo: usize = monkeys.iter().map(|m| m.devisable_by).product();

    let len = monkeys.len();
    for _ in 0..10000 {
        for i in 0..len {
            for (next, val) in monkeys[i].clone().operate(|x| x % modulo).iter() {
                monkeys[*next].inventory.push(*val);
            }
            monkeys[i].inspected += monkeys[i].inventory.len();
            monkeys[i].inventory.clear();
        }
    }
    let mut scores: Vec<usize> = monkeys.iter().map(|m| m.inspected).collect();
    scores.sort();
    Some(scores[len - 2] * scores[len - 1])
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
