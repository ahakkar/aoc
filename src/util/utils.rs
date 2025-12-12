/*
 * 2023 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
 */

#![allow(dead_code)]

use petgraph::graph::NodeIndex;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Write};
use std::fs;

use crate::util::direction::Direction;
use crate::util::point2::Point2;

pub type Grid<T> = Vec<Vec<T>>;
pub type Visited = HashSet<(Point2, Direction)>;
pub type NodeMap = HashMap<Point2, NodeIndex>;

pub enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Clone)]
pub struct GridMap<T> {
    d: Grid<T>,
    w: usize,
    h: usize,
}

impl<T> GridMap<T>
where
    T: Copy,
{
    pub fn new(d: Grid<T>) -> GridMap<T> {
        let w = d[0].len();
        let h = d.len();
        GridMap { d, w, h }
    }

    pub fn get_cell(&self, xy: &Point2) -> Option<T> {
        if xy.x < self.w as i64 && xy.y < self.h as i64 && xy.x >= 0 && xy.y >= 0 {
            Some(self.d[xy.y as usize][xy.x as usize])
        } else {
            None
        }
    }

    pub fn get_idx(&self, x: usize, y: usize) -> Option<&T> {
        if x < self.w && y < self.h {
            Some(&self.d[y][x])
        } else {
            None
        }
    }

    pub fn get_idx_ub(&self, x: isize, y: isize) -> Option<&T> {
        if (x >= 0 && y >= 0) && (x < self.w as isize && y < self.h as isize) {
            Some(&self.d[y as usize][x as usize])
        } else {
            None
        }
    }

    pub fn get_neighbor(&self, idx: &Point2, dir: Direction) -> Option<T> {
        let (dx, dy) = match dir {
            Direction::East => (1, 0),
            Direction::North => (0, -1),
            Direction::West => (-1, 0),
            Direction::South => (0, -1),
            Direction::NorthEast => (1, 1),
            Direction::NorthWest => (-1, 1),
            Direction::SouthEast => (1, -1),
            Direction::SouthWest => (-1, -1),
            _ => panic!("invalid direction"),
        };

        if idx.x + dx >= 0 && idx.y + dy >= 0 {
            return self.get_cell(&Point2::new(idx.x + dx, idx.y + dy));
        }
        None
    }

    pub fn get_data(&self) -> &Grid<T> {
        &self.d
    }
    pub fn get_height(&self) -> usize {
        self.h
    }
    pub fn get_width(&self) -> usize {
        self.w
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

pub fn manhattan_distance(a: &Point2, b: &Point2) -> i64 {
    (b.x - a.x).abs() + (b.y - a.y).abs()
}

pub fn data_as_chars(data: &[String]) -> Grid<char> {
    let mut data_as_chars: Grid<char> = vec![];
    for row in data {
        data_as_chars.push(row.chars().collect::<Vec<char>>());
    }
    data_as_chars
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
