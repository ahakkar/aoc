//!
//! The #[inline] attribute is a hint to the compiler that the function
//! should be inlined if possible. Inlining means that instead of making a
//! function call, the compiler copies the function’s code directly into
//! the caller's code. This can reduce function call overhead but may
//! increase binary size.
//!
//! The #[must_use] attribute indicates that the result of the function
//! should not be ignored. If a value returned by a function marked #
//! [must_use] is not used, the compiler emits a warning.
//!

use std::{
    fmt,
    ops::{Add, Sub},
};

pub const ORIGIN: Point = Point::new(0, 0);

pub const NORTH: Point = Point::new(0, -1);
pub const SOUTH: Point = Point::new(0, 1);
pub const EAST: Point = Point::new(1, 0);
pub const WEST: Point = Point::new(-1, 0);

pub const NORTHEAST: Point = Point::new(1, -1);
pub const NORTHWEST: Point = Point::new(-1, -1);
pub const SOUTHEAST: Point = Point::new(1, 1);
pub const SOUTHWEST: Point = Point::new(-1, 1);

pub const ORTHOGONAL: [Point; 4] = [NORTH, SOUTH, EAST, WEST];
pub const DIAGONAL: [Point; 4] = [NORTHEAST, NORTHWEST, SOUTHEAST, SOUTHWEST];

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Point {
    #[inline]
    #[must_use]
    pub const fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }

    #[inline]
    #[must_use]
    pub fn new_from_str(str: &str) -> Option<Self> {
        let v: Option<Vec<i64>> = str
            .trim()
            .split(',')
            .map(|s| s.parse::<i64>().ok())
            .collect();

        if let Some(success) = v {
            return Some(Point {
                x: success[0],
                y: success[1],
            });
        }
        None
    }

    pub const fn square_area(a: &Point, b: &Point) -> usize {
        ((a.x - b.x + 1).abs() * (a.y - b.y + 1).abs()) as usize
    }

    pub const fn width(a: &Point, b: &Point) -> usize {
        (a.x - b.x).unsigned_abs() as usize + 1
    }

    pub const fn height(a: &Point, b: &Point) -> usize {
        (a.y - b.y).unsigned_abs() as usize + 1
    }
}

impl Add for Point {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Point {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}
