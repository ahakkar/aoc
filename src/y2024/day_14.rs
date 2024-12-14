/*
 * 2024 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

use crate::{
    util::{point::Point, regex::MyCaptures},
    Fro, Solution, TaskResult,
};
use regex::Regex;
use std::collections::HashSet;

pub struct RestroomRedoubt {
    data: Vec<(Point, Point)>,
}

impl Fro for RestroomRedoubt {
    fn fro(input: &str) -> Self {
        let re = Regex::new(r"p=(\d+),(\d+)\s+v=(-?\d+),(-?\d+)").unwrap();
        let parse_points = |str| -> (Point, Point) {
            let c = re.captures(str).unwrap();
            (
                Point::new(c.get_i64(1), c.get_i64(2)),
                Point::new(c.get_i64(3), c.get_i64(4)),
            )
        };

        Self {
            data: input.split('\n').map(parse_points).collect(),
        }
    }
}

impl Solution for RestroomRedoubt {
    fn silver(&self) -> TaskResult {
        let mut end: Vec<usize> = vec![0; 4];

        let w = 101; // width
        let h = 103; // height
        let n = 100; // time secs

        // xn = ((x0 + vx * n ) mod w)
        // xy = ((y0 + vy * n ) mod h)
        for pair in &self.data {
            let x0 = pair.0.x;
            let y0 = pair.0.y;
            let vx = pair.1.x;
            let vy = pair.1.y;

            let xn = Self::modulo(x0 + vx * n, w);
            let yn = Self::modulo(y0 + vy * n, h);

            // Ignore middle row
            if xn != (w - 1) / 2 && yn != (h - 1) / 2 {
                // Match quadrant
                match (xn < w / 2, yn < h / 2) {
                    (true, true) => end[0] += 1,   // NW
                    (false, true) => end[1] += 1,  // NE
                    (true, false) => end[2] += 1,  // SW
                    (false, false) => end[3] += 1, // SE
                }
            }
        }
        TaskResult::Usize(end.iter().product::<usize>())
    }

    fn gold(&self) -> TaskResult {
        let w = 101; // width
        let h = 103; // height
        let mut set = HashSet::new();

        // Try to fish for a loop where all objects are in diff pos
        let mut n = 0;
        loop {
            set.clear();
            //test_results.clear();
            let mut success = true;
            for pair in &self.data {
                let x0 = pair.0.x;
                let y0 = pair.0.y;
                let vx = pair.1.x;
                let vy = pair.1.y;

                let xn = Self::modulo(x0 + vx * n, w);
                let yn = Self::modulo(y0 + vy * n, h);

                // Break outer loop too
                if !set.insert(Point::new(xn, yn)) {
                    success = false;
                    break;
                }
            }
            if success {
                break;
            }
            n += 1;
        }
        TaskResult::Usize(n as usize)
    }
}

impl RestroomRedoubt {
    // TODO Should initialize a w,h char map with . and just change the
    // value of the cell for each object
    fn _build_map(tr: &[Point], w: usize, h: usize) {
        for row in 0..h {
            for col in 0..w {
                let mut obj: usize = 0;
                for p in tr {
                    if p.x as usize == col && p.y as usize == row {
                        obj += 1;
                    }
                }
                match obj > 0 {
                    true => print!("{}", obj),
                    false => print!("."),
                }
            }
            println!();
        }
    }

    fn modulo(a: i64, n: i64) -> i64 {
        ((a % n) + n) % n
    }
}

// cargo test --lib day_XX
#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::utils::read_data_from_file;

    #[test]
    fn modulo() {
        assert_eq!(5 % 3, 2);
        assert_eq!(5 % -3, 2);
        assert_eq!(-5 % 3, -2);
        assert_eq!(-5 % -3, -2);

        // Modulo
        assert_eq!(RestroomRedoubt::modulo(5, 3), 2);
        assert_eq!(RestroomRedoubt::modulo(5, -3), -1);
        assert_eq!(RestroomRedoubt::modulo(-5, 3), 1);
        assert_eq!(RestroomRedoubt::modulo(-5, -3), -2);

        assert_eq!(RestroomRedoubt::modulo(106, 11), 7);
        assert_eq!(RestroomRedoubt::modulo(-297, 7), 4);
        assert_eq!(RestroomRedoubt::modulo(300, 11), 3);
        assert_eq!(RestroomRedoubt::modulo(-296, 7), 5);
    }

    #[test]
    fn test() {
        let test_data = read_data_from_file("input/2024/test/14.txt");
        let queue = RestroomRedoubt::fro(&test_data);

        assert_eq!(queue.silver(), TaskResult::Usize(12));
        assert_eq!(queue.gold(), TaskResult::Usize(0));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/2024/real/14.txt");
        let queue = RestroomRedoubt::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(214109808));
        assert_eq!(queue.gold(), TaskResult::Usize(7687));
    }
}
