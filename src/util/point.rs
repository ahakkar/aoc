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

pub const NORTH: Point = Point::new(0, -1);
pub const SOUTH: Point = Point::new(0, 1);
pub const EAST: Point = Point::new(1, 0);
pub const WEST: Point = Point::new(-1, 0);
pub const ORIGIN: Point = Point::new(0, 0);
pub const ORTHOGONAL: [Point; 4] = [NORTH, SOUTH, EAST, WEST];

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    #[inline]
    #[must_use]
    pub const fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}
