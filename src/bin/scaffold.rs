/*
 * This file contains template code.
 * There is no need to edit this file unless you want to change template functionality.
 */
use std::{
    fs::{File, OpenOptions},
    io::Write,
    process,
};

const MODULE_TEMPLATE: &str = r###"// Advent of Code 2022 - Day DAY


pub fn solution_day_DAY_01(file_path: String) -> Option<usize> {
    None
}

pub fn solution_day_DAY_02(file_path: String) -> Option<usize> {
    None
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_DAY_01() {
        let file_path: String = String::from("src/inputs/dayDAYe.txt");
        let result = solution_day_DAY_01(file_path).unwrap();
        assert_eq!(result, 15);
    }

    #[test]
    fn test_day_DAY_02() {
        let file_path: String = String::from("src/inputs/dayDAYe.txt");
        let result = solution_day_DAY_02(file_path).unwrap();
        assert_eq!(result, 12);
    }

    #[test]
    #[ignore]
    fn output_day_DAY_01() {
        let file_path: String = String::from("src/inputs/dayDAY.txt");
        let result = solution_day_DAY_01(file_path);
        dbg!(result.unwrap());
        assert_eq!(1, 1);
    }

    #[test]
    #[ignore]
    fn output_day_DAY_02() {
        let file_path: String = String::from("src/inputs/dayDAY.txt");
        let result = solution_day_DAY_02(file_path);
        dbg!(result.unwrap());
        assert_eq!(1, 1);
    }
}
"###;

fn parse_args() -> Result<u8, pico_args::Error> {
    let mut args = pico_args::Arguments::from_env();
    args.free_from_str()
}

fn safe_create_file(path: &str) -> Result<File, std::io::Error> {
    OpenOptions::new().write(true).create_new(true).open(path)
}

fn create_file(path: &str) -> Result<File, std::io::Error> {
    OpenOptions::new().write(true).create(true).open(path)
}

fn main() {
    let day = match parse_args() {
        Ok(day) => day,
        Err(_) => {
            eprintln!("Need to specify a day (as integer). example: `cargo scaffold 7`");
            process::exit(1);
        }
    };

    let day_padded = format!("{:02}", day);

    let input_path = format!("src/inputs/day{}.txt", day_padded);
    let example_path = format!("src/inputs/day{}e.txt", day_padded);
    let module_path = format!("src/solutions/day{}.rs", day_padded);

    let mut file = match safe_create_file(&module_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create module file: {}", e);
            process::exit(1);
        }
    };

    match file.write_all(MODULE_TEMPLATE.replace("DAY", &day_padded).as_bytes()) {
        Ok(_) => {
            println!("Created module file \"{}\"", &module_path);
        }
        Err(e) => {
            eprintln!("Failed to write module contents: {}", e);
            process::exit(1);
        }
    }

    match create_file(&input_path) {
        Ok(_) => {
            println!("Created empty input file \"{}\"", &input_path);
        }
        Err(e) => {
            eprintln!("Failed to create input file: {}", e);
            process::exit(1);
        }
    }

    match create_file(&example_path) {
        Ok(_) => {
            println!("Created empty example file \"{}\"", &example_path);
        }
        Err(e) => {
            eprintln!("Failed to create example file: {}", e);
            process::exit(1);
        }
    }

    println!("---");
    println!(
        "🎄 Type `cargo solve {}` to run your solution.",
        &day_padded
    );
}
