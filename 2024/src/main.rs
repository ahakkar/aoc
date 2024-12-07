/*
 * 2024 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
 */
mod utils;

use std::{path::Path, time::Duration};
use std::time::Instant;
use aoc2024::{solve, AocResult};
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
        solve(day, &read_data_from_file(&filepath));
        let duration = start.elapsed();
        println!("Time elapsed in day{} is: {:?}", day, duration);
    } 
    else if args.all {
        print_header();
        for day in ["01", "02", "03", "04", "05", "06", "07"] {
            let filepath = format!("input/real/{}.txt", day);
            if !Path::new(&filepath).is_file() {
                println!("File {} does not exist.", filepath);
                return;
            } 

            print!("║ {}   ║", day.green());       
            let res: AocResult = solve(day, &read_data_from_file(&filepath));
            print!("{:>23} ║", res.silver.0.to_string().bright_magenta());
            print!("{:>23} ║", res.gold.0.to_string().bright_magenta());
            print!("{:>15} ║", format_duration(res.silver.1));
            println!("{:>15} ║", format_duration(res.gold.1));
        }

        print_footer();
    }
}

fn format_duration(duration: Duration) -> String {
    let us = duration.as_micros(); // Total microseconds
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
        "{}{:<6}{}{:<24}{}{:<24}{}{}{}{}{}{}{}",
        vert_border,
        " Day".bright_red(),
        vert_border,
        " Silver",
        vert_border,
        " Gold".yellow(),    
        vert_border,
        " Time 01 ".bright_red(),
        " ".repeat(7),
        vert_border,
        " Time 02 ".bright_red(),
        " ".repeat(7),
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