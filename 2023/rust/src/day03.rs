// These warnings are reaally annoying while coding prototype stuff
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(clippy::needless_return)]

use std::collections::HashMap;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::time::Instant;
use lazy_static::lazy_static;

lazy_static! {
    static ref RE_ID: regex::Regex = Regex::new(r"\d+").unwrap();
    static ref RE_SYMBOLS: regex::Regex = Regex::new(r"[-*#&+$%/@=]").unwrap();
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

// extract nums with regex and check their surroundings
fn process(data: &[String], twod_grid: &[Vec<char>]) {    
    let mut sum:i64 = 0;

    for (y, row) in data.iter().enumerate() {
        let ids_iter = RE_ID.find_iter(row);  
        for iter in ids_iter {
            //println!("{:?}", iter);

            // Add the id to sum if it has a symbol as neighbour
            if check_neighbours(twod_grid, &y, &iter) {
                sum += iter.as_str().parse::<i64>().unwrap();
            }
        }
    }
    
    // too low 516060, new guess: 530849
    println!("{}", sum);
}

fn check_neighbours(twod_grid: &[Vec<char>], row: &usize, id_iter: &regex::Match<'_>) -> bool {
    let mut neighbours: String = String::new();    
    let grid_height = twod_grid.len();
    let grid_width = twod_grid[0].len();

    let start = id_iter.start();
    let end = id_iter.end();
    let length = end-start;

    let row_start = if row > &0 { row - 1 } else { 0 };
    let row_end = std::cmp::min(row + 2, grid_height);
    let col_start = if start > 0 { start - 1 } else { 0 };
    let col_end = std::cmp::min(end + 1, grid_width);

    // Iterate over the surrounding cells
    for r in row_start..row_end {
        for c in col_start..col_end {
            // Skip the actual numbers
            if r == *row && (c >= start && c < end) {
                continue;
            }
            if let Some(ch) = twod_grid[r].get(c) {
                neighbours.push(*ch);
            }
        }
    }

    return RE_SYMBOLS.find(neighbours.as_str()).is_some();
}

