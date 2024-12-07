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
    
            let start = Instant::now();    
            solve(day, &read_data_from_file(&filepath));
            let duration = start.elapsed();
            println!(" {:<15?}║", duration);
        }

        print_footer();
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
        "\n{}{}{}{}{}{}{}{}{}",
        top_left_corner,
        "═".repeat(6).yellow(),
        top_down_tee,
        "═".repeat(32).yellow(),
        top_down_tee,
        "═".repeat(32).yellow(),
        top_down_tee,
        "═".repeat(16).yellow(),
        top_right_corner,
    );
    println!(
        "{}{}{} Silver {}{}{}{}{}{}{}{}",
        vert_border,
        " Day  ".bright_red(),
        vert_border,
        " ".repeat(24),
        vert_border,
        " Gold ".yellow(),
        " ".repeat(26),
        vert_border,
        " Time ".bright_red(),
        " ".repeat(10),
        vert_border,
    );
    println!(
        "{}{}{}{}{}{}{}{}{}",
        vert_right_tee,
        "═".repeat(6).cyan(),
        cross,
        "═".repeat(32).cyan(),
        cross,
        "═".repeat(32).cyan(),
        cross,
        "═".repeat(16).cyan(),
        vert_left_tee,
    );
}

fn print_footer() {
    println!(
        "╚{}╩{}╩{}╩{}╝\n",
        "═".repeat(6),
        "═".repeat(32),
        "═".repeat(32),
        "═".repeat(16),
    );
}