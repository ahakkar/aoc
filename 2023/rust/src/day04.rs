// These warnings are reaally annoying while coding prototype stuff
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(clippy::needless_return)]
#![allow(clippy::needless_range_loop)]

use regex::Regex;
use std::fs;
use std::time::Instant;
use lazy_static::lazy_static;

lazy_static! {
    static ref RE_ID: regex::Regex = Regex::new(r"\d{1,3}").unwrap();
    static ref RE_SYMBOL: regex::Regex = Regex::new(r"[*]").unwrap();
}

fn main() {
    let start = Instant::now();
    let input = fs::read_to_string("input/03simple.txt").unwrap();
    let data: Vec<&str> = input.lines().collect();

    process(&data);

    let duration = start.elapsed();
    println!("Time elapsed in main() is: {:?}", duration);
}

fn process(data: &Vec<&str>) {    
    let mut sum:i64 = 0;

    for row in data {
        println!("{}", row);
    }

    println!("{}", sum); // 84900879
}