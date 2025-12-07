/*
 * 2025 Advent of Code with Rust
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
#![allow(clippy::collapsible_if)]

use std::{cell::RefCell, collections::HashSet};

use crate::{
    Fro, Solution, TaskResult,
    util::{self, point::Point},
};
use grid::Grid;
use num_integer::sqrt;
use util::grid::XyGrid;
use util::point::{EAST, SOUTH, WEST};

// Can add more shared vars here
pub struct Laboratories {
    grid: Grid<char>,
    silver: RefCell<usize>,
    gold: RefCell<usize>,
    grid_size: usize,
    start: Point,
    visited: RefCell<HashSet<Point>>,
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
            gold: RefCell::new(0),
            grid_size,
            start: Point::new((grid_size / 2 + 1) as i64, 0),
            visited: RefCell::new(HashSet::new()),
        }
    }
}

// Main solvers
impl Solution for Laboratories {
    fn silver(&self) -> TaskResult {
        //println!("{:?}", self.grid.print());
        println!("width:{}, start:{:?}", self.grid_size, self.start);
        self.trace_ray(&self.start);

        // 1501 too low
        TaskResult::Usize(*self.silver.borrow())
    }

    fn gold(&self) -> TaskResult {
        TaskResult::String("plaa".to_string())
    }
}

// For assisting functions
impl Laboratories {
    fn trace_ray(&self, current: &Point) {
        let next: Point = *current + SOUTH;
        if let Some(cell) = self.grid.get_point(next) {
            // Ray has visited current cell
            self.visited.borrow_mut().insert(*current);

            // println!("at cell: {:?}, next: {:?}", current, next);
            // Divide or continue ray
            match cell {
                '^' => {
                    let left = *current + WEST;
                    let right = *current + EAST;

                    // Left not visited
                    if self.visited.borrow().get(&left).is_none() {
                        *self.silver.borrow_mut() += 1;
                        self.visited.borrow_mut().insert(left);
                        self.trace_ray(&(*current + WEST));
                    }
                    // Right not visited
                    if self.visited.borrow().get(&right).is_none() {
                        *self.silver.borrow_mut() += 1;
                        self.visited.borrow_mut().insert(right);
                        self.trace_ray(&(*current + EAST));
                    }
                }
                '.' => {
                    self.trace_ray(&next);
                }
                _ => panic!("Unsupported cell type"),
            }
        }
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
        assert_eq!(queue.gold(), TaskResult::Usize(0));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/2025/real/07.txt");
        let queue = Laboratories::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(0));
        assert_eq!(queue.gold(), TaskResult::Usize(0));
    }
}
