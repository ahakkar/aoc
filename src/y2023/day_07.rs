/*
 * 2023 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

use std::collections::HashMap;
use std::{cmp::Ordering, collections::BTreeSet};

use crate::{Fro, Solution, TaskResult};

// Can add more shared vars here
pub struct CamelCards {
    data: Vec<String>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Hand {
    cards: Vec<i8>,
    htype: i8,
    str: String,
    bid: usize,
}

impl std::fmt::Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.str, self.bid)
    }
}

// With equal cases compare indexes from beginning
impl Hand {
    fn compare_cards(&self, other: &Self) -> Ordering {
        for i in 0..5 {
            match self.cards[i].cmp(&other.cards[i]) {
                Ordering::Equal => continue,
                other_ordering => return other_ordering,
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Hand comaprator
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.htype.cmp(&other.htype) {
            Ordering::Equal => self.compare_cards(other),
            other_ordering => other_ordering,
        }
    }
}

// Can be used to implement fancier task-specific parsing
impl Fro for CamelCards {
    fn fro(input: &str) -> Self {
        Self {
            data: input.split('\n').map(|line| line.to_string()).collect(),
        }
    }
}

// Main solvers
impl Solution for CamelCards {
    fn silver(&self) -> TaskResult {
        let mut score: BTreeSet<Hand> = BTreeSet::new();
        for row in &self.data {
            let (a, b) = row.split_once(' ').unwrap();
            let hand_str = a.parse::<String>().unwrap();
            let bid = b.parse::<usize>().unwrap();

            score.insert(Hand {
                cards: CamelCards::map_card_to_int(&hand_str),
                htype: CamelCards::calc_rank(&CamelCards::map_card_to_int(&hand_str)),
                str: hand_str,
                bid,
            });
        }

        let mut sum: usize = 0;

        for (i, hand) in score.iter().enumerate() {
            println!("{} {} {} {}", i, hand.str, hand.htype, hand.bid);
            sum += hand.bid as usize * (i + 1);
        }

        sum.into()
    }

    fn gold(&self) -> TaskResult {
        let mut score: Vec<Hand> = Vec::with_capacity(1000);

        for row in &self.data {
            let (a, b) = row.split_once(' ').unwrap();
            let hand_str = a.parse::<String>().unwrap();
            let bid = b.parse::<usize>().unwrap();

            score.push(Hand {
                cards: CamelCards::map_card_to_int(&hand_str),
                htype: CamelCards::calc_htype(&CamelCards::map_card_to_int(&hand_str)),
                str: hand_str,
                bid,
            });
        }

        score.sort();

        let sum: usize = score
            .iter()
            .enumerate()
            .map(|(i, hand)| hand.bid * (i + 1))
            .sum();

        sum.into()
    }
}

// For assisting functions
impl CamelCards {
    // Converts card symbols to int values for easier comparison
    fn map_card_to_int(string: &str) -> Vec<i8> {
        string
            .chars()
            .map(|card| match card {
                '2'..='9' => card.to_digit(10).unwrap() as i8,
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'T' => 10,
                'J' => 1, // joker, not a jack
                _ => panic!(),
            })
            .collect()
    }

    // count occurencies of each card and match their frequency
    fn calc_htype(cards: &[i8]) -> i8 {
        let mut card_counts = HashMap::new();
        let mut joker_count = 0;

        for &card in cards {
            match card {
                1 => joker_count += 1,
                _ => *card_counts.entry(card).or_insert(0) += 1,
            }
        }

        // Special case with full Joker hand
        if joker_count == 5 {
            return 7;
        }

        // For each joker, increase the amount of largest non-joker at hand by 1
        let mut freqs: Vec<_> = card_counts.values().cloned().collect();

        freqs.sort_unstable_by(|a, b| b.cmp(a));

        if joker_count > 0 {
            if let Some(max) = freqs.first_mut() {
                *max += joker_count;
            }
        }

        match freqs.as_slice() {
            [5] => 7,        // Five of a Kind
            [4, _] => 6,     // Four of a Kind
            [3, 2] => 5,     // Full House
            [3, ..] => 4,    // Three of a Kind
            [2, 2, ..] => 3, // Two Pair
            [2, ..] => 2,    // One Pair
            _ => 1,          // High Card
        }
    }

    fn calc_rank(cards: &[i8]) -> i8 {
        let mut counts = HashMap::new();
        for &card in cards {
            *counts.entry(card).or_insert(0) += 1;
        }

        let mut freqs: Vec<_> = counts.values().collect();
        freqs.sort_unstable_by(|a, b| b.cmp(a));

        match freqs.as_slice() {
            [5] => 7,             // Five of a Kind
            [4, _] => 6,          // Four of a Kind
            [3, 2] => 5,          // Full House
            [3, 1, 1] => 4,       // Three of a Kind
            [2, 2, 1] => 3,       // Two Pair
            [2, 1, 1, 1] => 2,    // One Pair
            [1, 1, 1, 1, 1] => 1, // High Card
            _ => i8::MIN,         // No hand (basically error case)
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
        let test_data = read_data_from_file("input/2023/test/07.txt");
        let queue = CamelCards::fro(&test_data);

        assert_eq!(queue.silver(), TaskResult::Usize(6440));
        assert_eq!(queue.gold(), TaskResult::Usize(5905));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/2015/real/07.txt");
        let queue = CamelCards::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(253954294));
        assert_eq!(queue.gold(), TaskResult::Usize(254837398));
    }

    #[test]
    fn test_hand_types() {
        assert_eq!(CamelCards::calc_htype(&[1, 1, 1, 1, 1]), 7);
        assert_eq!(CamelCards::calc_htype(&[2, 2, 3, 4, 5]), 2);
        assert_eq!(CamelCards::calc_htype(&[2, 3, 4, 5, 6]), 1);
        assert_eq!(CamelCards::calc_htype(&[2, 2, 3, 3, 5]), 3);
        assert_eq!(CamelCards::calc_htype(&[3, 3, 3, 4, 5]), 4);
        assert_eq!(CamelCards::calc_htype(&[2, 2, 4, 4, 4]), 5);
        assert_eq!(CamelCards::calc_htype(&[12, 12, 12, 12, 9]), 6);
        assert_eq!(CamelCards::calc_htype(&[6, 6, 6, 6, 6]), 7);
        assert_eq!(CamelCards::calc_htype(&[1, 2, 3, 4, 5]), 2);
        assert_eq!(CamelCards::calc_htype(&[2, 2, 1, 4, 5]), 4);
        assert_eq!(CamelCards::calc_htype(&[1, 1, 1, 4, 5]), 6);
        assert_eq!(CamelCards::calc_htype(&[1, 2, 2, 4, 4]), 5);
        assert_eq!(CamelCards::calc_htype(&[5, 5, 5, 4, 1]), 6);
        assert_eq!(CamelCards::calc_htype(&[2, 2, 3, 3, 1]), 5);
    }

    #[test]
    fn test_map_card_to_int() {
        assert_eq!(CamelCards::map_card_to_int("JJJJJ"), [1, 1, 1, 1, 1]);
        assert_ne!(CamelCards::map_card_to_int("JJJJJ"), [1, 1, 1, 1, 2]);
        assert_eq!(CamelCards::map_card_to_int("2233J"), [2, 2, 3, 3, 1]);
        assert_eq!(CamelCards::map_card_to_int("2986K"), [2, 9, 8, 6, 13]);
        assert_eq!(CamelCards::map_card_to_int("2233J"), [2, 2, 3, 3, 1]);
        assert_eq!(CamelCards::map_card_to_int("AAK9T"), [14, 14, 13, 9, 10]);
        assert_eq!(CamelCards::map_card_to_int("AA5Q8"), [14, 14, 5, 12, 8]);
        assert_eq!(CamelCards::map_card_to_int("KTJJT"), [13, 10, 1, 1, 10]);
        assert_eq!(CamelCards::map_card_to_int("TTTJT"), [10, 10, 10, 1, 10]);
    }

    #[test]
    fn test_hand_ord_impl() {
        let a = Hand {
            cards: vec![2, 3, 4, 5, 6],
            htype: 1,
            str: String::from("23456"),
            bid: 806,
        };
        let b = Hand {
            cards: vec![7, 8, 9, 10, 12],
            htype: 1,
            str: String::from("789TQ"),
            bid: 123,
        };
        let c = Hand {
            cards: vec![2, 2, 1, 4, 5],
            htype: 4,
            str: String::from("22J45"),
            bid: 949,
        };
        let d = Hand {
            cards: vec![2, 2, 3, 3, 1],
            htype: 5,
            str: String::from("2233J"),
            bid: 47,
        };
        let d2 = Hand {
            cards: vec![2, 2, 3, 3, 1],
            htype: 5,
            str: String::from("2233J"),
            bid: 47,
        };
        let e = Hand {
            cards: vec![2, 2, 3, 3, 4],
            htype: 5,
            str: String::from("22334"),
            bid: 200,
        };

        assert!(a < b);
        assert!(c > b);
        assert!(d == d2);
        assert!(b != c);
        assert!(e > d);
    }

    #[test]
    fn test_hand_sort_impl() {
        let a = Hand {
            cards: vec![2, 3, 4, 5, 6],
            htype: 1,
            str: String::from("23456"),
            bid: 806,
        };
        let b = Hand {
            cards: vec![7, 8, 9, 10, 12],
            htype: 1,
            str: String::from("789TQ"),
            bid: 123,
        };
        let c = Hand {
            cards: vec![2, 2, 1, 4, 5],
            htype: 4,
            str: String::from("22J45"),
            bid: 949,
        };
        let d = Hand {
            cards: vec![2, 2, 3, 3, 1],
            htype: 5,
            str: String::from("2233J"),
            bid: 47,
        };
        let e = Hand {
            cards: vec![2, 2, 3, 3, 4],
            htype: 5,
            str: String::from("22334"),
            bid: 200,
        };

        let mut vec = vec![c.clone(), d.clone(), a.clone(), b.clone(), e.clone()];
        vec.sort();
        assert_eq!(vec, vec![a, b, c, d, e]);
    }
}
