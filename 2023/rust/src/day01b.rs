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
    let terms:HashMap<&str, char> = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9')
        ]);
    let regex_first = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let regex_second = Regex::new(r"(\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)").unwrap();
    let mut sum:i64 = 0;

    for (i, row) in data.iter().enumerate() {
        //print!("{}: {:?}", i, row);

        // NOT SAFE BUT WHATEVER
        let first_str = regex_first.find(row).unwrap().as_str();
        let reversed_row:String = row.chars().rev().collect();
        let second_str = regex_second.find(&reversed_row).unwrap().as_str();

        let mut first: char = first_str.chars().next().unwrap();
        let mut second: char = second_str.chars().next().unwrap();

        // replace with actual char if needed
        if first_str.len() > 1 {
            first = terms[first_str];
        } 
        if second_str.len() > 1 {
            let original_str:String = second_str.chars().rev().collect();
            second = terms[&original_str.as_str()];
        } 

        //print!(" {}, {}", first, second);

        let coord = format!("{}{}", first, second).parse::<i64>().unwrap();
        //println!(" = {}", coord);
        sum += coord;

    }

    // 53551 too high, new guess: 53539
    println!("{}", sum);

}
