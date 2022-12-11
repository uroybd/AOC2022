// Advent of Code 2022 - Day 10

use crate::utils::read::read_lines;

pub struct ClockCircuit {
    cycle: usize,
    value: isize,
    strength: isize,
    image: Vec<String>,
}

impl ClockCircuit {
    fn new() -> Self {
        Self {
            cycle: 0,
            value: 1,
            strength: 0,
            image: vec![".".to_string(); 240],
        }
    }

    fn get_image(&self) -> String {
        self.image
            .chunks(40)
            .map(|line| format!("{}\n", line.join("")))
            .collect()
    }

    fn cycle_count(&mut self) {
        if (self.value - ((self.cycle % 40) as isize)).abs() == 1 {
            self.image[self.cycle] = "#".to_string();
        }

        self.cycle += 1;
        if self.cycle % 40 == 20 {
            self.strength += (self.cycle as isize) * self.value;
        }
    }

    fn execute(&mut self, instruction: &str) {
        if instruction == "noop" {
            self.cycle_count();
        } else {
            self.cycle_count();
            self.cycle_count();
            let amount: isize = instruction
                .split_whitespace()
                .nth(1)
                .unwrap()
                .parse()
                .unwrap();
            self.value += amount;
        }
    }
}

pub fn solution_day_10_01(file_path: String) -> Option<isize> {
    let mut circuit = ClockCircuit::new();
    read_lines(file_path)
        .iter()
        .for_each(|ins| circuit.execute(ins));
    Some(circuit.strength)
}

pub fn solution_day_10_02(file_path: String) -> Option<String> {
    let mut circuit = ClockCircuit::new();
    read_lines(file_path)
        .iter()
        .for_each(|ins| circuit.execute(ins));
    Some(circuit.get_image())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_10_01() {
        let file_path: String = String::from("src/inputs/day10e.txt");
        let result = solution_day_10_01(file_path).unwrap();
        assert_eq!(result, 13140);
    }

    #[test]
    fn test_day_10_02() {
        let file_path: String = String::from("src/inputs/day10e.txt");
        let result = solution_day_10_02(file_path).unwrap();
        println!("{}", result);
        let x = !result.is_empty();
        assert!(x, "Result is None!");
    }

    #[test]
    #[ignore]
    fn output_day_10_01() {
        let file_path: String = String::from("src/inputs/day10.txt");
        let result = solution_day_10_01(file_path);
        assert_eq!(1, 1);
    }

    #[test]
    #[ignore]
    fn output_day_10_02() {
        let file_path: String = String::from("src/inputs/day10.txt");
        let result = solution_day_10_02(file_path).unwrap();
        println!("{}", result);
        let x = !result.is_empty();
        assert!(x, "Result is None!");
    }
}
