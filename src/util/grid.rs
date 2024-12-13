use grid::Grid;

use super::point::Point;

// Workarounds for Grid crate's broken coordinate order
pub trait XyGrid<T> {
    fn get_xy(&self, x: i64, y: i64) -> Option<&T>;
    fn get_xy_mut(&mut self, x: i64, y: i64) -> Option<&mut T>;

    fn get_point(&self, p: Point) -> Option<&T>;
    fn get_point_mut(&mut self, p: Point) -> Option<&mut T>;
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
}
