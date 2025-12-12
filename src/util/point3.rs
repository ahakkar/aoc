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

use libm;
use num_integer::sqrt;
use std::ops::{Add, Sub};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Point3 {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Point3 {
    // d(p,q)=sqrt (p1-q1)^2+(p2-q2)^2+(p3-q3)^2
    pub fn euclid_floor(p: Point3, q: Point3) -> i64 {
        sqrt((p.x - q.x).pow(2) + (p.y - q.y).pow(2) + (p.z - q.z).pow(2))
    }

    pub fn euclid(p: Point3, q: Point3) -> f64 {
        libm::sqrt(((p.x - q.x).pow(2) + (p.y - q.y).pow(2) + (p.z - q.z).pow(2)) as f64)
    }

    pub fn squared_distance(p: Point3, q: Point3) -> i64 {
        (p.x - q.x).pow(2) + (p.y - q.y).pow(2) + (p.z - q.z).pow(2)
    }
}

impl Point3 {
    #[inline]
    #[must_use]
    pub const fn new(x: i64, y: i64, z: i64) -> Self {
        Point3 { x, y, z }
    }

    /// Parses a comma-separated list of values to Point3d.
    ///
    /// The input must be a string in the form `"a,b,c"`
    ///
    /// # Examples
    ///
    /// ```
    /// let items = new_from_str("one,two,three");
    /// ```
    #[inline]
    #[must_use]
    pub fn new_from_str(str: &str) -> Option<Self> {
        let v: Option<Vec<i64>> = str
            .trim()
            .split(',')
            .map(|s| s.parse::<i64>().ok())
            .collect();

        if let Some(success) = v {
            return Some(Point3 {
                x: success[0],
                y: success[1],
                z: success[2],
            });
        }
        None
    }
}

impl Add for Point3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        Point3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Point3 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Point3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}
