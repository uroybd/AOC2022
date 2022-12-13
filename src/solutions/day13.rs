use std::{cmp::Ordering, fs, str::FromStr};

// Advent of Code 2022 - Day 13

#[derive(Debug, PartialEq, Eq, Clone)]
enum Packet {
    Value(usize),
    List(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Value(a), Self::Value(b)) => a.partial_cmp(b),
            (Self::Value(a), Self::List(_)) => Self::List(vec![Self::Value(*a)]).partial_cmp(other),
            (Self::List(_), Self::Value(b)) => self.partial_cmp(&Self::List(vec![Self::Value(*b)])),
            (Self::List(a), Self::List(b)) => {
                let mut a = a.iter();
                let mut b = b.iter();

                loop {
                    match (a.next(), b.next()) {
                        (Some(_), None) => return Some(Ordering::Greater),
                        (None, Some(_)) => return Some(Ordering::Less),
                        (None, None) => return Some(Ordering::Equal),
                        (Some(a), Some(b)) => {
                            if let Some(ordering) = a.partial_cmp(b) {
                                if ordering != Ordering::Equal {
                                    return Some(ordering);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl FromStr for Packet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input: Vec<String> = s
            .replace('[', "[ ")
            .replace(']', " ]")
            .replace(',', " , ")
            .split_whitespace()
            .map(|v| v.to_string())
            .collect();
        let packet = Self::from(&input);
        Ok(packet)
    }
}

impl From<&Vec<String>> for Packet {
    fn from(v: &Vec<String>) -> Self {
        let mut packet = Self::List(Vec::new());
        let mut parts = v.iter();
        while let Some(token) = parts.next() {
            match token.as_str() {
                "[" => {
                    let mut nested = vec![];
                    let mut depth = 1;

                    while depth > 0 {
                        let token = parts.next().unwrap();

                        match token.as_str() {
                            "[" => depth += 1,
                            "]" => depth -= 1,
                            _ => {}
                        }

                        nested.push(token.clone());
                    }

                    let nested_packet = Self::from(&nested);
                    if let Self::List(list) = &mut packet {
                        list.push(nested_packet);
                    }
                }
                "," | "]" => {}
                number => {
                    if let Self::List(list) = &mut packet {
                        list.push(Self::Value(number.parse::<usize>().unwrap() as usize));
                    }
                }
            }
        }
        packet
    }
}

pub fn solution_day_13_01(file_path: String) -> Option<usize> {
    let result = fs::read_to_string(file_path)
        .expect("Unreadable file")
        .trim()
        .split("\n\n")
        .enumerate()
        .map(|(idx, val)| {
            let mut parts = val.lines();
            let (a, b) = (
                Packet::from_str(parts.next().unwrap()),
                Packet::from_str(parts.next().unwrap()),
            );
            if a < b {
                idx + 1
            } else {
                0
            }
        })
        .sum();
    Some(result)
}

pub fn solution_day_13_02(file_path: String) -> Option<usize> {
    let dividers = [
        Packet::List(vec![Packet::List(vec![Packet::Value(2)])]),
        Packet::List(vec![Packet::List(vec![Packet::Value(6)])]),
    ];

    let mut packets: Vec<Packet> = fs::read_to_string(file_path)
        .expect("Unreadable file")
        .trim()
        .split("\n\n")
        .flat_map(|pair| pair.lines().map(|l| Packet::from_str(l).unwrap()))
        .collect();
    packets.extend(dividers.clone());

    packets.sort();

    Some(
        packets
            .into_iter()
            .enumerate()
            .filter(|(_, a)| dividers.contains(a))
            .map(|(pos, _)| pos + 1)
            .product(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_13_01() {
        let file_path: String = String::from("src/inputs/day13e.txt");
        let result = solution_day_13_01(file_path).unwrap();
        assert_eq!(result, 13);
    }

    #[test]
    fn test_day_13_02() {
        let file_path: String = String::from("src/inputs/day13e.txt");
        let result = solution_day_13_02(file_path).unwrap();
        assert_eq!(result, 140);
    }

    #[test]
    #[ignore]
    fn output_day_13_01() {
        let file_path: String = String::from("src/inputs/day13.txt");
        let result = solution_day_13_01(file_path);
        dbg!(result.unwrap());
        assert_eq!(1, 1);
    }

    #[test]
    #[ignore]
    fn output_day_13_02() {
        let file_path: String = String::from("src/inputs/day13.txt");
        let result = solution_day_13_02(file_path);
        dbg!(result.unwrap());
        assert_eq!(1, 1);
    }
}
