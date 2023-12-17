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

use std::collections::HashMap;
use petgraph::graph::{DiGraph, NodeIndex};
use super::utils::*;

pub fn solve(data: Vec<String>) {    
    let map:GridMap<u8> = GridMap::new(data_as_intmap(&data));
    let graph:DiGraph<Coord, usize> = build_graph(&map);
    println!("Silver: {}", silver(&data));
    //println!("Gold: {}", gold(&data));
}

// Build a graph where each node will have direct edges to other
// nodes up to 3 tiles away in each direction
fn build_graph(map: &GridMap<u8>) -> DiGraph<Coord, usize> {
    let mut graph:DiGraph<Coord, usize> = DiGraph::new();
    let mut node_indices:NodeMap = HashMap::new();

    for (y, row) in map.get_data().iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            let node_index = graph.add_node(Coord::new(x as isize, y as isize));
            node_indices.insert(Coord::new(x as isize, y as isize), node_index);
        }
    }

    for (a, &node_index) in node_indices.iter() {
        for dx in 0..=2 {
            for dy in 0..=2 {
                if dx + dy > 0 && dx + dy <= 3 {
                    // Add edges in each direction if within grid bounds
                    let b:Coord = Coord::new(dx as isize, dy as isize);
                    if let Some(cost) = calculate_edge_cost(map, a, &b) {
                        if let Some(target_index) = 
                            node_indices.get(&Coord::new(a.x + b.x, a.y + b.y)) {
                            graph.add_edge(node_index, *target_index, cost);
                        }
                    }
                }
            }
        }
    }

    graph
}

// Get an appropriate cost for a edge, the sum of all nodes' weights between a-b
fn calculate_edge_cost(grid: &GridMap<u8>, a: &Coord, b: &Coord) -> Option<usize> {
    // Calculate the cost of moving from (x, y) to (x + dx, y + dy)
    // ...
    None
}

fn silver(data: &[String]) -> usize {
    let mut sum: usize = 0;    

    for row in data {

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
        let test_data:Vec<String> = read_data_from_file("input/test/17.txt");
        assert_eq!(silver(&test_data), 1320);
        //assert_eq!(gold(&test_data), 145);
    }

    #[test]
    fn test_silver() {
        let test_data:Vec<String> = read_data_from_file("input/real/17.txt");
        assert_eq!(silver(&test_data), 510801);
    }

    #[test]
    fn test_gold() {
        let test_data:Vec<String> = read_data_from_file("input/real/17.txt");
        //assert_eq!(gold(&test_data), 212763);
    }
}