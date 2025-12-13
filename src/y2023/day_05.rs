/*
 * 2023 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

use crate::{Fro, Solution, TaskResult};

#[derive(Debug)]
struct Range {
    sour_start: i64,
    range: i64,
    offset: i64,
}

// Can add more shared vars here
pub struct IfYouGiveASeedAFertilizer {
    data: Vec<String>,
}

// Can be used to implement fancier task-specific parsing
impl Fro for IfYouGiveASeedAFertilizer {
    fn fro(input: &str) -> Self {
        Self {
            data: input.split('\n').map(|line| line.to_string()).collect(),
        }
    }
}

// Main solvers
impl Solution for IfYouGiveASeedAFertilizer {
    fn silver(&self) -> TaskResult {
        let mut i: usize = 3;
        let mut range_vec: Vec<Range> = vec![];

        // seeds from 1st row and range data from rest
        let mut seeds: Vec<i64> = data[0]
            .split_once(": ")
            .unwrap()
            .1
            .split(' ')
            .filter_map(|n| n.parse::<i64>().ok())
            .collect::<Vec<_>>();

        // collect ranges and transform seeds on empty row
        while i < data.len() {
            if data.get(i).unwrap() == "" {
                transform_seeds(&mut seeds, &range_vec);
                range_vec.clear();
                i += 2;
                continue;
            }

            let ti = data[i]
                .split(' ')
                .filter_map(|n| n.parse::<i64>().ok())
                .collect::<Vec<_>>();

            range_vec.push(Range {
                sour_start: ti[1],
                range: ti[2],
                offset: ti[0] - ti[1],
            });
            i += 1;
        }
        TaskResult::Usize(*seeds.iter().min().unwrap() as usize)
    }

    fn gold(&self) -> TaskResult {
        let mut i: usize = 3;
        let mut seeds: Vec<i64> = vec![];
        let mut range_vec: Vec<Range> = vec![];

        // seeds from 1st row and range data from rest
        let vec: Vec<i64> = data[0]
            .split_once(": ")
            .unwrap()
            .1
            .split(' ')
            .filter_map(|n| n.parse::<i64>().ok())
            .collect::<Vec<_>>();

        for i in (0..vec.len()).step_by(2) {
            // brute force, 120 seconds
            if i + 1 < vec.len() {
                for seed in vec[i]..=(vec[i] + vec[i + 1]) {
                    seeds.push(seed);
                }
            }
        }
        // collect ranges and transform seeds on empty row
        while i < data.len() {
            if data.get(i).unwrap() == "" {
                transform_seeds(&mut seeds, &range_vec);
                range_vec.clear();
                i += 2;
                //println!("---------------");
                continue;
            }

            let ti = data[i]
                .split(' ')
                .filter_map(|n| n.parse::<i64>().ok())
                .collect::<Vec<_>>();

            range_vec.push(Range {
                sour_start: ti[1],
                range: ti[2],
                offset: ti[0] - ti[1],
            });
            i += 1;
        }
        TaskResult::Usize(*seeds.iter().min().unwrap() as usize)
    }
}

// For assisting functions
impl IfYouGiveASeedAFertilizer {
    fn transform_seeds(seeds: &mut [i64], range_vec: &[Range]) {
        for seed in seeds.iter_mut() {
            // should probably fix the range comparisons instead of using break;
            for range in range_vec {
                if *seed >= range.sour_start && *seed <= (range.sour_start + range.range)
                {
                    *seed += range.offset;
                    break;
                }
            }
        }
    }
}

// cargo test --lib day_XX
#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::utils::read_data_from_file;

    #[test]
    fn test() {
        let test_data = read_data_from_file("input/2015/test/05.txt");
        let queue = IfYouGiveASeedAFertilizer::fro(&test_data);

        assert_eq!(queue.silver(), TaskResult::Usize(0));
        assert_eq!(queue.gold(), TaskResult::Usize(0));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/2023/real/05.txt");
        let queue = IfYouGiveASeedAFertilizer::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(278755257));
        assert_eq!(queue.gold(), TaskResult::Usize(26829166));
    }
}
