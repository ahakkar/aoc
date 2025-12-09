/*
 * 2025 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

use std::{cell::RefCell, collections::HashSet};

use crate::{
    Fro, Solution, TaskResult,
    util::{grid::XyGrid, point::Point},
};
use geo::{Contains, LineString, Polygon};
use geo::{Coord, Rect};
use grid::Grid;
use itertools::Itertools;
//use rayon::prelude::*;

// Can add more shared vars here
pub struct MovieTheater {
    points: Vec<Point>,
    /*     min_x: RefCell<i64>,
    max_x: RefCell<i64>,
    min_y: RefCell<i64>,
    max_y: RefCell<i64>, */
    largest: RefCell<BestRectangle>,
}

// Can be used to implement fancier task-specific parsing
impl Fro for MovieTheater {
    fn fro(input: &str) -> Self {
        Self {
            // Discards bad input silently
            points: input.split('\n').filter_map(Point::new_from_str).collect(),
            /*             min_x: RefCell::new(i64::MAX),
            min_y: RefCell::new(i64::MAX),
            max_x: RefCell::new(0),
            max_y: RefCell::new(0), */
            largest: RefCell::new(BestRectangle::new()),
        }
    }
}

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

// Main solvers
impl Solution for MovieTheater {
    fn silver(&self) -> TaskResult {
        for i in 0..self.points.len() {
            for j in i + 1..self.points.len() {
                /*                 if self.points[i].x > *self.max_x.borrow() {
                    *self.max_x.borrow_mut() = self.points[i].x;
                }
                if self.points[i].y > *self.max_y.borrow() {
                    *self.max_y.borrow_mut() = self.points[i].y;
                }
                if self.points[i].x < *self.min_x.borrow() {
                    *self.min_x.borrow_mut() = self.points[i].x;
                }
                if self.points[i].y < *self.min_y.borrow() {
                    *self.min_y.borrow_mut() = self.points[i].y;
                } */

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

        TaskResult::Usize(self.largest.borrow().area)
    }

    fn gold(&self) -> TaskResult {
        let (xs, ys) = self.compress_coordinates();

        let outer: LineString<f64> = self
            .points
            .iter()
            .map(|p| (p.x as f64, p.y as f64))
            .collect::<LineString<f64>>();

        let poly: Polygon<f64> = Polygon::new(outer, vec![]);
        let allowed_points: HashSet<(i64, i64)> =
            self.points.iter().map(|p| (p.x, p.y)).collect();

        let mut best: Option<((i64, i64, i64, i64), i64)> = None;

        for i in 0..xs.len() {
            for k in (i + 1)..xs.len() {
                let x0 = xs[i];
                let x1 = xs[k];

                for j in 0..ys.len() {
                    for l in (j + 1)..ys.len() {
                        let y0 = ys[j];
                        let y1 = ys[l];

                        // Reject if rect isn't formed by diags in the points list
                        let bottom_left = (x0, y0);
                        let top_right = (x1, y1);

                        let bottom_right = (x1, y0);
                        let top_left = (x0, y1);

                        let diagonal1_ok = allowed_points.contains(&bottom_left)
                            && allowed_points.contains(&top_right);
                        let diagonal2_ok = allowed_points.contains(&bottom_right)
                            && allowed_points.contains(&top_left);

                        if !diagonal1_ok && !diagonal2_ok {
                            continue;
                        }

                        let rect_poly = self.rect_from_coords(x0, x1, y0, y1);

                        if poly.contains(&rect_poly) {
                            let area = (x1 - x0 + 1) * (y1 - y0 + 1);

                            if best.is_none_or(|(_, best_area)| area > best_area) {
                                best = Some(((x0, x1, y0, y1), area));
                            }
                        }
                    }
                }
            }
        }

        if let Some(results) = best {
            return TaskResult::Usize(results.1 as usize);
        }

        0.into()
    }
}

// For assisting functions
impl MovieTheater {
    fn rect_from_coords(&self, x0: i64, x1: i64, y0: i64, y1: i64) -> Polygon<f64> {
        let rect = Rect::new(
            Coord {
                x: x0 as f64,
                y: y0 as f64,
            },
            Coord {
                x: x1 as f64,
                y: y1 as f64,
            },
        );
        rect.to_polygon()
    }

    /*
    edges = []
    for i in 0 .. points.length-1:
        a = points[i]
        b = points[(i+1) % points.length]   // wrap around
        edges.append((a, b)) */

    // Not used
    fn _build_edges(&self) -> Vec<(Point, Point)> {
        let mut edges: Vec<(Point, Point)> = vec![];
        for i in 0..self.points.len() {
            let a = self.points[i];
            let b = self.points[(i + 1) % self.points.len()];
            edges.push((a, b));
        }
        edges
    }

    // In English: build lists of sorted unique coordinates, removing doubles
    fn compress_coordinates(&self) -> (Vec<i64>, Vec<i64>) {
        let mut xs: HashSet<i64> = HashSet::new();
        let mut ys: HashSet<i64> = HashSet::new();
        for p in &self.points {
            xs.insert(p.x);
            ys.insert(p.y);
        }
        (
            xs.into_iter().sorted_unstable().collect::<Vec<i64>>(),
            ys.into_iter().sorted_unstable().collect::<Vec<i64>>(),
        )
    }

    // Not used
    fn _trace_edges_to_grid(&self, grid: &mut Grid<char>, tile: char) {
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

        assert_eq!(queue.silver(), TaskResult::Usize(4740155680));
        assert_eq!(queue.gold(), TaskResult::Usize(1543501936));
    }
}
