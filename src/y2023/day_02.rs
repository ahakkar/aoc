/*
 * 2023 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REGEX_GAME_ID: regex::Regex = Regex::new(r"\d+").unwrap();
}

use crate::{Fro, Solution, TaskResult};

// Can add more shared vars here
pub struct CubeConundrum {
    data: Vec<String>,
}

// Can be used to implement fancier task-specific parsing
impl Fro for CubeConundrum {
    fn fro(input: &str) -> Self {
        Self {
            data: input.split('\n').map(|line| line.to_string()).collect(),
        }
    }
}

// Main solvers
impl Solution for CubeConundrum {
    fn silver(&self) -> TaskResult {
        let mut sum: usize = 0;

        for row in &self.data {
            let mut parts_iter = row.split(':');

            let game_id = REGEX_GAME_ID
                .find(parts_iter.next().unwrap())
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();

            if self.check_silver_game(parts_iter.next().unwrap()) {
                sum += game_id;
            }
        }
        sum.into()
    }

    fn gold(&self) -> TaskResult {
        let mut sum: usize = 0;

        for row in &self.data {
            let mut parts_iter = row.split(':');
            sum += self.check_gold_game(parts_iter.next_back().unwrap());
        }
        sum.into()
    }
}

// For assisting functions
impl CubeConundrum {
    // only 12 red cubes, 13 green cubes, and 14 blue cubes?
    fn check_silver_game(&self, games: &str) -> bool {
        let games_iter = games.split(';');
        for game in games_iter {
            let colors_iter = game.split(',');
            for color in colors_iter {
                let mut parts_iter = color.trim().split(' ');
                let num = parts_iter.next().unwrap().parse::<usize>().unwrap();
                let rgb = parts_iter.next().unwrap().chars().next().unwrap();

                if (rgb == 'r' && num > 12)
                    || (rgb == 'g' && num > 13)
                    || (rgb == 'b' && num > 14)
                {
                    return false;
                }
            }
        }
        true
    }

    // calculate power of max r*g*b
    fn check_gold_game(&self, games: &str) -> usize {
        let games_iter = games.split(';');
        let mut max_r: usize = 0;
        let mut max_g: usize = 0;
        let mut max_b: usize = 0;

        for game in games_iter {
            let colors_iter = game.split(',');

            for color in colors_iter {
                let mut parts_iter = color.trim().split(' ');
                let num = parts_iter.next().unwrap().parse::<usize>().unwrap();
                let rgb = parts_iter.next().unwrap().chars().next().unwrap();

                if rgb == 'r' && num > max_r {
                    max_r = num;
                } else if rgb == 'g' && num > max_g {
                    max_g = num;
                }
                if rgb == 'b' && num > max_b {
                    max_b = num;
                }
            }
        }
        max_r * max_g * max_b
    }
}

// cargo test --lib day_XX
#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::utils::read_data_from_file;

    #[test]
    fn test() {
        let test_data = read_data_from_file("input/2023/test/02.txt");
        let queue = CubeConundrum::fro(&test_data);

        assert_eq!(queue.silver(), TaskResult::Usize(0));
        assert_eq!(queue.gold(), TaskResult::Usize(0));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/2023/real/02.txt");
        let queue = CubeConundrum::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(2683));
        assert_eq!(queue.gold(), TaskResult::Usize(49710));
    }
}
