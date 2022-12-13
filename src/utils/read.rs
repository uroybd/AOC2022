use std::{fs, str::FromStr};

pub fn read_parsed_list<T: FromStr>(file_path: String, separator: String) -> Vec<T>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let content: Vec<T> = fs::read_to_string(file_path)
        .expect("Invalid File")
        .trim()
        .split(&separator[..])
        .map(|v| v.parse::<T>().unwrap())
        .collect();
    content
}

pub fn read_float_list(file_path: String, separator: String) -> Vec<f64> {
    let content: Vec<f64> = fs::read_to_string(file_path)
        .expect("Invalid File")
        .trim()
        .split(&separator[..])
        .map(|v| v.parse::<f64>().unwrap())
        .collect();
    content
}

pub fn read_usize_list(file_path: String, separator: String) -> Vec<usize> {
    let content: Vec<usize> = fs::read_to_string(file_path)
        .expect("Invalid File")
        .trim()
        .split(&separator[..])
        .map(|v| v.parse::<usize>().unwrap())
        .collect();
    content
}
