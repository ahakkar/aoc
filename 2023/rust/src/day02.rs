#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::collections::HashMap;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let data: Result<Vec<String>, io::Error> = read_file("input/01puzzle.txt");

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
}