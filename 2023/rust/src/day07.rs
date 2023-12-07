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
use std::collections::HashMap;
use std::fs;
use std::fmt;
use std::time::Instant;

#[derive(Eq, PartialEq)]
struct Hand {
    cards: Vec<i8>,    
    rank: i8,
    str: String,
    bid: i16
}

// Mandatory, Calls ord below
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Mandatory, comparator for Hand
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if cmp_hands(self, other) {
            Ordering::Greater
        } else if self.str.eq(&other.str) {
            Ordering::Equal
        } else {
            Ordering::Less
        }    
    }
}

fn strtoivec(string: &str) -> Vec<i8> {
    let mut hand : Vec<i8> = vec![];
    for card in string.chars() {  
        hand.push(match card {
            '2'..='9' => card as i8 - '0' as i8,
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            _   => 0,
        });        
    }
    hand
}

// Actual comparison logic: a > b ? true : false
fn cmp_hands(a:&Hand, b:&Hand) -> bool {
    if a.rank == b.rank {
        // compare cards from left to right if rank matches
        for i in 0..5 {
            if a.cards[i] > b.cards[i] { return true }
        }
    } else if (a.rank > b.rank) {
        return true;
    }
    false
}

// count occurencies of each card and match their frequency
fn calc_rank(cards: &[i8]) -> i8 {
    let mut counts = HashMap::new();
    for &card in cards {
        *counts.entry(card).or_insert(0) += 1;
    }

    let mut freqs: Vec<_> = counts.values().collect();
    freqs.sort_unstable_by(|a, b| b.cmp(a));

    match freqs.as_slice() {
        [5] => 1,                   // Five of a Kind
        [4, _] => 2,                // Four of a Kind
        [3, 2] => 3,                // Full House
        [3, 1, 1] => 4,             // Three of a Kind
        [2, 2, 1] => 5,             // Two Pair
        [2, 1, 1, 1] => 6,          // One Pair
        [1, 1, 1, 1, 1] => 7,       // High Card
        _ => i8::MAX,               // No hand (basically error case)
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
        let hand_str = a.parse::<String>().unwrap();
        let bid = b.parse::<i16>().unwrap();

        score.insert(
            Hand {     
                cards: strtoivec(&hand_str),                
                rank: calc_rank(&strtoivec(&hand_str)),
                str: hand_str,
                bid
            });
    }

    for hand in score {
        println!("{}, {} {}", hand.rank, hand.str, hand.bid);
    }
}