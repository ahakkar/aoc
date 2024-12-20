/*
 * 2024 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

use num_integer::lcm;
use regex::Regex;

use crate::{util::point::Point, Fro, Solution, TaskResult};
use pathfinding::prelude::dijkstra;

// Can add more shared vars here
pub struct ClawContraption {
    data: Vec<Route>,
}

#[derive(Debug)]
struct Route {
    a: Point,
    b: Point,
    p: Point,
}

// Can be used to implement fancier task-specific parsing
impl Fro for ClawContraption {
    fn fro(input: &str) -> Self {
        let re = Regex::new(r"\D*(\d+)\D*(\d+)").unwrap();
        Self {
            data: input
                .replace("\r\n", "\n")
                .split("\n\n")
                .map(|line| {
                    let result: Vec<&str> = line.split('\n').collect();

                    if let [a, b, prize] = &result[..] {
                        Route {
                            a: Self::extract_point(a, &re),
                            b: Self::extract_point(b, &re),
                            p: Self::extract_point(prize, &re),
                        }
                    } else {
                        panic!("Expected exactly three lines!");
                    }
                })
                .collect(),
        }
    }
}

// Main solvers
impl Solution for ClawContraption {
    // This is too slow for part Bc
    fn silver(&self) -> TaskResult {
        let a_weight = 3;
        let b_weight = 1;
        let mut sum = 0;

        for route in &self.data {
            //println!("{:?}", route);
            let x_target = route.p.x;
            let y_target = route.p.y;

            let moves = Vec::from([(route.a, a_weight), (route.b, b_weight)]);

            let path = dijkstra(
                &(0, 0, 0), // Start state
                |&(x, y, w)| {
                    // Generate neighbors (possible next states)
                    moves.iter().filter_map(move |&(r, dw)| {
                        let x_new = x + r.x;
                        let y_new = y + r.y;
                        let w_new = w + dw;

                        if x_new <= x_target && y_new <= y_target {
                            Some(((x_new, y_new, w_new), dw))
                        } else {
                            None
                        }
                    })
                },
                |&(x, y, _)| x == x_target && y == y_target, // Goal condition
            );

            // Save only the path's weight
            if let Some(p) = &path {
                sum += p.1;
            }
        }
        TaskResult::Usize(sum)
    }

    // A mathematical problem for a person with no mathematical skills
    // The solution is implemented based on MrQueasy's Kotlin solution
    // Includes using linear algebra and Cramer's rule
    fn gold(&self) -> TaskResult {
        let a_weight = 3;
        let mut sum = 0;

        for route in &self.data {
            let m = lcm(route.a.x, route.a.y);
            let xm = (m as f64) / (route.a.x as f64);
            let ym = (m as f64) / (route.a.y as f64);
            let px = (route.p.x + 10_000_000_000_000) as f64;
            let py = (route.p.y + 10_000_000_000_000) as f64;

            let b =
                (px * xm - py * ym) / ((route.b.x as f64) * xm - (route.b.y as f64) * ym);
            let a = (px - b * (route.b.x as f64)) / (route.a.x as f64);

            if a.fract() == 0.0 && b.fract() == 0.0 {
                sum += (a_weight as f64 * a + b) as i64;
            }
        }

        TaskResult::Usize(sum as usize)
    }
}

// For assisting functions
impl ClawContraption {
    fn extract_point(str: &str, re: &Regex) -> Point {
        let c = re.captures(str).unwrap();
        Point::new(
            c.get(1).unwrap().as_str().parse::<i64>().unwrap(),
            c.get(2).unwrap().as_str().parse::<i64>().unwrap(),
        )
    }
}

// cargo test --lib day_XX
#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::utils::read_data_from_file;

    #[test]
    fn test() {
        let test_data = read_data_from_file("input/2024/test/13.txt");
        let queue = ClawContraption::fro(&test_data);

        assert_eq!(queue.silver(), TaskResult::Usize(480));
        assert_eq!(queue.gold(), TaskResult::Usize(0));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/2024/real/13.txt");
        let queue = ClawContraption::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(35574));
        assert_eq!(queue.gold(), TaskResult::Usize(80882098756071));
    }
}
