/*
 * 2023 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
 */

use petgraph::graph::NodeIndex;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Write};
use std::fs;

use crate::util::direction::Direction;
use crate::util::point2::Point2;

pub type Visited = HashSet<(Point2, Direction)>;
pub type NodeMap = HashMap<Point2, NodeIndex>;

pub fn binomial_coefficient(n: usize, k: usize) -> usize {
    let mut result = 1;
    for i in 0..k {
        result *= n - i;
        result /= i + 1;
    }
    result
}

pub fn manhattan_distance(a: &Point2, b: &Point2) -> i64 {
    (b.x - a.x).abs() + (b.y - a.y).abs()
}

pub fn data_as_intmap(data: &[String]) -> Vec<Vec<u8>> {
    let mut map: Vec<Vec<u8>> = vec![];
    for row in data {
        let row_as_u8 = row
            .chars()
            .map(|c| c.to_digit(10).expect("Expected a digit") as u8)
            .collect::<Vec<u8>>();
        map.push(row_as_u8);
    }
    map
}

pub fn intvec_from_str(row: &str) -> Vec<isize> {
    row.split_ascii_whitespace()
        .map(|s| s.parse::<isize>().unwrap())
        .collect()
}

pub fn print_map<T>(map: &Vec<Vec<T>>)
where
    T: Display,
{
    println!("map size: [{} x {}]", map[0].len(), map.len());
    for row in map {
        let row_string = row.iter().fold(String::new(), |mut acc, item| {
            write!(acc, "{}", item)
                .expect("[utils::print_map]: Failed to write to string");
            acc
        });
        println!("{}", row_string);
    }
}

pub fn print_coords(coords: &HashSet<Point2>, c: char, e: char, w: usize, h: usize) {
    println!("map size: [{} x {}]", w, h);

    for y in 0..h {
        for x in 0..w {
            let coord = Point2::new(x as i64, y as i64);
            if coords.contains(&coord) {
                print!("{}", c);
            } else {
                print!("{}", e);
            }
        }
        println!();
    }
}

pub fn read_data_from_file(file_path: &str) -> String {
    fs::read_to_string(file_path)
        .unwrap_or_else(|_| panic!("Failed to read {}", file_path))
}

pub fn print_bit_vec<T>(vec: &[T])
where
    T: std::fmt::Binary + Sized,
{
    for line in vec {
        let bit_size = 8 * std::mem::size_of::<T>();
        println!("{:0width$b}", line, width = bit_size + 2); // +2 for '0b' prefix
    }
    println!();
}
