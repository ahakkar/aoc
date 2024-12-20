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


use std::collections::{HashMap, VecDeque};

const LESSER:u8 = 0;
const GREATER:u8 = 1;

type Name = String;
type Processes = HashMap<Name, Process>;

#[derive(Clone, Copy, Debug)]
struct Bound {
    min: usize,
    max: usize,
}

#[derive(Clone, Copy, Debug)]
struct Gold {
    x: Bound,
    m: Bound,
    a: Bound,
    s: Bound,
}

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
    next: Name,
}

#[derive(Debug)]
struct Process {
    name: Name,
    rules: Vec<Rule>,
    finally: String,
}

#[derive(Clone, Debug)]
struct Rule {
    condition: char,
    operator: u8, // 0: <, 1: >
    limit: usize,
    next: Name
}

impl Bound {
    const fn new(min: usize, max: usize) -> Bound {
        Bound { min, max }
    }
}

impl Part {
    fn get_sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}


pub fn solve(data: Vec<String>) {    
    println!("Silver: {}", silver(&data));
    println!("Gold: {}", gold(&data));
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
                let limit:usize = left_side[2..].parse::<usize>().unwrap();
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
    let mut x: usize = 0;
    let mut m: usize = 0;
    let mut a: usize = 0;
    let mut s: usize = 0;
    
    for str in str_parts {
        let (c, num) = str.split_once('=').unwrap();
        match c {
            "x" => x = num.parse::<usize>().unwrap(),
            "m" => m = num.parse::<usize>().unwrap(),
            "a" => a = num.parse::<usize>().unwrap(),
            "s" => s = num.parse::<usize>().unwrap(),
                _  => panic!("invalid part value"),
        }                
    }  
    Part{x,m,a,s, next: "in".to_string()}  
}

fn rule_matches(part: &Part, rule: Rule) -> (bool, Option<String>) {
    let compare = |value: usize| {
        (rule.operator == GREATER && value > rule.limit) || 
        (rule.operator == LESSER && value < rule.limit)
    };

    match rule.condition {
        'x' => if compare(part.x) { (true, Some(rule.next)) } else { (false, None) },
        'm' => if compare(part.m) { (true, Some(rule.next)) } else { (false, None) },
        'a' => if compare(part.a) { (true, Some(rule.next)) } else { (false, None) },
        's' => if compare(part.s) { (true, Some(rule.next)) } else { (false, None) },
        _   => panic!("bad condition"),
    }
}

fn silver(data: &[String]) -> usize {
    let mut procs: Processes = HashMap::new();
    let mut partq: VecDeque<Part> = VecDeque::new();
    let mut sum: usize = 0; 
    let mut process_input = true;

    // parse processes & rules from input
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

    // process parts according to process rules
    while let Some(mut part) = partq.pop_front() {
        if let Some(process) = procs.get(&part.next) {
            let mut stop = false;
            let mut rule_matched = false; 
    
            // Check for matching rules
            for rule in &process.rules {
                let (matches, next) = rule_matches(&part, rule.clone());
                if matches {
                    match next.as_deref() {
                        Some("A") => { stop = true;  sum += part.get_sum(); },
                        Some("R") =>   stop = true,
                        Some(next_process) => part.next = next_process.to_string(),
                        None => (),
                    };
                    rule_matched = true;
                    break;
                }
            }
    
            // Handle the "no rules matched" case
            if !stop {
                if !rule_matched {
                    match process.finally.as_str() {
                        "A" => sum += part.get_sum(),
                        "R" => (),
                        next_process => { 
                            part.next = next_process.to_string(); 
                            partq.push_back(part);
                        },
                    }
                } else {
                    partq.push_back(part);
                }
            }
        }
    }    
    sum 
}

fn gold(data: &Vec<String>) -> usize {
    let mut procs: Processes = HashMap::new();
    let mut sum: usize = 0; 

    // parse processes & rules from input
    for row in data {
        if row.is_empty() {       
            break;
        }
        let process = parse_process(row);
        procs.insert(process.name.clone(), process);
    }

    let mut paths:Vec<Gold> = vec![];
    let mut bounds:Vec<(&str, Gold)> = 
        vec![(
            "in",
             Gold { 
                x: Bound::new(1,4000),
                m: Bound::new(1,4000),
                a: Bound::new(1,4000),
                s: Bound::new(1,4000),
            }
        )];

    println!("{:?}", bounds);

/*     // calculate the min and max bounds for parts
    while let Some((loc, mut gold)) = bounds.pop() {
        // Bounds found for a path
        if loc == "A" {     
            paths.push(gold);
            continue;
        // Path leads to trash, ignore
        } else if loc == "R" {
            continue;
        }

        if let Some(process) = procs.get(loc) {
            for rule in &process.rules {
                match rule.operator {
                    LESSER => {
                        gold.x.min = 0; gold.x.max = 0;
                        gold.m.min = 0; gold.m.max = 0;
                        gold.a.min = 0; gold.a.max = 0;
                        gold.s.min = 0; gold.s.max = 0;
                    },
                    GREATER => {
                        gold.x.min = 0; gold.x.max = 0;
                        gold.m.min = 0; gold.m.max = 0;
                        gold.a.min = 0; gold.a.max = 0;
                        gold.s.min = 0; gold.s.max = 0;
                    },
                    _ => panic!("not valid"),
                }
            }
        }
    }
 */

    sum += 1;
    sum
}

// run these with cargo test --bin main -- day_XX::tests
#[cfg(test)]
mod tests {
    use crate::utils::read_data_from_file;
    use super::*;   

    #[test]
    fn test_test() {
        let test_data:Vec<String> = read_data_from_file("input/test/19.txt");
        assert_eq!(silver(&test_data), 19114);
        assert_eq!(gold(&test_data), 167409079868000);
    }

    #[test]
    fn test_silver() {
        let test_data:Vec<String> = read_data_from_file("input/real/19.txt");
        assert_eq!(silver(&test_data), 373302);
    }

/*     #[test]
    fn test_gold() {
        let test_data:Vec<String> = read_data_from_file("input/real/15.txt");
        //assert_eq!(gold(&test_data), 0);
    } */
}