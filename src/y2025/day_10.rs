/*
 * 2025 Advent of Code with Rust
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

use std::fmt;

use crate::{Fro, Solution, TaskResult};

// Can add more shared vars here
pub struct Factory {
    lines: Vec<String>,
}

#[derive(Debug)]
pub struct Machine {
    target: u16,
    commands: Vec<u16>,
    gold: Vec<u16>,
}

impl Machine {
    fn new(target: u16, commands: Vec<u16>, gold: Vec<u16>) -> Self {
        Machine {
            target,
            commands,
            gold,
        }
    }
}

impl fmt::Display for Machine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "--- Machine ---\n  target  = {}\n  commands= {:?}\n  gold    = {:?}\n",
            self.target, self.commands, self.gold
        )
    }
}

// Can be used to implement fancier task-specific parsing
impl Fro for Factory {
    fn fro(input: &str) -> Self {
        let lines: Vec<String> = input.split('\n').map(|line| line.to_string()).collect();
        Self { lines }
    }
}

// Main solvers
impl Solution for Factory {
    fn silver(&self) -> TaskResult {
        let data: Vec<Machine> = self.parse_lines();
        //data.iter().for_each(|m| println!("{}", m));

        data.iter()
            .map(|machine| {
                // Count, mask
                let mut best: (usize, u16) = (usize::MAX, 0);
                for mask in 0..(1 << machine.commands.len()) {
                    let mut xor_sum = 0;
                    let mut count = 0;
                    for i in 0..machine.commands.len() {
                        if mask & (1 << i) != 0 {
                            xor_sum ^= machine.commands[i];
                            count += 1;
                        }
                    }
                    if xor_sum == machine.target {
                        if count < best.0 {
                            best = (count, mask)
                        }
                    }
                }
                //println!("mask: {}, min presses: {}", best.1, best.0);
                best.0
            })
            .sum::<usize>()
            .into()
    }

    fn gold(&self) -> TaskResult {
        TaskResult::String("plaa".to_string())
    }
}

// For assisting functions
impl Factory {
    fn parse_lines(&self) -> Vec<Machine> {
        self.lines
            .iter()
            .map(|l| {
                let fields: Vec<String> =
                    l.split_ascii_whitespace().map(|s| s.to_string()).collect();
                let target = self.parse_target(fields.first().unwrap());
                let target_width: u16 = (fields.first().unwrap().len() - 2) as u16;
                let commands =
                    self.parse_commands(target_width, &fields[1..fields.len() - 1]);
                let gold = self.parse_gold(fields.last().unwrap());

                Machine::new(target, commands, gold)
            })
            .collect()
    }

    fn parse_target(&self, target: &str) -> u16 {
        let chars: Vec<char> = target[1..target.len() - 1].chars().collect();
        let mut mask = 0u16;

        for (i, c) in chars.iter().rev().enumerate() {
            if *c == '#' {
                mask |= 1 << i;
            }
        }
        mask
    }

    fn parse_commands(&self, target_width: u16, commands: &[String]) -> Vec<u16> {
        commands
            .iter()
            .map(|command| {
                let positions: Vec<u16> = command[1..command.len() - 1]
                    .split(',')
                    .map(|c| c.parse::<u16>().unwrap())
                    .collect();
                let mut mask: u16 = 0;

                for i in positions {
                    mask |= 1 << ((target_width - 1) - i);
                }
                mask
            })
            .collect()
    }

    fn parse_gold(&self, gold: &str) -> Vec<u16> {
        gold[1..gold.len() - 1]
            .split(',')
            .map(|n| n.parse::<u16>().unwrap())
            .collect()
    }
}

// cargo test --lib day_XX
#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::utils::read_data_from_file;

    #[test]
    fn test() {
        let test_data = read_data_from_file("input/2025/test/10.txt");
        let queue = Factory::fro(&test_data);

        assert_eq!(queue.silver(), TaskResult::Usize(7));
        assert_eq!(queue.gold(), TaskResult::Usize(33));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/2025/real/10.txt");
        let queue = Factory::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(502));
        assert_eq!(queue.gold(), TaskResult::Usize(0));
    }
}
