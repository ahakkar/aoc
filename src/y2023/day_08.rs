/*
 * 2023 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

use crate::{Fro, Solution, TaskResult};
use num_integer::lcm;
use std::collections::HashMap;

// Can add more shared vars here
pub struct HauntedWastelan {
    data: Vec<String>,
    nodes: HashMap<String, (String, String)>,
    start_nodes: Vec<String>,
}

// Can be used to implement fancier task-specific parsing
impl Fro for HauntedWastelan {
    fn fro(input: &str) -> Self {
        let data: Vec<String> = input.split('\n').map(|line| line.to_string()).collect();
        let mut start_nodes: Vec<String> = vec![];
        let nodes = build_tree(&data, &mut start_nodes);

        Self {
            data,
            nodes,
            start_nodes,
        }
    }
}

// Main solvers
impl Solution for HauntedWastelan {
    // traverse from start to end, counting iterations
    fn silver(&self) -> TaskResult {
        let mut endless_dir_iter = &self.data.first().unwrap().chars().cycle();
        let mut current_node: String = String::from("AAA");
        let mut dist: i64 = 0;

        while current_node != *"ZZZ" {
            let n = nodes.get(&current_node).unwrap();
            match endless_dir_iter.next().unwrap() {
                'L' => current_node = n.0.clone(),
                'R' => current_node = n.1.clone(),
                _ => panic!(),
            };
            dist += 1;
        }
        (dist as usize).into()
    }

    /**
     * The tree contains repeating paths with specific intervals. For each
     * tree traversal, find the repeating interval. Their least common multiple
     * tells the total distance.
     */
    fn gold(&self) -> TaskResult {
        let mut endless_dir_iter = &self.data.first().unwrap().chars().cycle();
        let mut current_nodes = self.start_nodes;
        let mut intervals = vec![0; current_nodes.len()];
        let mut steps: i64 = 0;

        // Find a repeating interval for every start node.
        while intervals.iter().any(|&i| i == 0) {
            // Determine the direction based on the current iteration
            let dir = endless_dir_iter.next().unwrap();

            for (i, node) in current_nodes.iter_mut().enumerate() {
                let n = nodes.get(node).unwrap();
                match dir {
                    'L' => *node = n.0.clone(),
                    'R' => *node = n.1.clone(),
                    _ => panic!(),
                }

                if check_node(node.as_str(), &'Z') && intervals[i] == 0 {
                    intervals[i] = steps + 1;
                }
            }

            steps += 1;
        }
        // [20093, 12169, 13301, 20659, 16697, 17263]
        // println!("{:?}", intervals);

        // Compute the LCM of the intervals
        TaskResult::Usize(intervals.into_iter().fold(1, lcm) as usize)
    }
}

// For assisting functions
impl HauntedWastelan {
    fn build_tree(
        data: &[String],
        start_nodes: &mut Vec<String>,
    ) -> HashMap<String, (String, String)> {
        let mut nodes: HashMap<String, (String, String)> = HashMap::new();

        for row in 2..data.len() {
            let (parent, children) = data.get(row).unwrap().split_once(" = ").unwrap();
            let (l, r) = children.split_once(", ").unwrap();

            // extract start nodes for gold, saves cpu cycles
            if check_node(parent, &'A') {
                start_nodes.push(String::from(parent));
            }

            nodes.insert(
                String::from(parent.trim()),
                (
                    String::from(l.trim_start_matches('(')),
                    String::from(r.trim_end_matches(')')),
                ),
            );
        }
        nodes
    }

    // Checks if node's 3rd char is 'char'
    fn check_node(node: &str, char: &char) -> bool {
        node.chars().nth(2) == Some(*char)
    }
}

// cargo test --lib day_XX
#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::utils::read_data_from_file;

    #[test]
    fn test() {
        let test_data = read_data_from_file("input/2023/test/08.txt");
        let queue = HauntedWastelan::fro(&test_data);

        assert_eq!(queue.silver(), TaskResult::Usize(0));
        assert_eq!(queue.gold(), TaskResult::Usize(0));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/2023/real/08.txt");
        let queue = HauntedWastelan::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(16697));
        assert_eq!(queue.gold(), TaskResult::Usize(10668805));
    }
}
