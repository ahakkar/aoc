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

use std::collections::{HashMap, HashSet};

use crate::{
    utils::{self, Vec2D},
    Fro, Solution, TaskResult,
};
use grid::*;
use utils::{EAST, NORTH, SOUTH, WEST};

// Can add more shared vars here
pub struct HoofIt {
    map: Grid<u32>,
    rows: usize,
    cols: usize,
    dirs: Vec<Vec2D>,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct Point {
    x: isize,
    y: isize,
}

// Can be used to implement fancier task-specific parsing
impl Fro for HoofIt {
    fn fro(input: &str) -> Self {
        let mut map: Grid<u32> = Grid::from_vec(
            input
                .replace('\n', "")
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect(),
            (input.len() as f64).sqrt() as usize,
        );
        let dirs = Vec::from([NORTH, SOUTH, EAST, WEST]);
        Self {
            rows: map.rows(),
            cols: map.cols(),
            map,
            dirs,
        }
    }
}

// Main solvers
impl Solution for HoofIt {
    fn silver(&self) -> TaskResult {
        //                           x, y     , ends found
        let mut starts: HashMap<Point, HashSet<Point>> = HashMap::new();

        // Extract into helper function
        for row in 0..self.rows {
            for col in 0..self.cols {
                if *Self::get_value(self, col as isize, row as isize).unwrap()
                    == 0
                {
                    starts
                        .entry(Point {
                            x: col as isize,
                            y: row as isize,
                        })
                        .or_default();
                }
                /*                print!(
                    "{}",
                    Self::get_value(self, col as isize, row as isize).unwrap()
                ); */
            }
            //println!();
        }

        for mut start in &mut starts {
            Self::seek_silver_ends(self, start.0, start.1)
        }
        //println!("{:?}", starts);
        TaskResult::Usize(Self::sum_points_silver(starts))
    }

    fn gold(&self) -> TaskResult {
        //                           x, y     , ends found
        let mut starts: HashMap<Point, usize> = HashMap::new();

        // Extract into helper function
        for row in 0..self.rows {
            for col in 0..self.cols {
                if *Self::get_value(self, col as isize, row as isize).unwrap()
                    == 0
                {
                    starts
                        .entry(Point {
                            x: col as isize,
                            y: row as isize,
                        })
                        .or_insert(0);
                }
            }
        }

        for mut start in &mut starts {
            Self::seek_gold_ends(self, *start.0, start.1)
        }
        //println!("{:?}", starts);
        TaskResult::Usize(Self::sum_points_gold(starts))
    }
}

// For assisting functions
impl HoofIt {
    fn sum_points_silver(starts: HashMap<Point, HashSet<Point>>) -> usize {
        let mut sum = 0;
        for val in starts.values() {
            sum += val.len();
        }
        sum
    }

    fn sum_points_gold(starts: HashMap<Point, usize>) -> usize {
        starts.values().sum()
    }

    // Wrapper for Grid crate, it wants (row, col) = (y, x) for some reason
    fn get_value(&self, x: isize, y: isize) -> Option<&u32> {
        self.map.get(y, x)
    }

    fn seek_silver_ends(
        &self,
        current: &Point,
        start_value: &mut HashSet<Point>,
    ) {
        // End condition
        if *Self::get_value(self, current.x, current.y).unwrap() == 9 {
            start_value.insert(*current);
            return;
        }

        // Check for valid neighbours
        for dir in &self.dirs {
            let dx = current.x + dir.x;
            let dy = current.y + dir.y;

            if let Some(neighbor) = Self::get_value(self, dx, dy) {
                // Check for neighbours if their value is 1 higher
                if Self::get_value(self, current.x, current.y).unwrap() + 1
                    == *neighbor
                {
                    Self::seek_silver_ends(
                        self,
                        &Point { x: dx, y: dy },
                        start_value,
                    );
                }
            }
        }
    }

    fn seek_gold_ends(&self, current: Point, start_value: &mut usize) {
        // End condition
        if *Self::get_value(self, current.x, current.y).unwrap() == 9 {
            *start_value += 1;
            return;
        }

        // Check for valid neighbours
        for dir in &self.dirs {
            let dx = current.x + dir.x;
            let dy = current.y + dir.y;

            if let Some(neighbor) = Self::get_value(self, dx, dy) {
                // Check for neighbours if their value is 1 higher
                if Self::get_value(self, current.x, current.y).unwrap() + 1
                    == *neighbor
                {
                    Self::seek_gold_ends(
                        self,
                        Point { x: dx, y: dy },
                        start_value,
                    );
                }
            }
        }
    }
}

// cargo test --lib day_XX
#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_data_from_file;

    #[test]
    fn test() {
        let test_data = read_data_from_file("input/test/10.txt");
        let queue = HoofIt::fro(&test_data);

        assert_eq!(queue.silver(), TaskResult::Usize(36));
        assert_eq!(queue.gold(), TaskResult::Usize(0));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/real/10.txt");
        let queue = HoofIt::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(822));
        assert_eq!(queue.gold(), TaskResult::Usize(1801));
    }
}
