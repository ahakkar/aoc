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

use good_lp::{
    Expression, ResolutionError, Solution, SolverModel, default_solver, variable,
    variables,
};

use crate::{Fro, Solution as AoCSolution, TaskResult};

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
impl AoCSolution for Factory {
    fn silver(&self) -> TaskResult {
        let data: Vec<Machine> = self.parse_lines();
        //data.iter().for_each(|m| println!("{}", m));
        let mut iterations = 0;

        let result = data
            .iter()
            .map(|machine| {
                // Count, mask
                let mut best: (usize, u16) = (usize::MAX, 0);
                for mask in 0..(1 << machine.commands.len()) {
                    let mut xor_sum = 0;
                    let mut count = 0;
                    for i in 0..machine.commands.len() {
                        iterations += 1;
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
            .sum::<usize>();

        println!("iterations: {}", iterations);
        TaskResult::Usize(result)
    }

    fn gold(&self) -> TaskResult {
        // Silver is completely useless for gold so
        // we implement a new solution from scratch
        let mut sum = 0;

        for line in &self.lines {
            let fields = self.get_fields(line);
            let target_width = fields.last().unwrap().len();
            let target = &fields.last().unwrap()[1..target_width - 1]
                .split(',')
                .map(|c| c.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            let buttons = &fields[1..fields.len() - 1]
                .iter()
                .map(|b| {
                    b[1..b.len() - 1]
                        .split(',')
                        .map(|c| c.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>();

            println!("target: {:?}", target);
            println!("buttons: {:?}", buttons);
            sum += self.solve_machine(target, buttons) as usize;
        }

        TaskResult::Usize(sum)
    }
}

// For assisting functions
impl Factory {
    fn solve_machine(&self, target: &[usize], buttons: &[Vec<usize>]) -> f64 {
        let dim = target.len();
        let n = buttons.len();

        // Convert buttons into coefficient vectors:
        // button_vecs[i][d] = 1 if button i affects dimension d, else 0.
        let mut button_vecs = vec![vec![0_i32; dim]; n];
        for (i, btn) in buttons.iter().enumerate() {
            for &d in btn {
                button_vecs[i][d] = 1;
            }
        }

        // x[i] = number of times we press button i (integer, >= 0)
        let mut vars = variables!();
        let x = vars.add_vector(variable().min(0).integer(), n);

        // Objective: minimize total number of button presses: sum_i x[i]
        let mut objective: Expression = 0.into();
        for i in 0..n {
            objective += x[i];
        }

        // Create model with the objective
        let mut model = vars.minimise(&objective).using(default_solver);

        // Constraints: for each dimension d,
        //   sum_i x[i] * button_vecs[i][d] == target[d]
        for d in 0..dim {
            let mut expr: Expression = 0.into();
            for i in 0..n {
                let coeff = button_vecs[i][d]; // i32
                if coeff != 0 {
                    expr += x[i] * coeff;
                }
            }
            // Expression::eq returns a Constraint
            model = model.with(expr.eq(target[d] as i32));
        }

        let result = model.solve().unwrap();
        result.eval(objective)
    }

    fn get_fields(&self, line: &str) -> Vec<String> {
        line.split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect()
    }

    fn parse_lines(&self) -> Vec<Machine> {
        self.lines
            .iter()
            .map(|l| {
                let fields = self.get_fields(l);
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
