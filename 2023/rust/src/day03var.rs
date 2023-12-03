// These warnings are reaally annoying while coding prototype stuff
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(clippy::needless_return)]
#![allow(clippy::needless_range_loop)]

use std::collections::HashMap;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::time::Instant;
use lazy_static::lazy_static;

lazy_static! {
    static ref RE_ID: regex::Regex = Regex::new(r"\d{1,3}").unwrap();
    static ref RE_SYMBOL: regex::Regex = Regex::new(r"[*]").unwrap();
}

fn main() {
    let start = Instant::now();
    let data: Result<Vec<String>, io::Error> = read_file("input/03puzzle.txt");

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

// find gears and check if they have neighbouring numbers
fn process(data: &[String]) {    
    let mut sum:i64 = 0;
    for (y, row) in data.iter().enumerate() {
        let gear_iters = RE_SYMBOL.find_iter(row);   
        for gear in gear_iters {
            sum += check_neighbours(data, &y, &gear.start());            
        }
    }  
    println!("{}", sum); // 84900879
}

/**
 *   0  1  2  3  4  5  6  7
 *   8  9 10  * 12 13 14 15 
 *  16 17 18 19 20 21 22 23
 */
fn check_neighbours(data: &[String], row: &usize, col: &usize) -> i64 {
    let mut neighbours: String = String::new();    
    let start = col.saturating_sub(3);
    let end = start + 7;

    // iterate through rows -1, 0, 1
    for i in 0..3 {
        neighbours += &data[row + i -1][start..end];
        neighbours.push('|');
    }    

    // check if any numbers are next to the gear in the flattened view
    let num_iters = RE_ID.find_iter(&neighbours);
    let mut n1: i64 = 0;
    let mut n2: i64 = 0;

    for num in RE_ID.find_iter(&neighbours) {        
        let start = num.start();
        let end = num.end();

        if (start..end).any(|index| [2,3,4,10,12,18,19,20].contains(&index)) {
            let num_value = num.as_str().parse::<i64>().unwrap_or(-1);
            if n1 == 0 {
                n1 = num_value;
            } else if n2 == 0 {
                n2 = num_value;               
            } 
        }
    }

    return n1*n2;
}