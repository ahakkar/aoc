/*
 * 2023 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
 */

mod utils;
// List of implemented solutions
mod day09;
mod day10;
mod day_11;
mod day_12;

use std::{fs, path::Path};
use std::time::Instant;
use clap::Parser;

/// 2023 Advent of Code with Rust
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// [01..25] day of calendar
    #[arg(short, long)]
    day: String,

    /// Run tests
    #[arg(short, long, action)]
    test: bool,
}

fn main() {
    let args = Args::parse();

    if args.day.len() == 2 {
        let folder = if args.test { "test" } else { "real" };
        let filepath = format!("input/{}/{}.txt", folder, args.day);

        if !Path::new(&filepath).is_file() {
            println!("File {} does not exist.", filepath);
            return;
        }

        let execute_with_data = |func: fn(Vec<String>)| {
            let start = Instant::now();
            func(read_data_from_file(&filepath));
            let duration = start.elapsed();
            println!("Time elapsed in day{} is: {:?}", args.day, duration);
        };

        // Add new days as they are implemented
        match args.day.as_str() {
            "09" => execute_with_data(day09::solve),
            "10" => execute_with_data(day10::solve),
            "11" => execute_with_data(day_11::solve),
            "12" => execute_with_data(day_12::solve),
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