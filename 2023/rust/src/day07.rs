#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(clippy::needless_return)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]

use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::fs;
use std::fmt;
use std::time::Instant;

#[derive(Eq, PartialEq)]
struct Hand {
    hand: String,
    mult: i16
}

// Calls ord below
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Comparator for Hand, actual comparison logic
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand < other.hand {
            Ordering::Less
        } else if self.hand > other.hand {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

fn main() {
    let start = Instant::now();
    let input = fs::read_to_string("input/07simple.txt").unwrap();
    let data: Vec<&str> = input.lines().collect();

    process(&data);

    let duration = start.elapsed();
    println!("Time elapsed in main() is: {:?}", duration);
}

fn process(data: &[&str]) {  
    let mut score: BTreeSet<Hand> = BTreeSet::new();
    for row in data {
        let (a, b) = row.split_once(' ').unwrap();
        let hand = a.parse::<String>().unwrap();
        let mult = b.parse::<i16>().unwrap();

        score.insert(Hand {hand, mult});
    }

    for game in score {
        println!("{0}, {1}", game.hand, game.mult);
    }
}