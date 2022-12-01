use std::fs;

pub fn read_integer_list(file_path: String, separator: String) -> Vec<i64> {
    let content: Vec<i64> = fs::read_to_string(file_path)
        .expect("Invalid File")
        .trim()
        .split(&separator[..])
        .map(|v| v.parse::<i64>().unwrap())
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

pub fn read_lines(file_path: String) -> Vec<String> {
    let content: Vec<String> = fs::read_to_string(file_path)
        .expect("Invalid File")
        .trim()
        .split('\n')
        .map(|s| s.to_string())
        .collect();
    content
}
