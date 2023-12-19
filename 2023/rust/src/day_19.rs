/*
 * 2023 Advent of Code with Rust
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
#![allow(clippy::while_let_on_iterator)]

use std::collections::{HashMap, VecDeque};

use super::utils::*;

const LESSER:u8 = 0;
const GREATER:u8 = 1;
type Name = String;

#[derive(Debug)]
struct Rule {
    condition: char,
    operator: u8, // 0: <, 1: >
    limit: u16,
    next: String
}

#[derive(Debug)]
struct Process {
    name: String,
    rules: Vec<Rule>,
    finally: String,
}

struct Part {
    x: u16,
    m: u16,
    a: u16,
    s: u16,
}


pub fn solve(data: Vec<String>) {    
    println!("Silver: {}", silver(&data));
    //println!("Gold: {}", gold(&data));
}

fn silver(data: &[String]) -> usize {
    let mut procs: HashMap<Name, Process> = HashMap::new();
    let mut partq: VecDeque<Part> = VecDeque::new();
    let mut sum: usize = 0; 
    let mut it = data.iter();

    while let Some(row) = it.next() {
        if let Some(idx) = row.find('{') {
            let name = &row[0..idx];
            let rest = &row[idx+1..].strip_suffix('}').unwrap();
            let mut rules: Vec<&str> = rest.split(',').collect();            
            let finally = rules.pop().unwrap().to_string();
            let mut proc = 
                Process{
                    name: name.to_string(),
                    rules: vec![],
                    finally
                };

            // for part in parts, add a rule to process
            for rule in rules {
                let mut cit = rule.chars();
                let condition:char = cit.next().unwrap();
                let operator:u8 = match cit.next().unwrap() {
                    '<' => LESSER,
                    '>' => GREATER,
                     _  => panic!("invalid operator for rule, < or > expected"),
                };
                if let Some(splitter) = rule.find(':') {
                    let (left_side, right_side) = rule.split_at(splitter);
                    let limit:u16 = left_side[2..].parse::<u16>().unwrap();
                    let next:String = right_side[1..].to_string(); 

                    proc.rules.push(Rule{condition, operator, limit, next});
                }                
            }
            procs.insert(name.to_string(), proc);
            //println!("{} rules {:?} next: {}", name, rules, next);
        }
        if row.is_empty() {            
            break;
        }
    }
    for proc in procs {
        println!("{:?}", proc);
    }
    sum 
}

/* fn gold(data: &Vec<String>) -> usize {
    let mut sum: usize = 0;    

    for row in data {
           } 
    sum 
} */

// run these with cargo test --bin main -- day_XX::tests
#[cfg(test)]
mod tests {
    use crate::utils::read_data_from_file;
    use super::*;   

    #[test]
    fn test_test() {
        let test_data:Vec<String> = read_data_from_file("input/test/19.txt");
        assert_eq!(silver(&test_data), 19114);
        //assert_eq!(gold(&test_data), 145);
    }

    #[test]
    fn test_silver() {
        let test_data:Vec<String> = read_data_from_file("input/real/19.txt");
        assert_eq!(silver(&test_data), 510801);
    }

    #[test]
    fn test_gold() {
        let test_data:Vec<String> = read_data_from_file("input/real/15.txt");
        //assert_eq!(gold(&test_data), 212763);
    }
}