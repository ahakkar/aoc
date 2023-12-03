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
    static ref RE_ID: regex::Regex = Regex::new(r"\d+").unwrap();
    static ref RE_SYMBOL: regex::Regex = Regex::new(r"[*]").unwrap();
}

fn main() {
    let start = Instant::now();
    let data: Result<Vec<String>, io::Error> = read_file("input/03puzzle.txt");
    let mut twod_grid: Vec<Vec<char>> = vec![];

    if let Ok(lines) = data {
        conv_to_2d_grid(&lines, &mut twod_grid);
        process(&lines, &twod_grid);
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

// convert data to 2d grid so we can access each index around a num
fn conv_to_2d_grid(data: &[String], twod_grid: &mut Vec<Vec<char>>) {
    for row in data {
        let mut twod_row: Vec<char> = vec![];
        for char in row.chars() {
            twod_row.push(char);
        }
        twod_grid.push(twod_row);
    } 
}

// find gears and check if they have neighbouring numbers
fn process(data: &[String], twod_grid: &[Vec<char>]) {    
    let mut sum:i64 = 0;
    for (y, row) in data.iter().enumerate() {
        let gear_iters = RE_SYMBOL.find_iter(row);

        for gear in gear_iters {
            sum += check_neighbours(twod_grid, &y, &gear);            
        }
    }    

    // 268285485 too high
    println!("{}", sum);
}

fn check_neighbours(twod_grid: &[Vec<char>], row: &usize, gear_iter: &regex::Match<'_>) -> i64 {
    let mut neighbours: String = String::new();    
    let neighbor_positions = [9, 11, 3, 17, 2, 4, 16, 18];
    
    let grid_height = twod_grid.len();
    let grid_width = twod_grid[0].len();

    let col = gear_iter.start();

    // Construct a 7x3 "view" with the * gear centered, flatten to string
    let row_start = if row > &0 { row - 1 } else { 0 };
    let row_end = std::cmp::min(row + 2, grid_height);
    let col_start = if col > 2 { col - 3 } else { 0 };
    let col_end = std::cmp::min(col + 4, grid_width);
    
    for r in row_start..row_end {
        for c in col_start..col_end {
            if let Some(ch) = twod_grid[r].get(c) {
                neighbours.push(*ch);
            }
        }
    }

    // check if any numbers are next to the gear in the flattened view
    let num_iters = RE_ID.find_iter(&neighbours);
    let mut n1: i64 = 0;
    let mut n2: i64 = 0;

    for num in RE_ID.find_iter(&neighbours) {
        let start = num.start();
        let end = num.end() - 1;

        if neighbor_positions.contains(&start) || neighbor_positions.contains(&end) {
            let num_value = num.as_str().parse::<i64>().unwrap_or(0);
            if n1 == 0 {
                n1 = num_value;
            } else {
                n2 = num_value;
                break;
            }
        }
    }

    //println!("{:?}", neighbours);

    return n1*n2;
}



