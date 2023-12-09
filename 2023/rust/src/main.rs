/*
 * 2023 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
 */

// List of implemented solutions
mod day09;

use std::{env, fs, path::Path};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        let day = &args[1];
        let filepath = format!("input/{}/{}.txt", &args[2], day);

        if !Path::new(&filepath).is_file() {
            println!("File input/{}/{}.txt does not exist.", &args[2], &args[1]);
            return;
        }

        let execute_with_data = |func: fn(Vec<String>)| {
            func(read_data_from_file(&filepath));
        };

        // Add new days as they are implemented
        match day.as_str() {
            "09" => execute_with_data(day09::solve),
            _ => println!("Unimplemented day"),
        }
    } else {
        println!("Args: [day: integer 01..25] [folder: string real/test]");
    }
}

// parameter is a function call to another module
fn read_data_from_file(file_path: &str) -> Vec<String> {
    fs::read_to_string(file_path)
        .unwrap_or_else(|_| panic!("Failed to read {}", file_path))
        .lines()
        .map(|s| s.to_string())
        .collect()
}