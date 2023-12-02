#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(clippy::needless_return)]

use std::collections::HashMap;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::time::Instant;
use lazy_static::lazy_static;

lazy_static! {
    static ref REGEX_GAME_ID: regex::Regex = Regex::new(r"\d+").unwrap();
}

fn main() {
    let start = Instant::now();
    let data: Result<Vec<String>, io::Error> = read_file("input/03simple.txt");

    if let Ok(lines) = data {
        process(&lines);
    }
    let duration = start.elapsed();
    println!("Time elapsed in main() is: {:?}", duration);
}

fn read_file(file_name: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    let mut data: Vec<String> = vec![];

    for line in reader.lines() {
        let line = line?;
        data.push(line);
    }

    Ok(data)
}

fn process(data: &[String]) {    
    let mut sum:i64 = 0;

    for row in data {
           
    }
    
    println!("{}", sum);
}