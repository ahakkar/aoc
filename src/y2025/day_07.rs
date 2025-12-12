/*
 * 2025 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

#![allow(dead_code)]

use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
};

use crate::{
    Fro, Solution, TaskResult,
    util::{self, point2::Point2},
};
use grid::Grid;
use num_integer::sqrt;
use util::grid::XyGrid;
use util::point2::{EAST, SOUTH, WEST};

// Can add more shared vars here
pub struct Laboratories {
    grid: Grid<char>,
    silver: RefCell<usize>,
    start: Point2,
    visited: RefCell<HashSet<Point2>>,
}

// Can be used to implement fancier task-specific parsing
impl Fro for Laboratories {
    fn fro(input: &str) -> Self {
        let data: Vec<char> = input.chars().filter(|c| *c != '\n').collect();
        // Grid is not actually perfectly square but this works well enough
        let grid_size = sqrt(data.len());

        Self {
            grid: Grid::from_vec(data, grid_size),
            silver: RefCell::new(0),
            start: Point2::new((grid_size / 2) as i64, 0),
            visited: RefCell::new(HashSet::new()),
        }
    }
}

// Main solvers
impl Solution for Laboratories {
    fn silver(&self) -> TaskResult {
        self.trace_ray(&(self.start + SOUTH));
        TaskResult::Usize(*self.silver.borrow())
    }

    fn gold(&self) -> TaskResult {
        self.count_paths(&self.start, &mut HashMap::new()).into()
    }
}

// For assisting functions
impl Laboratories {
    fn trace_ray(&self, current: &Point2) {
        // Have we visited this cell already?
        if self.visited.borrow().contains(current) {
            return;
        }

        // Are we in a cell at all?
        if let Some(cell) = self.grid.get_point(*current) {
            // Current cell exists and ray has visited it
            self.visited.borrow_mut().insert(*current);

            // Divide or continue ray
            match cell {
                '^' => {
                    *self.silver.borrow_mut() += 1;
                    self.trace_ray(&(*current + WEST));
                    self.trace_ray(&(*current + EAST));
                }
                '.' => {
                    self.trace_ray(&(*current + SOUTH));
                }
                _ => panic!("Unsupported cell type"),
            }
        }
    }

    fn count_paths(&self, current: &Point2, cache: &mut HashMap<Point2, usize>) -> usize {
        if self.grid.get_point(*current + SOUTH).is_none() {
            return 1;
        }

        if let Some(&result) = cache.get(current) {
            return result;
        }

        // Peek at the cell below
        let below = *current + SOUTH;
        let cell_below = self.grid.get_point(below).unwrap();

        // Splitter, go from the splitter cell left or right
        let result = if *cell_below == '^' {
            let left = self.count_paths(&(below + WEST), cache);
            let right = self.count_paths(&(below + EAST), cache);
            left + right
        } else {
            self.count_paths(&below, cache)
        };

        cache.insert(*current, result);
        result
    }

    fn print_ray(&self) {
        let mut map: Grid<char> = self.grid.clone();
        for point in self.visited.borrow().iter() {
            // Dont overlap splitters with beam gfx
            if self.grid.get_point(*point).unwrap() != &'^' {
                *map.get_point_mut(*point).unwrap() = '|';
            }
        }

        map.print();
        println!();
    }
}

// cargo test --lib day_XX
#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::utils::read_data_from_file;

    #[test]
    fn test() {
        let test_data = read_data_from_file("input/2025/test/07.txt");
        let queue = Laboratories::fro(&test_data);

        assert_eq!(queue.silver(), TaskResult::Usize(21));
        assert_eq!(queue.gold(), TaskResult::Usize(40));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/2025/real/07.txt");
        let queue = Laboratories::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(1594));
        assert_eq!(queue.gold(), TaskResult::Usize(15650261281478));
    }
}
