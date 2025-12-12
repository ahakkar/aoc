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
    ops::{Add, AddAssign, Sub},
};

use crate::util::direction::Direction;

pub const ORIGIN: Point2 = Point2::new(0, 0);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Point2 {
    pub x: i64,
    pub y: i64,
}

impl fmt::Display for Point2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Point2 {
    #[inline]
    #[must_use]
    pub const fn new(x: i64, y: i64) -> Self {
        Point2 { x, y }
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
            return Some(Point2 {
                x: success[0],
                y: success[1],
            });
        }
        None
    }

    pub fn step(self, dir: Direction) -> Self {
        self + dir.to_vector()
    }

    pub const fn square_area(a: &Point2, b: &Point2) -> usize {
        ((a.x - b.x + 1).abs() * (a.y - b.y + 1).abs()) as usize
    }

    pub const fn width(a: &Point2, b: &Point2) -> usize {
        (a.x - b.x).unsigned_abs() as usize + 1
    }

    pub const fn height(a: &Point2, b: &Point2) -> usize {
        (a.y - b.y).unsigned_abs() as usize + 1
    }
}

impl Add for Point2 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        Point2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Point2 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl Sub for Point2 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Point2::new(self.x - rhs.x, self.y - rhs.y)
    }
}
