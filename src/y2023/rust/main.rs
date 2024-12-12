/*
 * 2023 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
 */

mod utils;
// List of implemented solutions
mod day_01;
mod day_02;
mod day_05;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_20;

use std::path::Path;
use std::time::Instant;
use clap::Parser;
use utils::read_data_from_file;

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
            "01" => execute_with_data(day_01::solve),
            "02" => execute_with_data(day_02::solve),
            "05" => execute_with_data(day_05::solve),
            "08" => execute_with_data(day_08::solve),
            "09" => execute_with_data(day_09::solve),
            "10" => execute_with_data(day_10::solve),
            "11" => execute_with_data(day_11::solve),
            "12" => execute_with_data(day_12::solve),
            "13" => execute_with_data(day_13::solve),
            "14" => execute_with_data(day_14::solve),
            "15" => execute_with_data(day_15::solve),
            "16" => execute_with_data(day_16::solve),
            "17" => execute_with_data(day_17::solve),
            "18" => execute_with_data(day_18::solve),
            "19" => execute_with_data(day_19::solve),
            "20" => execute_with_data(day_20::solve),
            _ => println!("Unimplemented day"),
        }
    } else {
        println!("Args: [day: integer 01..25] [folder: string real/test]");
    }
}

