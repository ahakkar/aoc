/*
 * 2025 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

use crate::{Fro, Solution, TaskResult};

struct Box {
    l: usize,
    w: usize,
    h: usize,
}

impl Box {
    #[inline]
    #[must_use]
    fn new(values: Vec<usize>) -> Self {
        Box {
            l: values[0],
            w: values[1],
            h: values[2],
        }
    }

    fn surface_area(&self) -> usize {
        // 2*l*w + 2*w*h + 2*h*l.
        (2 * self.l * self.w) + (2 * self.w * self.h) + (2 * self.h * self.l)
    }

    fn smallest_face_area(&self) -> usize {
        (self.w * self.h).min(self.w * self.l).min(self.h * self.l)
    }

    fn smallest_perimeter(&self) -> usize {
        (2 * self.w + 2 * self.h)
            .min(2 * self.w + 2 * self.l)
            .min(2 * self.h + 2 * self.l)
    }

    fn volume(&self) -> usize {
        self.w * self.h * self.l
    }
}

// Can add more shared vars here
pub struct IWasToldThereWouldBeNoMath {
    data: Vec<Box>,
}

// Can be used to implement fancier task-specific parsing
impl Fro for IWasToldThereWouldBeNoMath {
    fn fro(input: &str) -> Self {
        Self {
            data: input
                .split('\n')
                .map(|line| {
                    Box::new(
                        line.split('x')
                            .map(|v| v.parse::<usize>().unwrap())
                            .collect::<Vec<usize>>(),
                    )
                })
                .collect(),
        }
    }
}

// Main solvers
impl Solution for IWasToldThereWouldBeNoMath {
    fn silver(&self) -> TaskResult {
        self.data
            .iter()
            .map(|boxx| boxx.surface_area() + boxx.smallest_face_area())
            .sum::<usize>()
            .into()
    }

    /*
    The ribbon required to wrap a present is the shortest distance around its sides, or the smallest perimeter of any one face. Each present also requires a bow made out of ribbon as well; the feet of ribbon required for the perfect bow is equal to the cubic feet of volume of the present. Don't ask how they tie the bow, though; they'll never tell.
     */
    fn gold(&self) -> TaskResult {
        self.data
            .iter()
            .map(|boxx| boxx.smallest_perimeter() + boxx.volume())
            .sum::<usize>()
            .into()
    }
}

// For assisting functions
impl IWasToldThereWouldBeNoMath {}

// cargo test --lib day_XX
#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::utils::read_data_from_file;

    #[test]
    fn test() {
        let test_data = read_data_from_file("input/2015/test/02.txt");
        let queue = IWasToldThereWouldBeNoMath::fro(&test_data);

        assert_eq!(queue.silver(), TaskResult::Usize(101));
        assert_eq!(queue.gold(), TaskResult::Usize(48));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/2015/real/02.txt");
        let queue = IWasToldThereWouldBeNoMath::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(1606483));
        assert_eq!(queue.gold(), TaskResult::Usize(0));
    }
}
