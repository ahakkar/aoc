/*
 * 2024 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
 */

#![allow(clippy::format_in_format_args)]
use aoc::*;
use clap::{ArgGroup, Parser};

use colored::*;
use std::path::Path;
use util::utils::read_data_from_file;

#[derive(Parser, Debug)]
#[command(
    author = env!("CARGO_PKG_AUTHORS"), 
    version = env!("CARGO_PKG_VERSION"),
    about = format!("{} v. {}\nAuthor: {} 2020-2024", 
        env!("CARGO_PKG_DESCRIPTION"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),          
        ), 
    long_about = None
)]
#[command(group = ArgGroup::new("mode").args(&["day", "all"]).required(true))]
struct Args {
    /// Run all solutions, can not be run at same time with day
    #[arg(short, long, action)]
    all: bool,

    /// [01..25] day of calendar
    #[arg(short, long)]
    day: Option<String>,

    /// Run N times
    #[arg(short, long, action)]
    number: Option<String>,

    /// Run with test data instead of real data
    #[arg(short, long, action)]
    test: bool,

    /// [2015..2024] calendar year
    #[arg(short, long)]
    year: Option<String>,
}

// rustfmt settings
// https://rust-lang.github.io/rustfmt/?version=v1.8.0&search=
fn main() {
    // example: cargo run -- --day 01 --test
    let args = Args::parse();
    let n = args
        .number
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(1);

    let year = args
        .year               
        .unwrap_or("2024".to_string()); // 2024 as default year

    if let Some(day) = args.day.as_deref() {
        run_one_day(&year, day, &n, &args.test);
    } else if args.all {
        run_all(&year, &n, &args.test);
    }
}

struct RunResult {
    score_silver: TaskResult,
    total_silver: Vec<u128>,
    score_gold: TaskResult,
    total_gold: Vec<u128>,
    total_fro: Vec<u128>,
}

fn run(day: &str, n: &usize, filepath: &str) -> RunResult {
    let mut total_silver: Vec<u128> = vec![];
    let mut total_gold: Vec<u128> = vec![];
    let mut total_fro: Vec<u128> = vec![];  

    // Run solution once to get the score
    let temp = solve(day, &read_data_from_file(filepath));
    total_silver.push(temp.silver.1.as_micros());
    total_gold.push(temp.gold.1.as_micros());
    total_fro.push(temp.fro.as_micros());
    let score_silver = temp.silver.0;
    let score_gold = temp.gold.0;

    // Collect runtimes across leftover iterations
    for _ in 0..*n-1 {
        let temp = solve(day, &read_data_from_file(filepath));
        total_silver.push(temp.silver.1.as_micros());
        total_gold.push(temp.gold.1.as_micros());
        total_fro.push(temp.fro.as_micros());
    }

    RunResult { score_silver, total_silver, score_gold, total_gold, total_fro }
}


fn run_one_day(year: &str, day: &str, n: &usize, test: &bool) {
    if day.len() != 2 {
        println!("Args: [day: integer 01..25] [folder: string real/test]");
        return;
    }
    let folder = if *test { "test" } else { "real" };
    let filepath = format!("input/{}/{}/{}.txt", year, folder, day);

    if !Path::new(&filepath).is_file() {
        println!("File {} does not exist.", filepath);
        return;
    }

    if *n == 0 { return } // todo complain about this

    let res = run(day, n, &filepath);

    println!("Silver: {}, Gold: {}", res.score_silver, res.score_gold);
    print!("Ran solutions {} times, avg: ", n);
    println!(
        "Silver: {}, Gold: {}, Input: {}",
        format_duration(res.total_silver),
        format_duration(res.total_gold),
        format_duration(res.total_fro),
    );
}

fn run_all(year: &str, n: &usize, _test: &bool) {
    print!("Time is a {} run average.", n);

    print_header();

    for day in ["01", "02", "03", "04", "05", "06", "07", "08", "09", "10"] {
        let filepath = format!("input/{}/real/{}.txt", year, day);
        if !Path::new(&filepath).is_file() {
            println!("File {} does not exist.", filepath);
            return;
        }

        print!("║ {}   ║", day.green());

        let res = run(day, n, &filepath);

        print!("{:>23} ║", res.score_silver.to_string().bright_magenta());
        print!("{:>23} ║", res.score_gold.to_string().bright_magenta());
        print!("{:>10} ║", format_duration(res.total_silver));
        print!("{:>10} ║", format_duration(res.total_gold));
        println!("{:>10} ║", format_duration(res.total_fro));
    }

    print_footer();
}

fn format_duration(duration: Vec<u128>) -> String {
    let us: u128 = duration.iter().sum::<u128>() / duration.len() as u128;

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
        "\n{}{}{}{}{}{}{}{}{}{}{}{}{}",
        top_left_corner,
        "═".repeat(6).yellow(),
        top_down_tee,
        "═".repeat(24).yellow(),
        top_down_tee,
        "═".repeat(24).yellow(),
        top_down_tee,
        "═".repeat(11).yellow(),
        top_down_tee,
        "═".repeat(11).yellow(),
        top_down_tee,
        "═".repeat(11).yellow(),
        top_right_corner,
    );

    println!(
        "{}{:<6}{}{:>24}{}{:>24}{}{}{}{:>16}{}{}{}",
        vert_border,
        " Day".bright_red(),
        vert_border,
        format!("{:>16}Silver{}", "Result (".bright_red(), ") ".bright_red()),
        vert_border,
        format!(
            "{:>18}{}{}",
            "Result (".bright_red(),
            "Gold".yellow(),
            ") ".bright_red()
        ),
        vert_border,
        format!("{:>8}S{}", "Time (".bright_red(), ") ".bright_red()),
        vert_border,
        format!(
            "{:>8}{}{}",
            "Time (".bright_red(),
            "G".yellow(),
            ") ".bright_red()
        ),
        vert_border,
        format!(
            "{:>8}{}{}",
            "Time (".bright_red(),
            "I",
            ") ".bright_red()
        ),
        vert_border,
    );
    println!(
        "{}{}{}{}{}{}{}{}{}{}{}{}{}",
        vert_right_tee,
        "═".repeat(6).cyan(),
        cross,
        "═".repeat(24).cyan(),
        cross,
        "═".repeat(24).cyan(),
        cross,
        "═".repeat(11).cyan(),
        cross,
        "═".repeat(11).cyan(),
        cross,
        "═".repeat(11).cyan(),
        vert_left_tee,
    );
}

fn print_footer() {
    println!(
        "╚{}╩{}╩{}╩{}╩{}╩{}╝\n",
        "═".repeat(6),
        "═".repeat(24),
        "═".repeat(24),
        "═".repeat(11),
        "═".repeat(11),
        "═".repeat(11),
    );
}
