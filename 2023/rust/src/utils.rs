/*
 * 2023 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
 */

#![allow(dead_code)]

use std::{fmt, fs};

#[derive(Clone)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl fmt::Debug for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}
    

pub fn binomial_coefficient(n: usize, k: usize) -> usize {
    let mut result = 1;
    for i in 0..k {
        result *= n - i;
        result /= i + 1;
    }
    result
}

pub fn manhattan_distance(a: &Coord, b: &Coord) -> i64 {    
    (b.x as i64 - a.x as i64).abs() + 
    (b.y as i64 - a.y as i64).abs()   
}

pub fn data_as_chars(data: &[String]) -> Vec<Vec<char>> {
    let mut data_as_chars: Vec<Vec<char>> = vec![];
    for row in data {
        data_as_chars.push(row.chars().collect::<Vec<char>>());
    }
    data_as_chars
}

fn print_map(galaxies: &[Coord], map_w: usize, map_h: usize) {
    let mut map:Vec<Vec<char>> = vec![vec!['.'; map_w]; map_h];
    for galaxy in galaxies {
        map[galaxy.y][galaxy.x] = '#';
    }
    println!("map size: [{} x {}]", map_w, map_h);
    for row in &map {
        let row_string: String = row.iter().collect();
        println!("{}", row_string);
    }
}

pub fn read_data_from_file(file_path: &str) -> Vec<String> {
    fs::read_to_string(file_path)
        .unwrap_or_else(|_| panic!("Failed to read {}", file_path))
        .lines()
        .map(|s| s.to_string())
        .collect()
}

pub fn print_bit_vec<T>(vec: &[T]) 
where
    T: std::fmt::Binary + Sized,
{
    for line in vec {
        let bit_size = 8 * std::mem::size_of::<T>();
        println!("{:0width$b}", line, width=bit_size + 2); // +2 for '0b' prefix
    }
    println!();
}