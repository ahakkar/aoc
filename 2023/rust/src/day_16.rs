/*
 * 2023 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

use std::collections::HashSet;
use super::utils::*;

enum Elem {
    Empty,
    Ver,
    Hor,
    LMir,
    RMir,    
}

pub fn solve(data: Vec<String>) {    
    println!("Silver: {}", silver(&data)); // 7210
    println!("Gold: {}", gold(&data)); // 7672 too low, 7673
}

fn silver(data: &[String]) -> usize {
    let map:GridMap<char> = GridMap::new(data_as_chars(data)); 
    get_energized(&map, Coord::new(-1,0), EAST)
}

fn gold(data: &[String]) -> usize {
    let map:GridMap<char> = GridMap::new(data_as_chars(data)); 
    let width = map.get_width() as isize;
    let height = map.get_height() as isize;

    let top_row = (0..width).map(|x| (Coord::new(x, -1), SOUTH));
    let bottom_row = (0..width).map(|x| (Coord::new(x, height), NORTH));
    let left_column = (0..height).map(|y| (Coord::new(-1, y), EAST));
    let right_column = (0..height).map(|y| (Coord::new(width, y), WEST));

    top_row.chain(bottom_row).chain(left_column).chain(right_column)
        .map(|(coord, dir)| get_energized(&map, coord, dir))
        .max()
        .unwrap()
}

fn get_energized(map: &GridMap<char>, start: Coord, dir: Vec2D) -> usize {
    let mut visited:Visited = HashSet::new();
    follow_path(map, start, dir, &mut visited);

    let mut energized:HashSet<Coord> = HashSet::new();
    for visit in visited {        
        energized.insert(visit.0);
    }
    energized.len() 
}

fn follow_path(map: &GridMap<char>, prev: Coord, dir: Vec2D, visited: &mut Visited) {
    let cur = Coord::new(prev.x + dir.x, prev.y + dir.y);

    if let Some(char) = map.get(&cur) {
        if !visited.insert((cur.clone(), dir.clone())) {
            return;
        }

        match char {
            '|' => {
                if dir == EAST || dir == WEST {
                    follow_path(map, cur.clone(), NORTH, visited);
                    follow_path(map, cur, SOUTH, visited);
                } else {
                    follow_path(map, cur, dir, visited);
                }                
            },
            '-' => {
                if dir == NORTH || dir == SOUTH {
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
            '.' => follow_path(map, cur, dir, visited), // continue ahead
             _  => panic!("Invalid char at map {:?}, char: {}", cur, char),
        }
    }
}

// run these with cargo test --bin main -- day_13::tests
#[cfg(test)]
mod tests {
    use crate::utils::read_data_from_file;
    use super::*;   

    #[test]
    fn test_silver() {
        let test_data:Vec<String> = read_data_from_file("input/real/16.txt");
        assert_eq!(silver(&test_data), 7210);
    }

    #[test]
    fn test_gold() {
        let test_data:Vec<String> = read_data_from_file("input/real/16.txt");
        assert_eq!(gold(&test_data), 7673);
    }
}