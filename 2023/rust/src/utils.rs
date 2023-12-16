/*
 * 2023 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
 */

#![allow(dead_code)]

use std::collections::HashSet;
use std::{fmt, fs};
use std::fmt::{Display, Write};

pub type Grid<T> = Vec<Vec<T>>;
pub type Visited = HashSet<(Coord, Vec2D)>;

pub const NORTH: Vec2D = Vec2D::new(0, -1);
pub const SOUTH: Vec2D = Vec2D::new(0, 1);
pub const EAST: Vec2D = Vec2D::new(1, 0);
pub const WEST: Vec2D = Vec2D::new(-1, 0);

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Coord {
    pub x: isize,
    pub y: isize,
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Vec2D {
    pub x: isize,
    pub y: isize,
}

#[derive(Clone)]
pub struct GridMap<T> {
    d: Grid<T>,
    w: usize,
    h: usize,
}

impl Coord {
    pub fn new(x: isize, y: isize) -> Coord {
        Coord { x, y }
    }
}    

impl fmt::Debug for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

impl Vec2D {
    pub const fn new(x: isize, y: isize) -> Vec2D {
        Vec2D { x, y }
    }
}  

impl fmt::Debug for Vec2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

impl<T> GridMap<T>
where
    T: Copy,
{
    pub fn new(d: Grid<T>) -> GridMap<T> {
        let w = d[0].len();
        let h = d.len();
        GridMap {d, w, h}
    }

    pub fn get(&self, xy: &Coord) -> Option<T> {
        if xy.x < self.w as isize && xy.y < self.h as isize && xy.x >= 0 && xy.y >= 0{
            Some(self.d[xy.y as usize][xy.x as usize])
        } else {
            None
        }
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

pub fn manhattan_distance(a: &Coord, b: &Coord) -> i64 {    
    (b.x as i64 - a.x as i64).abs() + 
    (b.y as i64 - a.y as i64).abs()   
}

pub fn data_as_chars(data: &[String]) -> Grid<char> {
    let mut data_as_chars: Grid<char> = vec![];
    for row in data {
        data_as_chars.push(row.chars().collect::<Vec<char>>());
    }
    data_as_chars
}

pub fn print_map<T>(map: &Vec<Vec<T>>)
where
    T: Display,
{
    println!("map size: [{} x {}]", map[0].len(), map.len());
    for row in map {
        let row_string = row.iter().fold(String::new(), |mut acc, item| {
            write!(acc, "{}", item).expect("[utils::print_map]: Failed to write to string");
            acc
        });
        println!("{}", row_string);
    }
}

pub fn print_coords(coords: &HashSet<Coord>, c: char, e: char, w: usize, h: usize) {
    println!("map size: [{} x {}]", w, h);

    for y in 0..h {
        for x in 0..w {
            let coord = Coord::new(x as isize, y as isize);
            if coords.contains(&coord) {
                print!("{}", c);
            } else {
                print!("{}", e);
            }
        }
        println!();
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