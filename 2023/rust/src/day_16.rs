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

use super::utils::{Grid, GridMap, Coord, Vec2D, data_as_chars};


pub fn solve(data: Vec<String>) {
    println!("Silver: {}", silver(&data));
    //println!("Gold: {}", gold(&data));
}

fn silver(data: &[String]) -> usize {
    let mut map:GridMap = GridMap::new(data_as_chars(data)); 
    let mut energized:HashSet<Coord> = HashSet::new();
    energized.insert(Coord::new(0,0)); // easier to add starting position manually
    follow_path(&mut map, Coord{x: 0, y:0}, Vec2D::new(1, 0), &mut energized);

    energized.len() 
}

/*
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
*/

const NORTH: Vec2D = Vec2D::new(0, -1);
const SOUTH: Vec2D = Vec2D::new(0, 1);
const EAST: Vec2D = Vec2D::new(1, 0);
const WEST: Vec2D = Vec2D::new(-1, 0);

fn follow_path(
    map: &mut GridMap, 
    prev: Coord,
    dir: Vec2D, 
    energized: &mut HashSet<Coord>
) {
    let cur = Coord::new(prev.x + dir.x, prev.y + dir.y);

    if let Some(char) = map.get(&cur) {
        energized.insert(prev.clone());
        match char {
            '|' => {
                if dir.x.abs() == 1 && dir.y == 0 {
                    follow_path(map, cur.clone(), Vec2D::new(0, -1), energized);
                    follow_path(map, cur, Vec2D::new(0, 1), energized);
                } else {
                    follow_path(map, cur, dir, energized);
                }                
            },
            '-' => {
                if dir.y.abs() == 1 && dir.x == 0 {
                    follow_path(map, cur.clone(), Vec2D::new(1, 0), energized);
                    follow_path(map, cur, Vec2D::new(-1, 0), energized);
                } else {
                    follow_path(map, cur, dir, energized);
                };
                
            },
            '/' => {
                match dir {
                    NORTH => follow_path(map, cur, EAST, energized),
                    SOUTH => follow_path(map, cur, WEST, energized),
                    EAST  => follow_path(map, cur, NORTH, energized),
                    WEST  => follow_path(map, cur, SOUTH, energized),
                    _     => panic!("invalid dir"),
                };
            },
           '\\' => {
                match dir {
                    NORTH => follow_path(map, cur, WEST, energized),
                    SOUTH => follow_path(map, cur, EAST, energized),
                    EAST  => follow_path(map, cur, SOUTH, energized),
                    WEST  => follow_path(map, cur, NORTH, energized),
                    _     => panic!("invalid dir"),
                };
            },
            '.' => follow_path(map, cur, dir, energized),
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