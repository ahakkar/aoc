/*
 * 2024 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/
use crate::{Fro, Solution};
use regex::Regex;

pub struct MullItOver {
    data: Vec<String>
}

impl Fro for MullItOver {
    fn fro(data: &str) -> Self{
        Self { data: data.split('\n').map(|line| line.to_string()).collect() }
    }
}

impl Solution for MullItOver {
    fn silver(&self) -> usize {
        let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    
        self.data.iter()
            .flat_map(|row| re.find_iter(row))
            .map(|m| Self::parse_tuple(m.as_str()))
            .sum()
    }
    
    fn gold(&self) -> usize {
        let re = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don\'t\(\)").unwrap();
        let mut mode = true;
    
        self.data.iter()
            .flat_map(|row| re.find_iter(row))
            .filter_map(|s| {
                match s.as_str() {
                    "don't()" => { mode = false; None },
                    "do()"    => { mode = true; None },
                    _ => if mode { Some(Self::parse_tuple(s.into())) } else { None },
                }
            })
            .sum()
    }    
}

impl MullItOver {
    fn parse_tuple(s: &str) -> usize {
        s
            .get(4..s.len() - 1)
            .unwrap()
            .split_once(',')
            .map(|(a, b)| 
                a.trim().parse::<usize>().unwrap() * 
                b.trim().parse::<usize>().unwrap())  
            .unwrap()
    }
}

// cargo test --bin main -- day_XX::tests
#[cfg(test)]
mod tests {
    use crate::utils::read_data_from_file;   
    use super::*;   

    #[test]
    fn test_silver_test() {  
        let test_data = read_data_from_file("input/test/03s.txt"); 
        let queue = MullItOver::fro(&test_data);        
  
        assert_eq!(queue.silver(), 161);
    }

    #[test]
    fn test_gold_test() {  
        let test_data = read_data_from_file("input/test/03g.txt"); 
        let queue = MullItOver::fro(&test_data);        
  
        assert_eq!(queue.gold(), 48);
    }

    #[test]
    fn test_real() {
        let real_data = read_data_from_file("input/real/03.txt");
        let queue = MullItOver::fro(&real_data);        
  
        assert_eq!(queue.silver(), 175615763);
        assert_eq!(queue.gold(), 74361272);

    }
}