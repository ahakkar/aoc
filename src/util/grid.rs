use super::point2::Point2;
use crate::util::direction::Direction;
use grid::Grid;
use std::{cmp::min, fmt::Display};

pub enum Orientation {
    Horizontal,
    Vertical,
}

// Workarounds for Grid crate's broken coordinate order
pub trait XyGrid<T> {
    fn get_xy(&self, x: i64, y: i64) -> Option<&T>;
    fn get_xy_mut(&mut self, x: i64, y: i64) -> Option<&mut T>;

    fn get_point(&self, p: Point2) -> Option<&T>;
    fn get_point_mut(&mut self, p: Point2) -> Option<&mut T>;

    // print below can't use the Grid's size for whatever reason
    fn size(&self) -> (usize, usize);
    fn print(&self)
    where
        T: Display,
    {
        let size = self.size();
        for row in 0..size.0 {
            for col in 0..size.1 {
                match self.get_xy(col as i64, row as i64) {
                    Some(val) => print!("{}", val),
                    None => print!("."),
                }
            }
            println!();
        }
    }

    fn get_neighbors(&self, x: usize, y: usize) -> Option<Vec<&T>>;

    fn draw_line(&mut self, a: &Point2, b: &Point2, tile: T)
    where
        T: Clone;
}

impl<T> XyGrid<T> for Grid<T> {
    fn get_xy(&self, x: i64, y: i64) -> Option<&T> {
        self.get(y, x)
    }

    fn get_xy_mut(&mut self, x: i64, y: i64) -> Option<&mut T> {
        self.get_mut(y, x)
    }

    fn get_point(&self, p: Point2) -> Option<&T> {
        self.get(p.y, p.x)
    }

    fn get_point_mut(&mut self, p: Point2) -> Option<&mut T> {
        self.get_mut(p.y, p.x)
    }

    fn size(&self) -> (usize, usize) {
        (self.rows(), self.cols())
    }

    fn get_neighbors(&self, x: usize, y: usize) -> Option<Vec<&T>> {
        let mut neighbors = vec![];

        for dir in Direction::DIRS {
            let check = dir.to_vector();
            let xn = x as i64 + check.x;
            let xy = y as i64 + check.y;
            if let Some(point) = self.get_xy(xn, xy) {
                neighbors.push(point);
            }
        }
        if !neighbors.is_empty() {
            return Some(neighbors);
        }
        None
    }

    fn draw_line(&mut self, a: &Point2, b: &Point2, tile: T)
    where
        T: Clone,
    {
        let o: Orientation;
        let dx = (a.x - b.x).abs();
        let dy = (a.y - b.y).abs();

        if dx == 0 && dy > 0 {
            o = Orientation::Vertical;
        } else if dx > 0 && dy == 0 {
            o = Orientation::Horizontal;
        } else {
            panic!("Unsupported orientation");
        }

        match o {
            Orientation::Horizontal => {
                let width = Point2::width(a, b);
                //println!("a: {:?}, b: {:?}, w: {}", a, b, width);
                let sx = min(a.x, b.x);
                for i in 0..width {
                    *self.get_xy_mut(sx + i as i64, b.y).unwrap() = tile.clone();
                }
            }
            Orientation::Vertical => {
                let height = Point2::height(a, b);
                //println!("a: {:?}, b: {:?}, w: {}", a, b, height);
                let sy = min(a.y, b.y);
                for i in 0..height {
                    *self.get_xy_mut(b.x, sy + i as i64).unwrap() = tile.clone();
                }
            }
        }
    }
}
