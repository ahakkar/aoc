use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::time::Instant;

#[derive(Eq, PartialEq)]
struct Hand {
    cards: Vec<i8>,    
    htype: i8,
    str: String,
    bid: usize
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

// Converts card symbols to int values for easier comparison
fn strtoivec(string: &str) -> Vec<i8> {
    string.chars().map(|card| match card {
        '2'..='9' => card.to_digit(10).unwrap() as i8,
        'A' => 14,
        'K' => 13,
        'Q' => 12,            
        'T' => 10,
        'J' => 1, // joker, not a jack 
         _  => panic!(),
    }).collect()    
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
    if joker_count == 5 { return 7 }

    // For each joker, increase the amount of largest non-joker at hand by 1
    let mut freqs: Vec<_> = card_counts
        .values()
        .cloned()
        .collect();

    freqs.sort_unstable_by(|a, b| b.cmp(a));
    
    if joker_count > 0 {
        if let Some(max) = freqs.first_mut() {
            *max += joker_count;
        }
    }

    match freqs.as_slice() {
        [5]         => 7, // Five of a Kind
        [4, _]      => 6, // Four of a Kind
        [3, 2]      => 5, // Full House
        [3, ..]     => 4, // Three of a Kind
        [2, 2, ..]  => 3, // Two Pair
        [2, ..]     => 2, // One Pair
        _           => 1, // High Card
    }
}

fn main() {
    let start = Instant::now();
    let input = fs::read_to_string("input/07puzzle.txt").unwrap();
    let data: Vec<&str> = input.lines().collect();

    process(&data);

    let duration = start.elapsed();
    println!("Time elapsed in main() is: {:?}", duration);
}

fn process(data: &[&str]) {  
    let mut score: Vec<Hand> = Vec::with_capacity(1000);

    for row in data {
        let (a, b) = row.split_once(' ').unwrap();
        let hand_str = a.parse::<String>().unwrap();
        let bid = b.parse::<usize>().unwrap();

        score.push(
            Hand {     
                cards: strtoivec(&hand_str),                
                htype: calc_htype(&strtoivec(&hand_str)),
                str: hand_str,
                bid
            });
    }

    score.sort();

    let sum: usize = score
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i + 1))
        .sum();

    println!("sum: {}", sum);
    // 254837398 CORRECT! (on 7th try after fixing hand cmp logic errors)
}