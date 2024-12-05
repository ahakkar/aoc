/*
 * 2024 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

use std::{cmp::Ordering, collections::{HashMap, HashSet}};

type Rules = HashMap<usize, HashSet<usize>>;
type Print = Vec<(bool, Vec<usize>)>; // bool for is the list in correct order

pub fn solve(data: Vec<String>) {
    let (rules, mut print) = parse_data(&data);       

    println!("Silver: {}", silver(&rules, &mut print));
    println!("Gold: {}", gold(&rules, &mut print));
}

// For each page in pagelist, check the rules for page.
// If any of the pages in rules[page] are already in printed, 
// the order is invalid. Otherwise, add page to printed.
fn silver(rules: &Rules, print: &mut Print) -> usize {
    let mut sum: usize = 0;     

    for (corr, pagelist) in print {
        if pagelist.is_sorted_by(|a, b| custom_sort(a, b, rules) != Ordering::Greater) {
            sum += pagelist.get(pagelist.len()/2).unwrap();
        }        
        else { *corr = false; } // tag the pagelist as being in incorrect order
    }    
    sum 
}

fn gold(rules: &Rules, print: &mut Print) -> usize {
    let mut sum: usize = 0;   

    for (valid, pages) in print {    
        if *valid { continue; }   
        pages.sort_by(|a, b| custom_sort(a, b, rules));
        sum += pages.get(pages.len()/2).unwrap();        
    }
    sum 
}

fn custom_sort(a: &usize, b: &usize, rules: &Rules) -> Ordering {
    if a == b { return Ordering::Equal }
    if let Some(rules) = rules.get(a) {
        if rules.contains(b) { return Ordering::Less }
    }
    Ordering::Greater
}

fn parse_data(data: &[String]) -> (Rules, Print) {
    let mut rules: Rules = HashMap::new();
    let mut print: Print= Vec::new();
    let mut toggle = false; 

    for row in data {
        if row.is_empty() { toggle = true; continue }
        if toggle {
            print.push(
                (true,
                row.split(',')
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect()
                )
            );
        }
        else if let Some((a, b)) = row.split_once('|') {
            let a = a.trim().parse::<usize>().unwrap();
            let b = b.trim().parse::<usize>().unwrap();
            rules.entry(a).or_default().insert(b);  
        }        
    }
    (rules, print)
}


// cargo test --bin main -- day_XX::tests
#[cfg(test)]
mod tests {
    use crate::utils::read_data_from_file;   
    use super::*;   

    #[test]
    fn test_test() {  
        let test_data = read_data_from_file("input/test/05.txt");
        let (rules, mut print) = parse_data(&test_data);       
        assert_eq!(silver(&rules, &mut print), 143);
        assert_eq!(gold(&rules, &mut print), 123);
    }

    #[test]
    fn test_real() {
        let real_data = read_data_from_file("input/real/05.txt");
        let (rules, mut print) = parse_data(&real_data);  
        assert_eq!(silver(&rules, &mut print), 5713);
        assert_eq!(gold(&rules, &mut print), 5180);
    }
}