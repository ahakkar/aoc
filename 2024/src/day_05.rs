/*
 * 2024 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(clippy::needless_return)]
#![allow(clippy::needless_range_loop)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use std::{collections::{HashMap, HashSet}, hash::Hash};

use super::utils::*;

type Rules = HashMap<usize, HashSet<usize>>;
type Print = Vec<(bool, Vec<usize>)>;

pub fn solve(data: Vec<String>) {
    let (rules, mut print) = parse_data(&data);       

    println!("Silver: {}", silver(&rules, &print));
    println!("Gold: {}", gold(&rules, &print));
}

// For each page in pagelist, check the rules for page.
// If any of the pages in rules[page] are already in printed, 
// the order is invalid. Otherwise, add page to printed.
pub fn silver(rules: &Rules, mut print: &Print) -> usize {
    let mut sum: usize = 0;     

    print.iter().for_each(|(mut corr, pagelist)| {
        let mut printed: HashSet<usize> = HashSet::new();
        let mut valid = true;
        
        for page in pagelist {
            if let Some(rules) = rules.get(page) {
                if rules.iter().any(|r| printed.contains(r)) {
                    valid = false;
                    corr = false;
                    break;        
                }        
            } 
            printed.insert(*page);            
        }
        if valid { sum += pagelist.get(pagelist.len()/2).unwrap() }
    });
    sum 
}

pub fn gold(rules: &Rules, mut print: &Print) -> usize {
    let mut sum: usize = 0;   


    sum 
}

fn parse_data(data: &[String]) -> (Rules, Print) {
    let mut rules: Rules = HashMap::new();
    let mut print: Print= Vec::new();
    let mut toggle = false; 

    for row in data {
        if row.is_empty() { toggle = true; continue }
        match toggle {
            true => {
                print.push(
                    (true,
                    row.split(',')
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect()
                    )
                );
            }
            false => {     
                if let Some((a, b)) = row.split_once('|') {
                    let a = a.trim().parse::<usize>().unwrap();
                    let b = b.trim().parse::<usize>().unwrap();
                    rules.entry(a).or_default().insert(b);  
                } 
            },
        }
    }
    (rules, print)
}



// cargo test --bin main -- day_XX::tests
#[cfg(test)]
mod tests {
    use crate::utils::read_data_from_file;    
    use lazy_static::lazy_static;
    use super::*;   

    lazy_static! {
        static ref TEST_DATA: Vec<String> = read_data_from_file("input/test/05.txt");
        static ref REAL_DATA: Vec<String> = read_data_from_file("input/real/05.txt");
    } 

    #[test]
    fn test_test() {  
        let (rules, mut print) = parse_data(&TEST_DATA);       
        assert_eq!(silver(&rules, &print), 143);
        //assert_eq!(gold(&rules, &print), 123);
    }

    #[test]
    fn test_silver() {
        let (rules, mut print) = parse_data(&REAL_DATA);  
        assert_eq!(silver(&rules, &print), 5713);
    }

    #[test]
    fn test_gold() {
        let (rules, mut print) = parse_data(&REAL_DATA);  
        //assert_eq!(gold(&rules, &print), 212763);
    }
}