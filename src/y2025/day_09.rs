/*
 * 2025 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

use std::collections::HashSet;

use crate::{
    Fro, Solution, TaskResult,
    util::{grid::XyGrid, point::Point},
};
use geo::{Contains, LineString, Polygon};
use geo::{Coord, Rect};
use grid::Grid;
use itertools::Itertools;
use rayon::prelude::*;

// Can add more shared vars here
pub struct MovieTheater {
    points: Vec<Point>,
    allowed_points: HashSet<(i64, i64)>,
}

// Can be used to implement fancier task-specific parsing
impl Fro for MovieTheater {
    fn fro(input: &str) -> Self {
        // Discards bad input silently
        let points: Vec<Point> =
            input.split('\n').filter_map(Point::new_from_str).collect();

        Self {
            allowed_points: points.iter().map(|p| (p.x, p.y)).collect(),
            points,
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
        let mut largest: BestRectangle = BestRectangle::new();

        for i in 0..self.points.len() {
            for j in i + 1..self.points.len() {
                let area = Point::square_area(&self.points[i], &self.points[j]);
                if area > largest.area {
                    largest.update(area, self.points[i], self.points[j]);
                }
            }
        }

        TaskResult::Usize(largest.area)
    }

    fn gold(&self) -> TaskResult {
        let outer_points: LineString<f64> = self
            .points
            .iter()
            .map(|p| (p.x as f64, p.y as f64))
            .collect::<LineString<f64>>();

        let poly: Polygon<f64> = Polygon::new(outer_points, vec![]);
        let (xs, ys) = self.compress_coordinates();

        let best_area = (0..xs.len())
            .into_par_iter()
            .map(|i| {
                let mut thread_best = 0;

                for k in (i + 1)..xs.len() {
                    let x0 = xs[i];
                    let x1 = xs[k];

                    for j in 0..ys.len() {
                        for l in (j + 1)..ys.len() {
                            let y0 = ys[j];
                            let y1 = ys[l];

                            if !self.is_valid_rect(&x0, &x1, &y0, &y1) {
                                continue;
                            }

                            let rect_poly = self.rect_from_coords(x0, x1, y0, y1);

                            if poly.contains(&rect_poly) {
                                let area = (x1 - x0 + 1) * (y1 - y0 + 1);
                                if area > thread_best {
                                    thread_best = area;
                                }
                            }
                        }
                    }
                }

                thread_best
            })
            .reduce(|| 0, |a, b| a.max(b));

        TaskResult::Usize(best_area as usize)
    }
}

// For assisting functions
impl MovieTheater {
    // Reject if rect isn't formed by diags in the points list
    fn is_valid_rect(&self, x0: &i64, x1: &i64, y0: &i64, y1: &i64) -> bool {
        let diagonal1_ok = self.allowed_points.contains(&(*x0, *y0))
            && self.allowed_points.contains(&(*x1, *y1));
        let diagonal2_ok = self.allowed_points.contains(&(*x1, *y0))
            && self.allowed_points.contains(&(*x0, *y1));

        if !diagonal1_ok && !diagonal2_ok {
            return false;
        }
        true
    }

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
