#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(clippy::needless_return)]
#![allow(clippy::needless_range_loop)]

use std::fs;
use std::fmt;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let input = fs::read_to_string("input/07puzzle.txt").unwrap();
    let data: Vec<&str> = input.lines().collect();

    process(&data);

    let duration = start.elapsed();
    println!("Time elapsed in main() is: {:?}", duration);
}

fn process(data: &[&str]) {  
    for row in data {

    }
}