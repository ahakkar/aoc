use std::fmt::Display;

use grid::Grid;

use super::point::Point;

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
}
