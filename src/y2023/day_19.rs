/*
 * 2023 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

#![allow(dead_code)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_assignments)]
#![allow(unused_must_use)]
#![allow(clippy::needless_return)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::only_used_in_recursion)]
#![allow(clippy::never_loop)]
#![allow(clippy::useless_vec)]
#![allow(clippy::collapsible_if)]

use crate::{Fro, Solution, TaskResult};

use std::collections::{HashMap, VecDeque};

const LESSER: u8 = 0;
const GREATER: u8 = 1;

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
    next: Name,
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

// Can add more shared vars here
pub struct Aplenty {
    data: Vec<String>,
}

// Can be used to implement fancier task-specific parsing
impl Fro for Aplenty {
    fn fro(input: &str) -> Self {
        Self {
            data: input.split('\n').map(|line| line.to_string()).collect(),
        }
    }
}

// Main solvers
impl Solution for Aplenty {
    fn silver(&self) -> TaskResult {
        let mut procs: Processes = HashMap::new();
        let mut partq: VecDeque<Part> = VecDeque::new();
        let mut sum: usize = 0;
        let mut process_input = true;

        // parse processes & rules from input
        for row in &self.data {
            if row.is_empty() {
                process_input = false;
                continue;
            }
            if process_input {
                let process = Aplenty::parse_process(row);
                procs.insert(process.name.clone(), process);
            } else {
                partq.push_back(Aplenty::parse_part(row));
            }
        }

        // process parts according to process rules
        while let Some(mut part) = partq.pop_front() {
            if let Some(process) = procs.get(&part.next) {
                let mut stop = false;
                let mut rule_matched = false;

                // Check for matching rules
                for rule in &process.rules {
                    let (matches, next) = Aplenty::rule_matches(&part, rule.clone());
                    if matches {
                        match next.as_deref() {
                            Some("A") => {
                                stop = true;
                                sum += part.get_sum();
                            }
                            Some("R") => stop = true,
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
                            }
                        }
                    } else {
                        partq.push_back(part);
                    }
                }
            }
        }
        sum.into()
    }

    fn gold(&self) -> TaskResult {
        let mut procs: Processes = HashMap::new();
        let mut _sum: usize = 0;

        // parse processes & rules from input
        for row in &self.data {
            if row.is_empty() {
                break;
            }
            let process = Aplenty::parse_process(row);
            procs.insert(process.name.clone(), process);
        }

        let mut _paths: Vec<Gold> = vec![];
        let mut bounds: Vec<(&str, Gold)> = vec![(
            "in",
            Gold {
                x: Bound::new(1, 4000),
                m: Bound::new(1, 4000),
                a: Bound::new(1, 4000),
                s: Bound::new(1, 4000),
            },
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

        _sum += 1;
        TaskResult::String("unsolved".to_string())
    }
}

// For assisting functions
impl Aplenty {
    fn parse_process(row: &str) -> Process {
        if let Some(idx) = row.find('{') {
            let name = &row[0..idx];
            let rest = &row[idx + 1..].strip_suffix('}').unwrap();
            let mut rules: Vec<&str> = rest.split(',').collect();
            let finally = rules.pop().unwrap().to_string();
            let mut proc = Process {
                name: name.to_string(),
                rules: vec![],
                finally,
            };

            for rule in rules {
                let mut cit = rule.chars();
                let condition: char = cit.next().unwrap();
                let operator: u8 = match cit.next().unwrap() {
                    '<' => LESSER,
                    '>' => GREATER,
                    _ => panic!("invalid operator for rule, < or > expected"),
                };
                if let Some(splitter) = rule.find(':') {
                    let (left_side, right_side) = rule.split_at(splitter);
                    let limit: usize = left_side[2..].parse::<usize>().unwrap();
                    let next: String = right_side[1..].to_string();

                    proc.rules.push(Rule {
                        condition,
                        operator,
                        limit,
                        next,
                    });
                }
            }
            return proc;
        }
        panic!("could not parse process");
    }

    fn parse_part(row: &str) -> Part {
        let str_parts = row
            .strip_prefix('{')
            .unwrap()
            .strip_suffix('}')
            .unwrap()
            .split(',');
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
                _ => panic!("invalid part value"),
            }
        }
        Part {
            x,
            m,
            a,
            s,
            next: "in".to_string(),
        }
    }

    fn rule_matches(part: &Part, rule: Rule) -> (bool, Option<String>) {
        let compare = |value: usize| {
            (rule.operator == GREATER && value > rule.limit)
                || (rule.operator == LESSER && value < rule.limit)
        };

        match rule.condition {
            'x' => {
                if compare(part.x) {
                    (true, Some(rule.next))
                } else {
                    (false, None)
                }
            }
            'm' => {
                if compare(part.m) {
                    (true, Some(rule.next))
                } else {
                    (false, None)
                }
            }
            'a' => {
                if compare(part.a) {
                    (true, Some(rule.next))
                } else {
                    (false, None)
                }
            }
            's' => {
                if compare(part.s) {
                    (true, Some(rule.next))
                } else {
                    (false, None)
                }
            }
            _ => panic!("bad condition"),
        }
    }
}

// cargo test --lib day_XX
#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::utils::read_data_from_file;

    #[test]
    fn test() {
        let test_data = read_data_from_file("input/2023/test/19.txt");
        let queue = Aplenty::fro(&test_data);

        assert_eq!(queue.silver(), TaskResult::Usize(19114));
        assert_eq!(queue.gold(), TaskResult::Usize(167409079868000));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/2023/real/19.txt");
        let queue = Aplenty::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(373302));
        assert_eq!(queue.gold(), TaskResult::Usize(0));
    }
}
