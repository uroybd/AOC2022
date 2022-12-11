use std::{
    collections::HashMap,
    fs::{self, File, OpenOptions},
    io::{self, BufRead, Write},
};

const TOP_TEMPLATE: &str = r###"## Benchmarks

| Day/Part | Part 1 | Part 2 | Total |
|:---------|-------:|-------:|------:|
"###;

#[derive(Debug, Clone)]
struct BenchData {
    id: String,
    value: f64,
    unit: String,
}

#[derive(Debug, Clone)]
struct Value {
    value: f64,
    unit: String,
}

#[derive(Debug, Clone)]
struct Row {
    day: String,
    part1: Option<Value>,
    part2: Option<Value>,
}

const UNITS: &[&str] = &["s", "ms", "us", "ns", "ps"];

fn add_durations(v1: &Value, v2: &Value) -> Value {
    if v1.unit == v2.unit {
        Value {
            value: v1.value + v2.value,
            unit: v1.unit.clone(),
        }
    } else {
        let v1_unit_pos = UNITS.iter().position(|r| r == &v1.unit.clone()).unwrap();
        let v2_unit_pos = UNITS.iter().position(|r| r == &v2.unit.clone()).unwrap();
        if v1_unit_pos > v2_unit_pos {
            Value {
                value: (v1.value / (1000.0_f64.powf((v1_unit_pos - v2_unit_pos) as f64)))
                    + v2.value,
                unit: v2.unit.clone(),
            }
        } else {
            Value {
                value: (v2.value / (1000.0_f64.powf((v2_unit_pos - v1_unit_pos) as f64)))
                    + v1.value,
                unit: v1.unit.clone(),
            }
        }
    }
}

fn format_time(v: &Value) -> String {
    let (mut unit, mut value) = (v.unit.clone(), v.value);
    while value > 1000.0 {
        value /= 1000.0;
        unit = match unit.as_str() {
            "ms" => "s".to_string(),
            "us" => "ms".to_string(),
            "ns" => "us".to_string(),
            "ps" => "ns".to_string(),
            _ => "".to_string(),
        }
    }
    if unit == *"us" {
        unit = "μs".to_string()
    }

    format!("{:.2}{}", value, unit)
}

fn format_row(r: &Row, writer: &mut File) -> Value {
    let mut times = Vec::new();
    let part_1 = match &r.part1 {
        Some(v) => {
            times.push(v);
            format_time(v)
        }
        None => "".to_string(),
    };

    let part_2 = match &r.part2 {
        Some(v) => {
            times.push(v);
            format_time(v)
        }
        None => "".to_string(),
    };
    let total = if times.len() < 2 {
        times[0].clone()
    } else {
        add_durations(times[0], times[1])
    };
    writer
        .write_all(
            format!(
                "| **{}** | {} | {} | {} |\n",
                r.day,
                part_1,
                part_2,
                format_time(&total)
            )
            .as_bytes(),
        )
        .unwrap();
    total
}

impl BenchData {
    fn from_string(line: &str) -> Option<Self> {
        let json: serde_json::Value = serde_json::from_str(line).unwrap();
        if json.get("reason").unwrap() == "benchmark-complete" {
            return Some(BenchData {
                id: json["id"].as_str().unwrap().to_string(),
                value: json["typical"]["estimate"].as_f64().unwrap(),
                unit: json["typical"]["unit"].as_str().unwrap().to_string(),
            });
        }

        None
    }
}

fn main() {
    let lines = io::stdin().lock().lines();
    let mut rows = HashMap::new();
    for line in lines {
        if let Ok(line_content) = line {
            let b_data = BenchData::from_string(&line_content);
            if let Some(row_data) = b_data {
                let mut parts = row_data.id.split('/').skip(1);
                let day = parts.next().unwrap().to_owned();
                let part = parts.next().unwrap().to_owned();
                let row = rows.entry(day.clone()).or_insert_with(|| {
                    if part == "Part 01" {
                        Row {
                            day: day.clone(),
                            part1: Some(Value {
                                value: row_data.value,
                                unit: row_data.unit.clone(),
                            }),
                            part2: None,
                        }
                    } else {
                        Row {
                            day: day.to_string(),
                            part2: Some(Value {
                                value: row_data.value,
                                unit: row_data.unit.clone(),
                            }),
                            part1: None,
                        }
                    }
                });
                if part == "Part 01" {
                    row.part1 = Some(Value {
                        value: row_data.value,
                        unit: row_data.unit.clone(),
                    });
                } else {
                    row.part2 = Some(Value {
                        value: row_data.value,
                        unit: row_data.unit.clone(),
                    });
                }
            }
        } else {
            continue;
        }
    }
    let read_me_content = fs::read_to_string("README.md").unwrap_or_else(|_| "".to_string());
    let read_me = read_me_content.split("## Benchmarks").next().unwrap();
    let mut writer = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("README.md")
        .unwrap();
    writer.write_all(read_me.as_bytes()).unwrap();
    writer.write_all(TOP_TEMPLATE.as_bytes()).unwrap();
    let mut values: Vec<&Row> = rows.values().collect();
    values.sort_by(|a, b| a.day.cmp(&b.day));
    let mut total = None;
    for v in values {
        let v_total = format_row(v, &mut writer);
        if total.is_none() {
            total = Some(v_total.clone());
        } else {
            total = Some(add_durations(&total.unwrap(), &v_total));
        }
    }
    writer.write_all("\n\n".as_bytes()).unwrap();
    if let Some(t) = total {
        writer
            .write_all(format!("**Total runtime: {}**", format_time(&t)).as_bytes())
            .unwrap();
        writer.write_all("\n\n".as_bytes()).unwrap();
    }
}
