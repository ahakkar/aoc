/*
 * 2024 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
 */
mod utils;

use std::path::Path;
use std::time::Instant;
use aoc2024::solve;
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

        let start = Instant::now();    
        solve(args.day.as_str(), &read_data_from_file(&filepath));
        let duration = start.elapsed();
        println!("Time elapsed in day{} is: {:?}", args.day, duration);

    } else {
        println!("Args: [day: integer 01..25] [folder: string real/test]");
    }
}

