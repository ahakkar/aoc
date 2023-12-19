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
type Processes = HashMap<Name, Process>;

#[derive(Debug)]
struct Rule {
    condition: char,
    operator: u8, // 0: <, 1: >
    limit: u16,
    next: Name
}

#[derive(Debug)]
struct Process {
    name: Name,
    rules: Vec<Rule>,
    finally: String,
}

#[derive(Debug)]
struct Part {
    x: u16,
    m: u16,
    a: u16,
    s: u16,
    next: Name,
}


pub fn solve(data: Vec<String>) {    
    println!("Silver: {}", silver(&data));
    //println!("Gold: {}", gold(&data));
}

fn parse_process(row: &str) -> Process {
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
        return proc;
    }
    panic!("could not parse process");
}

fn parse_part(row: &str) -> Part {
    let str_parts = row.strip_prefix('{').unwrap().strip_suffix('}').unwrap().split(',');
    let mut x: u16 = 0;
    let mut m: u16 = 0;
    let mut a: u16 = 0;
    let mut s: u16 = 0;
    
    for str in str_parts {
        let (c, num) = str.split_once('=').unwrap();
        match c {
            "x" => x = num.parse::<u16>().unwrap(),
            "m" => m = num.parse::<u16>().unwrap(),
            "a" => a = num.parse::<u16>().unwrap(),
            "s" => s = num.parse::<u16>().unwrap(),
                _  => panic!("invalid part value"),
        }                
    }  
    Part{x,m,a,s, next: "in".to_string()}  
}

fn silver(data: &[String]) -> usize {
    let mut procs: HashMap<Name, Process> = HashMap::new();
    let mut partq: VecDeque<Part> = VecDeque::new();
    let mut sum: usize = 0; 
    let mut process_input = true;

    for row in data {
        if row.is_empty() {    
            process_input = false;        
            continue;
        }
        if process_input  { 
            let process = parse_process(row);
            procs.insert(process.name.clone(), process);
        } else {
            partq.push_back(parse_part(row)); 
        }
    }

    while let Some(mut part) = partq.pop_front() {
        if let Some(process) = procs.get(&part.next) {
            let mut processed = false; 
            // does part match any rules?
            for rule in &process.rules {
                match rule.condition {
                    'x' => {
                        if (rule.operator == LESSER && part.x < rule.limit) || 
                           (rule.operator == GREATER && part.x > rule.limit)
                        {       
                            part.next = rule.next.clone();
                            processed = true;                          
                            break;                             
                        }
                    },
                    'm' => {
                        if (rule.operator == LESSER && part.m < rule.limit) || 
                        (rule.operator == GREATER && part.m > rule.limit)
                        {       
                            part.next = rule.next.clone();
                            processed = true;                          
                            break;                             
                        }
                    },
                    'a' => {
                        if (rule.operator == LESSER && part.a < rule.limit) || 
                        (rule.operator == GREATER && part.a > rule.limit)
                        {       
                            part.next = rule.next.clone();
                            processed = true;                          
                            break;                             
                        }
                    },
                    's' => {
                        if (rule.operator == LESSER && part.s < rule.limit) || 
                        (rule.operator == GREATER && part.s > rule.limit)
                        {       
                            part.next = rule.next.clone();
                            processed = true;                          
                            break;                             
                        }
                    },
                     _  => panic!("invalid rule while processing"),
                }
            }
            // if no rules matched we get here
            if !processed {
                match process.finally.as_str() {
                    "A" => sum += part.x as usize + part.m as usize + part.a as usize + part.s as usize, 
                    "R" => (), // that'is for this part
                    _  => {
                        part.next = process.finally.clone();
                        partq.push_back(part); 
                    },
                }
            }
            else {
                partq.push_back(part);   
            }            
        }
    }
    sum 
}

/*
for proc in procs {
    println!("{:?}", proc);
}
for part in partq {
    println!("{:?}", part);
}
*/

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

/*     #[test]
    fn test_silver() {
        let test_data:Vec<String> = read_data_from_file("input/real/19.txt");
        assert_eq!(silver(&test_data), 510801);
    }

    #[test]
    fn test_gold() {
        let test_data:Vec<String> = read_data_from_file("input/real/15.txt");
        //assert_eq!(gold(&test_data), 212763);
    } */
}