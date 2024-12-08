/*
 * 2024 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
 */

#![allow(clippy::format_in_format_args)]

mod utils;

use std::{path::Path};
use std::time::Instant;
use aoc2024::{solve};
use clap::Parser;
use utils::read_data_from_file;
use colored::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// [01..25] day of calendar
    #[arg(short, long)]
    day: Option<String>,

    /// Run tests
    #[arg(short, long, action)]
    test: bool,

    /// Run all solutions
    #[arg(short, long, action)]
    all: bool,

}


fn main() {
    // example: cargo run -- --day 01 --test
    let args = Args::parse();

    if let Some(day) = args.day.as_deref() {
        if day.len() != 2 {            
            println!("Args: [day: integer 01..25] [folder: string real/test]");
            return;
        }
        let folder = if args.test { "test" } else { "real" };
        let filepath = format!("input/{}/{}.txt", folder, day);

        if !Path::new(&filepath).is_file() {
            println!("File {} does not exist.", filepath);
            return;
        }

        let start = Instant::now();    
        let result = solve(day, &read_data_from_file(&filepath));
        println!("Silver: {}, Gold: {}", result.silver.0, result.gold.0);
        let duration = start.elapsed();
        println!("Time elapsed in day{} is: {:?}", day, duration);
    } 
    else if args.all {
        const RUNS: u8 = 10;
        print!("Time is a {} run average.", RUNS);

        print_header();

        for day in ["01", "02", "03", "04", "05", "06", "07", "08"] {
            let filepath = format!("input/real/{}.txt", day);
            if !Path::new(&filepath).is_file() {
                println!("File {} does not exist.", filepath);
                return;
            } 

            print!("║ {}   ║", day.green());  
   
            let mut avg_silver: Vec<u128> = vec![];
            let mut avg_gold: Vec<u128> = vec![];

            let res = solve(day, &read_data_from_file(&filepath));
            let score_silver = res.silver.0;
            let score_gold = res.gold.0;

            for _ in 0..RUNS {                 
                let temp = solve(day, &read_data_from_file(&filepath));
                avg_silver.push(temp.silver.1.as_micros());
                avg_gold.push(temp.gold.1.as_micros());
            }

            print!("{:>23} ║", score_silver.to_string().bright_magenta());
            print!("{:>23} ║", score_gold.to_string().bright_magenta());
            print!("{:>15} ║", format_duration(avg_silver));
            println!("{:>15} ║", format_duration(avg_gold));
        }

        print_footer();
    }
}

fn format_duration(duration: Vec<u128>) -> String {
    let us: u128 = duration
        .iter()        
        .sum::<u128>() / duration.len() as u128;

    if us < 1_000 {
        format!("{} µs", us) // Microseconds
    } else if us < 1_000_000 {
        format!("{:.0} ms", us as f64 / 1_000.0) // Milliseconds
    } else {
        format!("{:.0} s", us as f64 / 1_000_000.0) // Seconds
    }
}

fn print_header() {
    let top_left_corner = "╔".yellow();
    let top_right_corner = "╗".yellow();
    let top_down_tee = "╦".yellow();
    let vert_border = "║".yellow();
    let vert_right_tee = "╠".cyan();
    let vert_left_tee = "╣".cyan();
    let cross = "╬".cyan();

    println!(
        "\n{}{}{}{}{}{}{}{}{}{}{}",
        top_left_corner,
        "═".repeat(6).yellow(),
        top_down_tee,
        "═".repeat(24).yellow(),
        top_down_tee,
        "═".repeat(24).yellow(),
        top_down_tee,
        "═".repeat(16).yellow(),
        top_down_tee,
        "═".repeat(16).yellow(),
        top_right_corner,
    );
    
    println!(
        "{}{:<6}{}{:>24}{}{:>24}{}{}{}{:>16}{}",
        vert_border,
        " Day".bright_red(),
        vert_border,
        format!("{:>16}Silver{}", "Result (".bright_red(), ") ".bright_red()),
        vert_border,
        format!("{:>18}{}{}", "Result (".bright_red(), "Gold".yellow(), ") ".bright_red()),
        vert_border,
        format!("{:>8}Silver{}", "Time (".bright_red(), ") ".bright_red()),
        vert_border,
        format!("{:>10}{}{}", "Time (".bright_red(), "Gold".yellow(), ") ".bright_red()),
        vert_border,
    );
    println!(
        "{}{}{}{}{}{}{}{}{}{}{}",
        vert_right_tee,
        "═".repeat(6).cyan(),
        cross,
        "═".repeat(24).cyan(),
        cross,
        "═".repeat(24).cyan(),
        cross,
        "═".repeat(16).cyan(),
        cross,
        "═".repeat(16).cyan(),
        vert_left_tee,
    );
}

fn print_footer() {
    println!(
        "╚{}╩{}╩{}╩{}╩{}╝\n",
        "═".repeat(6),
        "═".repeat(24),
        "═".repeat(24),
        "═".repeat(16),
        "═".repeat(16),
    );
}