/*
 * 2024 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/
use crate::{Fro, Solution, TaskResult};

use std::collections::HashMap;

pub struct HistorianHysteria {
    data: Vec<String>
}

impl Fro for HistorianHysteria {
    fn fro(data: &str) -> Self{
        Self { data: data.split('\n').map(|line| line.to_string()).collect() }
    }
}

impl Solution for HistorianHysteria {
    fn silver(&self) -> TaskResult {
        let mut sum: usize = 0;  
        let mut a: Vec<usize> = vec![];
        let mut b: Vec<usize> = vec![];
    
        for row in &self.data {
            let mut iter = row.split_ascii_whitespace();  
            a.push(
                iter.next().unwrap().parse::<usize>().unwrap()
            );
            b.push(
                iter.next().unwrap().parse::<usize>().unwrap()
            );
        }
    
        a.sort();
        b.sort();
    
        for i in 0..a.len() {
            if a[i] > b[i] {
                sum += a[i] - b[i];
            } else {
                sum += b[i] - a[i];
            }        
        }
    
        TaskResult::Usize(sum)
    }
    
    fn gold(&self) -> TaskResult {
        let mut sum: usize = 0;    
    
        let mut a: Vec<usize> = vec![];
        let mut b: HashMap<usize, usize> = HashMap::new();
    
        for row in &self.data {
            let mut iter = row.split_ascii_whitespace();  
            a.push(
                iter.next().unwrap().parse::<usize>().unwrap()
            );
    
            let num = iter.next().unwrap().parse::<usize>().unwrap();
            if let std::collections::hash_map::Entry::Vacant(e) = b.entry(num) {
                e.insert(1);
            } else  if let Some(x) = b.get_mut(&num) {
                *x += 1;           
            }        
        } 
    
        for num in a {
            if let Some(val) = b.get(&num) {
                sum += num * val;
            }
        }
    
        TaskResult::Usize(sum)
    }
}

impl HistorianHysteria {
    
}


// run these with cargo test --bin main -- day_XX::tests
#[cfg(test)]
mod tests {
    use crate::utils::read_data_from_file;   
    use super::*;   

    #[test]
    fn test_test() {  
        let test_data = read_data_from_file("input/test/01.txt"); 
        let queue = HistorianHysteria::fro(&test_data);        
  
        assert_eq!(queue.silver(), TaskResult::Usize(11));
        assert_eq!(queue.gold(), TaskResult::Usize(31));
    }

    #[test]
    fn test_real() {
        let real_data = read_data_from_file("input/real/01.txt");
        let queue = HistorianHysteria::fro(&real_data);        
  
        assert_eq!(queue.silver(), TaskResult::Usize(1879048));
        assert_eq!(queue.gold(), TaskResult::Usize(21024792));

    }
}