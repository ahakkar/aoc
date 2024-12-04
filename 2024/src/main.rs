/*
 * 2024 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
 */

mod utils;
// List of implemented solutions
mod day_01;
mod day_01b;
mod day_02;
mod day_03;
mod day_04;
mod day_05;

use std::path::Path;
use std::time::Instant;
use clap::Parser;
use utils::read_data_from_file;

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
    // example: cargo run -- --day 01 --test
    let args = Args::parse();

    if args.day.len() == 2 {
        let folder = if args.test { "test" } else { "real" };
        let filepath = format!("input/{}/{}.txt", folder, args.day);

        if !Path::new(&filepath).is_file() {
            println!("File {} does not exist.", filepath);
            return;
        }

        let execute_with_data = |silver: fn(&[String]) -> usize, gold: fn(&[String]) -> usize| {
            let data = read_data_from_file(&filepath);  
            let start = Instant::now();          
            println!("Silver: {}", silver(&data));
            println!("Gold: {}", gold(&data));
            let duration = start.elapsed();
            println!("Time elapsed in day{} is: {:?}", args.day, duration);
        };

        // Add new days as they are implemented
        match args.day.as_str() {
            "01" => execute_with_data(day_01::silver, day_01::gold),
            "01b" => execute_with_data(day_01b::silver, day_01b::gold),
            "02" => execute_with_data(day_02::silver, day_02::gold),
            "03" => execute_with_data(day_03::silver, day_03::gold),
            "04" => execute_with_data(day_04::silver, day_04::gold),
            "05" => execute_with_data(day_05::silver, day_05::gold),
            _ => println!("Unimplemented day"),
        }
    } else {
        println!("Args: [day: integer 01..25] [folder: string real/test]");
    }
}

