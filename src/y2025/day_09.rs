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

use std::cell::RefCell;

use crate::{
    Fro, Solution, TaskResult,
    util::{grid::XyGrid, point::Point, utils::Orientation},
};
use grid::Grid;

#[derive(Debug)]
struct BestRectangle {
    area: usize,
    a: Point,
    b: Point,
}

impl BestRectangle {
    fn new() -> Self {
        Self {
            area: 0,
            a: Point::new(0, 0),
            b: Point::new(0, 0),
        }
    }

    fn update(&mut self, area: usize, a: Point, b: Point) {
        if area > self.area {
            self.area = area;
            self.a = a;
            self.b = b;
        }
    }
}

// Can add more shared vars here
pub struct MovieTheater {
    points: Vec<Point>,
    max_x: RefCell<i64>,
    max_y: RefCell<i64>,
    largest: RefCell<BestRectangle>,
}

// Can be used to implement fancier task-specific parsing
impl Fro for MovieTheater {
    fn fro(input: &str) -> Self {
        Self {
            // Discards bad input silently
            points: input.split('\n').filter_map(Point::new_from_str).collect(),
            max_x: RefCell::new(0),
            max_y: RefCell::new(0),
            largest: RefCell::new(BestRectangle::new()),
        }
    }
}

// Main solvers
impl Solution for MovieTheater {
    fn silver(&self) -> TaskResult {
        for i in 0..self.points.len() {
            for j in i + 1..self.points.len() {
                if self.points[i].x > *self.max_x.borrow() {
                    *self.max_x.borrow_mut() = self.points[i].x;
                }
                if self.points[i].y > *self.max_y.borrow() {
                    *self.max_y.borrow_mut() = self.points[i].y;
                }

                let area = Point::square_area(&self.points[i], &self.points[j]);
                if area > self.largest.borrow().area {
                    self.largest.borrow_mut().update(
                        area,
                        self.points[i],
                        self.points[j],
                    );
                }
            }
        }
        //println!("min_x: {:?}, max_y: {:?}", self.min_x, self.max_y);

        TaskResult::String("plii".to_string())
    }

    fn gold(&self) -> TaskResult {
        // Computed during silver
        println!("{:?}", self.largest);
        let a = &self.largest.borrow().a;
        let b = &self.largest.borrow().b;
        let height: usize = (*self.max_y.borrow()).try_into().unwrap();
        let width: usize = (*self.max_x.borrow()).try_into().unwrap();

        println!("w: {:?}, h: {:?}", width, height);

        let mut grid: Grid<char> = Grid::init(height + 1, width + 1, '.');

        self.trace_edges_to_grid(&mut grid, '#');

        TaskResult::String("plaa".to_string())
    }
}

// For assisting functions
impl MovieTheater {
    fn trace_edges_to_grid(&self, grid: &mut Grid<char>, tile: char) {
        for i in 1..self.points.len() {
            let current = self.points[i];
            let prev = self.points[i - 1];

            grid.draw_line(&current, &prev, '#');
        }

        if self.points.len() < 2 {
            return;
        }

        for pair in self.points.windows(2) {
            grid.draw_line(&pair[0], &pair[1], tile);
        }

        // close loop
        grid.draw_line(&self.points[self.points.len() - 1], &self.points[0], tile);

        grid.print();
    }
}

// cargo test --lib day_XX
#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::utils::read_data_from_file;

    #[test]
    fn test() {
        let test_data = read_data_from_file("input/2025/test/09.txt");
        let queue = MovieTheater::fro(&test_data);

        assert_eq!(queue.silver(), TaskResult::Usize(50));
        assert_eq!(queue.gold(), TaskResult::Usize(24));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/2025/real/09.txt");
        let queue = MovieTheater::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(0));
        assert_eq!(queue.gold(), TaskResult::Usize(0));
    }
}
