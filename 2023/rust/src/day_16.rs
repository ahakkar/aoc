/*
 * 2023 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(clippy::needless_return)]
#![allow(clippy::needless_range_loop)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use std::collections::{BTreeSet, HashSet};

use super::utils::{Grid, GridMap, Coord, Vec2D, data_as_chars, print_coords};

const NORTH: Vec2D = Vec2D::new(0, -1);
const SOUTH: Vec2D = Vec2D::new(0, 1);
const EAST: Vec2D = Vec2D::new(1, 0);
const WEST: Vec2D = Vec2D::new(-1, 0);

pub fn solve(data: Vec<String>) {
    println!("Silver: {}", silver(&data)); // 7210
    //println!("Gold: {}", gold(&data));
}

fn silver(data: &[String]) -> usize {
    let mut map:GridMap = GridMap::new(data_as_chars(data)); 
    let mut visited:HashSet<(Coord, Vec2D)> = HashSet::new();
    follow_path(&mut map, Coord::new(-1,0), EAST, &mut visited);

    let mut energized:HashSet<Coord> = HashSet::new();
    for visit in visited {        
        energized.insert(visit.0);
    }

    //print_coords(&energized, '\u{1F44D}', '\u{2B1B}', map.get_width(), map.get_height());
    //println!("{:?}", energized);
    energized.len() 
}

fn follow_path(
    map: &mut GridMap, 
    prev: Coord,
    dir: Vec2D, 
    visited: &mut HashSet<(Coord, Vec2D)>
) {
    let cur = Coord::new(prev.x + dir.x, prev.y + dir.y);

    if let Some(char) = map.get(&cur) {
        if !visited.insert((cur.clone(), dir.clone())) {
            return;
        }

        //println!("Current: {:?}, char: {}", cur, char);
        match char {
            '|' => {
                if dir.x.abs() == 1 && dir.y == 0 {
                    follow_path(map, cur.clone(), NORTH, visited);
                    follow_path(map, cur, SOUTH, visited);
                } else {
                    follow_path(map, cur, dir, visited);
                }                
            },
            '-' => {
                if dir.x == 0 && dir.y.abs() == 1 {
                    follow_path(map, cur.clone(), EAST, visited);
                    follow_path(map, cur, WEST, visited);
                } else {
                    follow_path(map, cur, dir, visited);
                };
                
            },
            '/' => {
                match dir {
                    NORTH => follow_path(map, cur, EAST, visited),
                    SOUTH => follow_path(map, cur, WEST, visited),
                    EAST  => follow_path(map, cur, NORTH, visited),
                    WEST  => follow_path(map, cur, SOUTH, visited),
                    _     => panic!("invalid dir"),
                };
            },
           '\\' => {
                match dir {
                    NORTH => follow_path(map, cur, WEST, visited),
                    SOUTH => follow_path(map, cur, EAST, visited),
                    EAST  => follow_path(map, cur, SOUTH, visited),
                    WEST  => follow_path(map, cur, NORTH, visited),
                    _     => panic!("invalid dir"),
                };
            },
            '.' => follow_path(map, cur, dir, visited),
             _  => panic!("Invalid char at map {:?}, char: {}", cur, char),
        }
    }
}

/* fn gold(data: &[String]) -> usize {
    let mut sum: usize = 0;    

    for row in data {
           } 
    sum 
} */

// run these with cargo test --bin main -- day_13::tests
#[cfg(test)]
mod tests {
    use crate::utils::read_data_from_file;
    use super::*;   

    #[test]
    fn test_test() {
        let test_data:Vec<String> = read_data_from_file("input/test/16.txt");
        assert_eq!(silver(&test_data), 46);
        //assert_eq!(gold(&test_data), 145);
    }

    #[test]
    fn test_silver() {
        let test_data:Vec<String> = read_data_from_file("input/real/16.txt");
        assert_eq!(silver(&test_data), 510801);
    }

    #[test]
    fn test_gold() {
        let test_data:Vec<String> = read_data_from_file("input/real/16.txt");
        //assert_eq!(gold(&test_data), 212763);
    }
}