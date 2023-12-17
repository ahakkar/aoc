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
use petgraph::{graph::{DiGraph, NodeIndex}, algo::dijkstra};
use super::utils::*;

const MAX_DIST: usize = 3;

pub fn solve(data: Vec<String>) {    
    let map:GridMap<u8> = GridMap::new(data_as_intmap(&data));
    let start: Coord = Coord::new(0,0);
    let end: Coord = Coord::new((map.get_height()-1) as isize, (map.get_width()-1) as isize);
    let mut node_indices:NodeMap = HashMap::new();
    let graph:DiGraph<Coord, usize> = build_graph(&map, &mut node_indices);

    println!("Silver: {}", silver(&start, &end, &graph, &node_indices));
    //println!("Gold: {}", gold(&data));
}

/*
    for (y, row) in grid.iter().enumerate() {
        for (x, &cost) in row.iter().enumerate() {
            // Create nodes for each direction and step count
            for &dir in &[NORTH, SOUTH, EAST, WEST] {
                for steps in 1..=3 {
                    let state = ((x, y), dir, steps);
                    let node_index = graph.add_node(state);
                    node_indices.insert(state, node_index);
                    // Add edges (omitted for brevity)
                }
            }
        }
    }

*/

// Build a graph where each node will have direct edges to other
// nodes up to 3 tiles away in each direction
fn build_graph(map: &GridMap<u8>, node_indices: &mut NodeMap) -> DiGraph<Coord, usize> {
    let mut graph:DiGraph<Coord, usize> = DiGraph::new();    
    let top_left: Coord = Coord::new(0,0);
    let bot_rght: Coord = Coord::new((map.get_height()-1) as isize, (map.get_width()-1) as isize);

    for (y, row) in map.get_data().iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            let node_index = graph.add_node(Coord::new(x as isize, y as isize));
            node_indices.insert(Coord::new(x as isize, y as isize), node_index);
        }
    }

    // create edges up to 4 tiles away. This adds up to 12 edges for each node
    for (cur_coord, &cur_index) in node_indices.iter() {
        // potential nodes in X in 1 to 3 steps away to either left or right
        for dx in -3..=3 {
            if dx == 0 { continue }
            let t_coord:Coord = Coord::new(cur_coord.x + dx as isize, cur_coord.y);
            if t_coord.fits_bounds(&top_left, &bot_rght) {
                let cost = calculate_edge_cost(map, cur_coord, &t_coord);
                if let Some(t_index) = node_indices.get(&t_coord) {
                    graph.add_edge(cur_index, *t_index, cost);
                } 
            }   
        }

        // potential nodes in Y in 1 to 3 steps away to either up or down
        for dy in -3..=3 {
            if dy == 0 { continue }
            let t_coord:Coord = Coord::new(cur_coord.x, cur_coord.y + dy as isize);
            if t_coord.fits_bounds(&top_left, &bot_rght) {
                let cost = calculate_edge_cost(map, cur_coord, &t_coord);
                if let Some(t_index) = node_indices.get(&t_coord) {
                    graph.add_edge(cur_index, *t_index, cost);
                }     
            }            
        }     
    }
    graph
}

// Get an appropriate cost for a edge, the sum of all nodes' weights between a-b
fn calculate_edge_cost(grid: &GridMap<u8>, a: &Coord, b: &Coord) -> usize {
    let mut sum: usize = 0;
    let x_dist = (b.x - a.x).abs();
    let y_dist = (b.y - a.y).abs();
    
    if (x_dist != 0 && y_dist != 0) {
        panic!("Should create only horizontal or vertical edges between nodes");
    }
    // println!("xd, {}, yd: {}, calc weight between a: {:?}, b: {:?}", x_dist, y_dist, a, b);
    if x_dist != 0 {
        // Horizontal line
        let step = if b.x > a.x { 1 } else { -1 };
        for i in 0..=x_dist {
            let coord = Coord::new(a.x + i * step, a.y);
            sum += grid.get_cell(&coord).unwrap_or(0) as usize;
        }
    } else {
        // Vertical line
        let step = if b.y > a.y { 1 } else { -1 };
        for i in 0..=y_dist {
            let coord = Coord::new(a.x, a.y + i * step);
            sum += grid.get_cell(&coord).unwrap_or(0) as usize;
        }
    }
  
    /*
    Hopefully sums like these are correct

    a: [3, 3], b: [0, 3], sum: 22
    a: [3, 3], b: [1, 3], sum: 18
    a: [3, 3], b: [2, 3], sum: 13
    a: [3, 3], b: [3, 0], sum: 22
    a: [3, 3], b: [3, 1], sum: 18
    a: [3, 3], b: [3, 2], sum: 10
    a: [3, 1], b: [0, 1], sum: 26
    a: [3, 1], b: [1, 1], sum: 21
    a: [3, 1], b: [2, 1], sum: 15
    a: [3, 1], b: [3, 0], sum: 12
    a: [3, 1], b: [3, 2], sum: 11
    a: [3, 1], b: [3, 3], sum: 18
    */
    //println!("a: {:?}, b: {:?}, sum: {}", a, b, sum);
    sum
}

fn silver(start: &Coord, end: &Coord, graph: &DiGraph<Coord, usize>, node_indices: &NodeMap) -> usize { 
    if let (Some(&start_index), Some(&end_index)) = (
        node_indices.get(start), 
        node_indices.get(end)
    ) {
        let node_costs = dijkstra(graph, start_index, Some(end_index), |e| *e.weight());

        // Retrieve the cost to the end node
        if let Some(&cost) = node_costs.get(&end_index) {
            return cost;
        } else {
            panic!("no path found between start and end when one should be found!");
        }
    } else {
        panic!("Start or end node not found in the graph");
    }
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
        let map:GridMap<u8> = GridMap::new(data_as_intmap(&test_data));
        let start: Coord = Coord::new(0,0);
        let end: Coord = Coord::new((map.get_height()-1) as isize, (map.get_width()-1) as isize);
        let mut node_indices:NodeMap = HashMap::new();
        let graph:DiGraph<Coord, usize> = build_graph(&map, &mut node_indices); 
        
        assert_eq!(silver(&start, &end, &graph, &node_indices), 102);
        //assert_eq!(gold(&test_data), 145);
    }

/*     #[test]
    fn test_silver() {
        let test_data:Vec<String> = read_data_from_file("input/real/17.txt");
        //assert_eq!(silver(&test_data), 510801);
    }

    #[test]
    fn test_gold() {
        let test_data:Vec<String> = read_data_from_file("input/real/17.txt");
        //assert_eq!(gold(&test_data), 212763);
    } */
}