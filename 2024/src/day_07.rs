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

use itertools::Itertools;

use crate::{Fro, Solution};
use super::utils::*;
use rayon::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Operation {
    Add,
    Multiply,    
    Conc,
}

// Can add more shared vars here
pub struct BridgeRepair {    
    data: Vec<(usize, Vec<usize>)>,
}

// Can be used to implement fancier task-specific parsing
impl Fro for BridgeRepair {
    fn fro(data: &str) -> Self{
        Self {
        data: data
            .split('\n')
            .map(|line| {
                if let Some((a, b)) = line.split_once(':') {
                    let a = a.parse::<usize>().unwrap();
                    let b = b.split_ascii_whitespace()
                        .map(|n| n.trim().parse::<usize>().unwrap())
                        .collect();
                    (a, b)
                }
                else {
                    panic!("invalid input data");
                }
            })
            .collect()
        }
    }
}

// Main solvers
impl Solution for BridgeRepair {
    fn silver(&self) -> usize {
        let op = Vec::from([Operation::Add, Operation::Multiply]);
        self.data.iter()
            .map(|row| if Self::solve(&row.0, &row.1, &op) { row.0 } else { 0 })
            .sum()
    }
    

    fn gold(&self) -> usize {    
        let op = Vec::from([Operation::Add, Operation::Multiply, Operation::Conc]);
        self.data.par_iter()            
            .map(|row| if Self::solve(&row.0, &row.1, &op) { row.0 } else { 0 })
            .sum()
    }

}

// For assisting functions
impl BridgeRepair {
    fn solve(res: &usize, nums: &[usize], op: &[Operation]) -> bool {  
        // Use itertools to calculate all permutations of operators      
        for combination in (0..nums.len()-1).map(|_| op.iter().cloned())
            .multi_cartesian_product()
        {                
            let mut sum = nums[0];
            let mut i = 1;

            for c in &combination {
                match c {
                    Operation::Add => sum += nums[i],
                    Operation::Multiply => sum *= nums[i],
                    Operation::Conc => sum = format!("{}{}", sum, nums[i]).parse::<usize>().unwrap(),               
                }
                i += 1;
            }
        
            if sum == *res { return true }               
        }
        false
    }    
}


// cargo test --lib day_XX
#[cfg(test)]
mod tests {
    use crate::utils::read_data_from_file;   
    use super::*;   

    #[test]
    fn test() {  
        let test_data = read_data_from_file("input/test/07.txt"); 
        let queue = BridgeRepair::fro(&test_data);        
  
        assert_eq!(queue.silver(), 3749);
        assert_eq!(queue.gold(), 11387);
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/real/07.txt");
        let queue = BridgeRepair::fro(&real_data);        
  
        assert_eq!(queue.silver(), 663613490587);
        assert_eq!(queue.gold(), 110365987435001);
    }
}
