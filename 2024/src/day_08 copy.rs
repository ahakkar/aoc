/*
 * 2024 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use crate::{Fro, Solution, TaskResult};

// Can add more shared vars here
pub struct ResonantCollinearity  {
    points: HashMap<char, Vec<[isize; 2]>>,
    rows: isize,
    cols: isize,
}

// Can be used to implement fancier task-specific parsing
impl Fro for ResonantCollinearity {
    // Parse data to list of Points
    fn fro(data: &str) -> Self{   
        let mut points: HashMap<char, Vec<[isize; 2]>> = HashMap::new(); 
        let mut rows = 0;
        let mut cols = 0;            
        
        for (row_idx, row) in data.lines().enumerate() {
            rows = row_idx + 1;
            cols = cols.max(row.chars().count());
            for (col_idx, ch) in row.chars().enumerate() {
                if ch != '.' {
                    points.entry(ch)
                        .or_default()
                        .push([col_idx as isize, row_idx as isize]);              
                }    
            }
        }

        Self { points, rows: (rows as isize), cols: (cols as isize) }        
    }
}

// Main solvers
impl Solution for ResonantCollinearity {
    fn silver(&self) -> TaskResult {
        let mut u: HashSet<[isize; 2]> = HashSet::new();

        for point_vec in self.points.values() {
            point_vec.iter()
            .combinations(2)      
            .for_each(|pair| {    
                let p3 = Self::point_at_line(pair[0], pair[1], 1);
                if Self::fits_bounds(self, &p3) { u.insert(p3); }    
                let p4 = Self::point_at_line(pair[1], pair[0], 1);                            
                if Self::fits_bounds(self, &p4) { u.insert(p4); }      
            });
        } 
        TaskResult::Usize(u.len())       
    }    

    fn gold(&self) -> TaskResult {   
        let mut u: HashSet<[isize; 2]> = HashSet::new();

        for point_vec in self.points.values() {
            point_vec.iter()
            .combinations(2)      
            .for_each(|pair| {    
                Self::points_on_line(self, pair).iter()
                    .for_each(|point| { let _ = u.insert(*point); })         
            });
        } 
        TaskResult::Usize(u.len())
    }
}

// For assisting functions
impl ResonantCollinearity {
    fn fits_bounds(&self, p: &[isize; 2]) -> bool {
        p[0] >= 0 && p[1] >= 0 && p[0] < self.cols && p[1] < self.rows
    }

    fn point_at_line(p1: &[isize; 2], p2: &[isize; 2], m: isize) -> [isize; 2] {
        let v = [p2[0] - p1[0], p2[1] - p1[1]];        
        [ 
            p1[0] - m * v[0],
            p1[1] - m * v[1]
        ]   
    }

    fn points_on_line(&self, pair: Vec<&[isize; 2]>) -> Vec<[isize; 2]> {
        std::iter::once(*pair[0])
            .chain(std::iter::once(*pair[1]))
            .chain(self.extend_line(pair[0], pair[1]))
            .chain(self.extend_line(pair[1], pair[0]))
            .collect()
    }

    fn extend_line(&self, start: &[isize; 2], end: &[isize; 2]) -> Vec<[isize; 2]> {
        (1..)
            .map(|m| Self::point_at_line(start, end, m))
            .take_while(|np| Self::fits_bounds(self, np))
            .collect()
    }
}


// cargo test --lib day_XX
#[cfg(test)]
mod tests {
    use crate::utils::read_data_from_file;   
    use super::*;   

    #[test]
    fn test() {  
        let test_data = read_data_from_file("input/test/08.txt"); 
        let queue = ResonantCollinearity::fro(&test_data);        
  
        assert_eq!(queue.silver(), TaskResult::Usize(14));
        assert_eq!(queue.gold(), TaskResult::Usize(34));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/real/08.txt");
        let queue = ResonantCollinearity::fro(&real_data);        
  
        assert_eq!(queue.silver(), TaskResult::Usize(318));
        assert_eq!(queue.gold(), TaskResult::Usize(1126));
    }
}
