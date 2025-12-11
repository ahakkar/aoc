/*
 * 2025 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

use crate::{Fro, Solution, TaskResult};
use pathfinding::prelude::count_paths;
use std::collections::HashMap;

// Can add more shared vars here
pub struct Reactor {
    nodes: HashMap<String, Vec<String>>,
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Node {
    node: String,
    saw_dac: bool,
    saw_fft: bool,
}

// Can be used to implement fancier task-specific parsing
impl Fro for Reactor {
    fn fro(input: &str) -> Self {
        let mut nodes: HashMap<String, Vec<String>> = HashMap::new();

        input.split('\n').for_each(|line| {
            let (node, edges) = line.split_once(':').unwrap();
            nodes.insert(
                node.to_owned(),
                edges
                    .trim()
                    .split_ascii_whitespace()
                    .map(|s| s.to_string())
                    .collect(),
            );
        });

        Self { nodes }
    }
}

// Main solvers
impl Solution for Reactor {
    fn silver(&self) -> TaskResult {
        let start: String = "you".to_string();
        let success: String = "out".to_string();

        TaskResult::Usize(count_paths(
            start,
            |n| self.successors_silver(n),
            |n| n == &success,
        ))
    }

    fn gold(&self) -> TaskResult {
        let start: Node = Node {
            node: "svr".to_string(),
            saw_dac: false,
            saw_fft: false,
        };

        TaskResult::Usize(count_paths(
            start,
            |n| self.successors_gold(n),
            |n| n.node == "out" && n.saw_dac && n.saw_fft,
        ))
    }
}

// For assisting functions
impl Reactor {
    fn successors_silver(&self, node: &String) -> Vec<String> {
        self.nodes.get(node).cloned().unwrap_or_else(Vec::new)
    }

    fn successors_gold(&self, node: &Node) -> Vec<Node> {
        let mut result = Vec::new();

        if let Some(edges) = self.nodes.get(&node.node) {
            for e in edges {
                result.push(Node {
                    node: e.clone(),
                    saw_dac: node.saw_dac || (e == "dac"),
                    saw_fft: node.saw_fft || (e == "fft"),
                });
            }
        }
        result
    }
}

// cargo test --lib day_XX
#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::utils::read_data_from_file;

    #[test]
    fn test() {
        let test_data = read_data_from_file("input/2025/test/11.txt");
        let queue = Reactor::fro(&test_data);

        assert_eq!(queue.silver(), TaskResult::Usize(5));
        assert_eq!(queue.gold(), TaskResult::Usize(2));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/2025/real/11.txt");
        let queue = Reactor::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(0));
        assert_eq!(queue.gold(), TaskResult::Usize(0));
    }
}
