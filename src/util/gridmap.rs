/*
 * Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
 *
 * "Legacy" type which should be really retired and use Grid crate instead.
 * The implementation here is so small that using it is not a good idea.
 */

use crate::util::{direction::Direction, point2::Point2};

pub type Grid<T> = Vec<Vec<T>>;

#[derive(Clone)]
pub struct GridMap<T> {
    d: Grid<T>,
    w: usize,
    h: usize,
}

impl<T> GridMap<T>
where
    T: Copy,
{
    pub fn new(d: Grid<T>) -> GridMap<T> {
        let w = d[0].len();
        let h = d.len();
        GridMap { d, w, h }
    }

    pub fn get_cell(&self, xy: &Point2) -> Option<T> {
        if xy.x < self.w as i64 && xy.y < self.h as i64 && xy.x >= 0 && xy.y >= 0 {
            Some(self.d[xy.y as usize][xy.x as usize])
        } else {
            None
        }
    }

    pub fn get_idx(&self, x: usize, y: usize) -> Option<&T> {
        if x < self.w && y < self.h {
            Some(&self.d[y][x])
        } else {
            None
        }
    }

    pub fn get_idx_ub(&self, x: isize, y: isize) -> Option<&T> {
        if (x >= 0 && y >= 0) && (x < self.w as isize && y < self.h as isize) {
            Some(&self.d[y as usize][x as usize])
        } else {
            None
        }
    }

    pub fn get_neighbor(&self, idx: &Point2, dir: Direction) -> Option<T> {
        let (dx, dy) = match dir {
            Direction::East => (1, 0),
            Direction::North => (0, -1),
            Direction::West => (-1, 0),
            Direction::South => (0, -1),
            Direction::NorthEast => (1, 1),
            Direction::NorthWest => (-1, 1),
            Direction::SouthEast => (1, -1),
            Direction::SouthWest => (-1, -1),
            _ => panic!("invalid direction"),
        };

        if idx.x + dx >= 0 && idx.y + dy >= 0 {
            return self.get_cell(&Point2::new(idx.x + dx, idx.y + dy));
        }
        None
    }

    pub fn get_data(&self) -> &Grid<T> {
        &self.d
    }
    pub fn get_height(&self) -> usize {
        self.h
    }
    pub fn get_width(&self) -> usize {
        self.w
    }
}

pub fn data_as_chars(data: &[String]) -> Grid<char> {
    let mut data_as_chars: Grid<char> = vec![];
    for row in data {
        data_as_chars.push(row.chars().collect::<Vec<char>>());
    }
    data_as_chars
}
