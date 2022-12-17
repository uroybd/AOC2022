// Advent of Code 2022 - Day 16

use std::{collections::HashMap, fs, str::FromStr, string::ParseError};

#[derive(Debug, Clone)]
struct Valve {
    name: String,
    flow_rate: usize,
    leads_to: Vec<String>,
}

impl FromStr for Valve {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("; ");
        let f_parts: Vec<&str> = parts.next().unwrap().split_whitespace().collect();
        let name = f_parts[1].to_string();
        let flow_rate = f_parts[4].split('=').last().unwrap().parse().unwrap();
        let l_parts: Vec<&str> = parts.next().unwrap().split_whitespace().collect();
        let leads_to = l_parts[4..]
            .iter()
            .map(|x| x.trim_matches(',').to_string())
            .collect();
        Ok(Self {
            name,
            flow_rate,
            leads_to,
        })
    }
}

fn parse_input(file_path: String) -> HashMap<String, Valve> {
    fs::read_to_string(file_path)
        .unwrap()
        .trim()
        .lines()
        .map(|l| {
            let valve = Valve::from_str(l).unwrap();
            (valve.name.clone(), valve)
        })
        .collect()
}

fn update(
    time: usize,
    loc: &str,
    open: usize,
    flow: usize,
    new_open: usize,
    valves: &HashMap<String, Valve>,
    flows: &mut HashMap<(String, usize, usize), usize>,
) {
    let new_flow = flow
        + valves
            .iter()
            .enumerate()
            .map(|(idx, (_, valve))| {
                if open & (1 << idx) != 0 {
                    valve.flow_rate
                } else {
                    0
                }
            })
            .sum::<usize>();
    if let Some(prev_flow) = flows.get(&(loc.to_string(), new_open, time)) {
        if new_flow > *prev_flow {
            flows.insert((loc.to_string(), new_open, time), new_flow);
        }
    } else {
        flows.insert((loc.to_string(), new_open, time), new_flow);
    }
}

fn generate_bitmap(valves: &HashMap<String, Valve>) -> HashMap<String, usize> {
    let mut keys: Vec<&String> = valves.keys().collect();
    keys.sort();
    keys.into_iter()
        .enumerate()
        .map(|(idx, key)| (key.clone(), 1 << idx))
        .collect()
}

fn get_flow_at(valves: &HashMap<String, Valve>, limit: usize) -> Option<usize> {
    let mut flows: HashMap<(String, usize), usize> = HashMap::new();
    let bitmap: HashMap<String, usize> = generate_bitmap(valves);
    let mut state = vec![("AA".to_string(), 0, 0)];
    for t in 1..=limit {
        let mut new_state = vec![];
        for (loc, opened, flow) in state.iter() {
            let key = (loc.clone(), *opened);
            if let Some(v) = flows.get(&key) {
                if flow <= v {
                    continue;
                }
            }
            flows.insert(key, *flow);

            let valve = valves.get(loc).unwrap();
            if bitmap[loc] & opened == 0 && valve.flow_rate > 0 {
                new_state.push((
                    loc.clone(),
                    opened | bitmap[loc],
                    flow + valve.flow_rate * (limit - t),
                ));
            }
            for dest in valve.leads_to.iter() {
                new_state.push((dest.clone(), *opened, *flow));
            }
        }

        state = new_state;
    }
    state.iter().map(|(_, _, flow)| *flow).max()
}

fn get_flow_history_at(
    valves: &HashMap<String, Valve>,
    limit: usize,
) -> HashMap<(String, usize, usize), usize> {
    let mut flows: HashMap<(String, usize, usize), usize> = HashMap::new();
    let bitmap: HashMap<String, usize> = valves
        .keys()
        .enumerate()
        .map(|(idx, key)| (key.clone(), idx))
        .collect();
    flows.insert(("AA".to_string(), 0, 0), 0);
    for time in 1..=limit {
        for loc in valves.keys() {
            for (from_loc, _) in valves.iter().filter(|(_, v)| v.leads_to.contains(loc)) {
                let prev: Vec<(usize, usize)> = flows
                    .iter()
                    .filter(|((ol, _, t), _)| *t == time - 1 && ol == from_loc)
                    .map(|((_, open, _), flow)| (*open, *flow))
                    .collect();

                for (open, flow) in prev {
                    update(time, loc, open, flow, open, valves, &mut flows)
                }
            }
            if valves[loc].flow_rate > 0 {
                let prev: Vec<(usize, usize)> = flows
                    .iter()
                    .filter(|((ol, open, t), _)| {
                        *t == time - 1 && ol == loc && (open & (1 << bitmap[loc]) == 0)
                    })
                    .map(|((_, open, _), flow)| (*open, *flow))
                    .collect();
                for (open, flow) in prev {
                    let new_open = open | (1 << bitmap[loc]);
                    update(time, loc, open, flow, new_open, valves, &mut flows);
                }
            }
        }
        flows.retain(|(_, _, t), _| *t == time);
    }
    flows
}

pub fn solution_day_16_01(file_path: String) -> Option<usize> {
    // let valves = parse_input(file_path);
    get_flow_at(&parse_input(file_path), 30)
}

pub fn solution_day_16_02(file_path: String) -> Option<usize> {
    let valves = parse_input(file_path);
    let flows = get_flow_history_at(&valves, 26);
    let mut sanitized_flow: HashMap<usize, usize> = HashMap::new();
    for ((_, open, t), flow) in flows.iter() {
        if *t == 26 {
            let s = sanitized_flow.entry(*open).or_default();
            *s = (*s).max(*flow);
        }
    }
    let mut total_flow = 0;
    for (&open1, &press1) in sanitized_flow.iter() {
        for (&open2, &press2) in sanitized_flow.iter() {
            if open1 & open2 == 0 {
                total_flow = total_flow.max(press1 + press2);
            }
        }
    }
    Some(total_flow)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_16_01() {
        let file_path: String = String::from("src/inputs/day16e.txt");
        let result = solution_day_16_01(file_path).unwrap();
        assert_eq!(result, 1651);
    }

    #[test]
    fn test_day_16_02() {
        let file_path: String = String::from("src/inputs/day16e.txt");
        let result = solution_day_16_02(file_path).unwrap();
        assert_eq!(result, 1707);
    }

    #[test]
    #[ignore]
    fn output_day_16_01() {
        let file_path: String = String::from("src/inputs/day16.txt");
        let result = solution_day_16_01(file_path);
        dbg!(result.unwrap());
        assert_eq!(1, 1);
    }

    #[test]
    #[ignore]
    fn output_day_16_02() {
        let file_path: String = String::from("src/inputs/day16.txt");
        let result = solution_day_16_02(file_path);
        dbg!(result.unwrap());
        assert_eq!(1, 1);
    }
}
