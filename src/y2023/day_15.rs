/*
 * 2023 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/
use std::collections::BTreeMap;

use crate::{Fro, Solution, TaskResult};

struct Box {
    lenses: Vec<Lens>,
}

struct Lens {
    fl: usize,
    label: String,
    deleted: bool,
}

// Can add more shared vars here
pub struct LensLibrary {
    data: Vec<String>,
}

// Can be used to implement fancier task-specific parsing
impl Fro for LensLibrary {
    fn fro(input: &str) -> Self {
        Self {
            data: input.split('\n').map(|line| line.to_string()).collect(),
        }
    }
}

// Main solvers
impl Solution for LensLibrary {
    fn silver(&self) -> TaskResult {
        self.data[0]
            .split(',')
            .map(LensLibrary::hash_str)
            .sum::<usize>()
            .into()
    }

    fn gold(&self) -> TaskResult {
        let mut shelf: BTreeMap<usize, Box> = BTreeMap::new();

        for idx in 0..=255 {
            shelf.insert(idx, Box { lenses: Vec::new() });
        }

        for hash in self.data[0].split(',') {
            let (label, op, fl) = LensLibrary::tokenize(hash);
            match op {
                '-' => LensLibrary::subtract(&mut shelf, &label),
                '=' => LensLibrary::add(&mut shelf, &label, &fl),
                _ => panic!("unsupported op"),
            }
        }
        LensLibrary::summarize(shelf).into()
    }
}

// For assisting functions
impl LensLibrary {
    fn hash_str(str: &str) -> usize {
        str.chars()
            .fold(0, |acc, c| (acc + (c as usize)) * 17 % 256)
    }

    fn subtract(shelf: &mut BTreeMap<usize, Box>, label: &str) {
        if let Some(boxx) = shelf.get_mut(&LensLibrary::hash_str(label)) {
            for lens in boxx.lenses.iter_mut() {
                if lens.label == label {
                    lens.deleted = true;
                }
            }
        }
    }

    fn add(shelf: &mut BTreeMap<usize, Box>, label: &str, fl: &usize) {
        if let Some(boxx) = shelf.get_mut(&LensLibrary::hash_str(label)) {
            let mut lens_found = false;
            // check for existing lens
            for lens in boxx.lenses.iter_mut() {
                if lens.label == label && !lens.deleted {
                    lens.fl = *fl;
                    lens_found = true;
                    break;
                }
            }
            // if not found, add a new one
            if !lens_found {
                boxx.lenses.push(Lens {
                    fl: *fl,
                    label: label.to_owned(),
                    deleted: false,
                });
            }
        }
    }

    fn tokenize(hash: &str) -> (String, char, usize) {
        let mut label: String = String::new();
        let mut op: char = 'x';
        let mut fl: usize = usize::MAX;

        for c in hash.chars() {
            if c != '=' && c != '-' {
                label.push(c);
            } else if c == '=' {
                op = c;
                fl = hash.chars().last().unwrap().to_digit(10).unwrap() as usize;
                break;
            } else {
                op = c;
            }
        }
        (label, op, fl)
    }

    fn summarize(shelf: BTreeMap<usize, Box>) -> usize {
        let mut sum: usize = 0;
        for boxx in shelf {
            let mut slot = 1;
            for lens in boxx.1.lenses {
                match lens.deleted {
                    true => slot -= 1,
                    false => sum += (boxx.0 + 1) * slot * lens.fl,
                }
                slot += 1;
            }
        }
        sum
    }
}

// cargo test --lib day_XX
#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::utils::read_data_from_file;

    #[test]
    fn test() {
        let test_data = read_data_from_file("input/2023/test/15.txt");
        let queue = LensLibrary::fro(&test_data);

        assert_eq!(queue.silver(), TaskResult::Usize(1320));
        assert_eq!(queue.gold(), TaskResult::Usize(145));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/2023/real/15.txt");
        let queue = LensLibrary::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(510801));
        assert_eq!(queue.gold(), TaskResult::Usize(212763));
    }
}
