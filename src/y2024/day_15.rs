/*
 * 2024 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

#![allow(dead_code)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_assignments)]
#![allow(unused_must_use)]
#![allow(clippy::needless_return)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::only_used_in_recursion)]
#![allow(clippy::never_loop)]
#![allow(clippy::useless_vec)]
#![allow(clippy::while_immutable_condition)]

use grid::Grid;
use num_integer::Roots;
use util::grid::XyGrid;

use crate::{
    Fro, Solution, TaskResult,
    util::{
        self,
        point::{EAST, NORTH, Point, SOUTH, WEST},
    },
};

// Can add more shared vars here
pub struct WarehouseWoes {
    map: Grid<char>,
    commands: Vec<char>,
}

// Can be used to implement fancier task-specific parsing
impl Fro for WarehouseWoes {
    fn fro(input: &str) -> Self {
        let binding = input.replace("\r\n", "\n");
        let pair = binding.split_once("\n\n").unwrap();

        let map = Grid::from_vec(
            pair.0.replace("\n", "").chars().collect(),
            pair.0.len().sqrt(),
        );
        Self {
            map,
            commands: pair.1.replace("\n", "").chars().collect(),
        }
    }
}

// Main solvers
impl Solution for WarehouseWoes {
    fn silver(&self) -> TaskResult {
        let mut map = self.map.clone();
        let mut current = Point::new(-1, -1);

        // Find start
        for row in 0..map.rows() {
            for col in 0..map.cols() {
                if *map.get_xy(col as i64, row as i64).unwrap() == '@' {
                    current = Point::new(col as i64, row as i64);
                }
            }
        }

        for (i, c) in self.commands.iter().enumerate() {
            let (mut target, mut dir) = Self::get_dir(c, current);
            match map.get_point(target).unwrap() {
                '.' => current = Self::move_to(&mut map, &target, &current, '.', '@'),
                '#' => (),
                'O' => {
                    if Self::move_rocks(&mut map, &current, &dir) {
                        current = Self::move_to(&mut map, &target, &current, '.', '@')
                    }
                }
                _ => panic!("invalid char on map"),
            }
        }
        TaskResult::Usize(Self::rock_gps(&map))
    }

    fn gold(&self) -> TaskResult {
        // Just adding spaghetti over more spaghetti here
        let mut map: Grid<char> = Grid::init(self.map.rows(), self.map.cols() * 2, 'x');
        let mut current = Point::new(-1, -1);

        for row in 0..self.map.rows() {
            for col in 0..self.map.cols() {
                let c = *self.map.get_xy(col as i64, row as i64).unwrap();
                match c {
                    '@' => {
                        current = Point::new((col * 2) as i64, row as i64);
                        *map.get_xy_mut((col * 2) as i64, row as i64).unwrap() = c;
                        *map.get_xy_mut((col * 2 + 1) as i64, row as i64).unwrap() = '.';
                    }
                    'O' => {
                        *map.get_xy_mut((col * 2) as i64, row as i64).unwrap() = '[';
                        *map.get_xy_mut((col * 2 + 1) as i64, row as i64).unwrap() = ']';
                    }
                    _ => {
                        *map.get_xy_mut((col * 2) as i64, row as i64).unwrap() = c;
                        *map.get_xy_mut((col * 2 + 1) as i64, row as i64).unwrap() = c;
                    }
                }
            }
        }

        println!("initial stage, start: {:?}", current);
        map.print();

        for (i, c) in self.commands.iter().enumerate() {
            println!("move {}, cmd: {}", i + 1, c);

            let (mut target, mut dir) = Self::get_dir(c, current);

            match map.get_point(target).unwrap() {
                '.' => current = Self::move_to(&mut map, &target, &current, '.', '@'),
                '#' => (),
                '[' | ']' => {
                    if Self::move_gold(&mut map, &current, &dir) {
                        current = Self::move_to(&mut map, &target, &current, '.', '@')
                    }
                }
                _ => panic!("invalid char on map"),
            }
            if i > 19 {
                map.print();
            }

            if i > 21 {
                break;
            }
        }

        TaskResult::String("plaa".to_string())
    }
}

// For assisting functions
impl WarehouseWoes {
    fn move_gold(map: &mut Grid<char>, start: &Point, dir: &Point) -> bool {
        let mut pos = *start;
        //print!("moving rocks, start: {:?}", start);
        while let Some(idx) = map.get_point_mut(pos + *dir) {
            //println!(" looking at: {:?}", pos + *dir);
            match idx {
                '.' => {
                    // Move all rock halves recursively between this & start
                    // with north/south movement need to look at if stones are half blocked
                    // Perhaps try to get horizontal movement to work first
                    if (*dir == EAST || *dir == WEST) {
                        while pos != *start {
                            *map.get_point_mut(pos + *dir).unwrap() =
                                *map.get_point_mut(pos).unwrap();
                            pos = pos - *dir;
                        }
                        return true;
                    }
                    // Implement NORTH & SOUTH too
                    return false;
                }
                '#' => break,
                '[' | ']' => pos += *dir,
                _ => panic!("[Move_rocks]: Unexpected char {} at [{:?}", idx, pos),
            }
        }
        false
    }

    // Target, dir
    fn get_dir(c: &char, current: Point) -> (Point, Point) {
        match c {
            '<' => (current + WEST, WEST),

            '>' => (current + EAST, EAST),

            '^' => (current + NORTH, NORTH),

            'v' => (current + SOUTH, SOUTH),

            _ => panic!("invalid command"),
        }
    }
    fn rock_gps(map: &Grid<char>) -> usize {
        let mut sum = 0;
        for row in 0..map.rows() {
            for col in 0..map.cols() {
                if *map.get_xy(col as i64, row as i64).unwrap() == 'O' {
                    sum += 100 * row + col;
                }
            }
        }
        sum
    }
    fn move_to(
        map: &mut Grid<char>,
        target: &Point,
        current: &Point,
        c1: char,
        c2: char,
    ) -> Point {
        let mut p = map.get_point_mut(*current).unwrap();
        *p = c1;
        p = map.get_point_mut(*target).unwrap();
        *p = c2;
        *target
    }

    // Look at idx from start to dir. If a empty is found, switch rock to empty
    // From start. If wall or rock is found with no space between, abort.
    fn move_rocks(map: &mut Grid<char>, start: &Point, dir: &Point) -> bool {
        let mut pos = *start;
        while let Some(idx) = map.get_point_mut(pos + *dir) {
            //println!(" looking at: {:?}", pos + *dir);
            match idx {
                '.' => {
                    Self::move_to(map, start, &(pos + *dir), 'O', '.');
                    return true;
                }
                '#' => break,
                'O' => pos += *dir,
                _ => panic!("[Move_rocks]: Unexpected char {} at [{:?}", idx, pos),
            }
        }
        false
    }
}

// cargo test --lib day_XX
#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::utils::read_data_from_file;

    #[test]
    fn test() {
        let test_data = read_data_from_file("input/2024/test/15.txt");
        let queue = WarehouseWoes::fro(&test_data);

        assert_eq!(queue.silver(), TaskResult::Usize(10092));
        assert_eq!(queue.gold(), TaskResult::Usize(9021));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/2024/real/15.txt");
        let queue = WarehouseWoes::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(1475249));
        assert_eq!(queue.gold(), TaskResult::Usize(0));
    }
}
