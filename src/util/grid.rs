use std::fmt::Display;

use grid::Grid;

use crate::util::utils::Direction;

use super::point::Point;

const DIRS: [Direction; 8] = [
    Direction::NorthWest,
    Direction::North,
    Direction::NorthEast,
    Direction::East,
    Direction::SouthEast,
    Direction::South,
    Direction::SouthWest,
    Direction::West,
];

// Workarounds for Grid crate's broken coordinate order
pub trait XyGrid<T> {
    fn get_xy(&self, x: i64, y: i64) -> Option<&T>;
    fn get_xy_mut(&mut self, x: i64, y: i64) -> Option<&mut T>;

    fn get_point(&self, p: Point) -> Option<&T>;
    fn get_point_mut(&mut self, p: Point) -> Option<&mut T>;

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
}

impl<T> XyGrid<T> for Grid<T> {
    fn get_xy(&self, x: i64, y: i64) -> Option<&T> {
        self.get(y, x)
    }

    fn get_xy_mut(&mut self, x: i64, y: i64) -> Option<&mut T> {
        self.get_mut(y, x)
    }

    fn get_point(&self, p: Point) -> Option<&T> {
        self.get(p.y, p.x)
    }

    fn get_point_mut(&mut self, p: Point) -> Option<&mut T> {
        self.get_mut(p.y, p.x)
    }

    fn size(&self) -> (usize, usize) {
        (self.rows(), self.cols())
    }

    fn get_neighbors(&self, x: usize, y: usize) -> Option<Vec<&T>> {
        let mut neighbors = vec![];

        for dir in DIRS {
            let check = dir.to_vector();
            let xn = x as i64 + check.0 as i64;
            let xy = y as i64 + check.1 as i64;
            if let Some(point) = self.get_xy(xn, xy) {
                neighbors.push(point);
            }
        }
        if !neighbors.is_empty() {
            return Some(neighbors);
        }
        None
    }
}
