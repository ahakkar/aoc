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

use std::cmp::{Reverse, Ordering};
use std::collections::{HashMap, VecDeque, BinaryHeap, HashSet};
use binary_tree::Node;
use petgraph::{graph::{DiGraph, NodeIndex}, algo::dijkstra, Direction::Incoming, visit::EdgeRef};
use super::utils::*;

const MAX_DIST: usize = 3;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Agent {
    cost: usize,
    pos: Coord,
    idx: NodeIndex,
    dir: Vec2D,
    steps: usize,
    prev: Coord,
}

impl Agent {
    pub fn new(
        cost: usize, pos: Coord, idx: NodeIndex, 
        dir: Vec2D, steps: usize, prev: Coord
    ) -> Agent {
        Agent {cost, pos, idx, dir, steps, prev}
    }
}

impl Ord for Agent {
    fn cmp(&self, other: &Agent) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Agent {
    fn partial_cmp(&self, other: &Agent) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn solve(data: Vec<String>) {    
    let map:GridMap<u8> = GridMap::new(data_as_intmap(&data));
    let mut node_indices:NodeMap = HashMap::new();
    let graph:DiGraph<Coord, usize> = build_graph(&map, &mut node_indices);

    println!("Silver: {}", silver(&map, &graph, &node_indices)); // 936 too high
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

// Reconstructs path based on edge weights... a bit clumsy way to do this
fn reconstruct_path(
    node_costs: &HashMap<NodeIndex, usize>,
    graph: &DiGraph<Coord, usize>,
    start: NodeIndex,
    end: NodeIndex,
) -> Vec<NodeIndex> {
    let mut path = Vec::new();
    let mut current = end;
    while current != start {
        path.push(current);

        let mut min_cost = usize::MAX;
        let mut predecessor = None;

        // Find the predecessor of the current node
        for edge in graph.edges_directed(current, Incoming) {
            let node = edge.source();
            let edge_cost = edge.weight();
            if let Some(&total_cost) = node_costs.get(&node) {
                let cost_to_current = total_cost + edge_cost;
                if (cost_to_current <= node_costs[&current] && 
                    cost_to_current < min_cost)
                {
                    min_cost = cost_to_current;
                    predecessor = Some(node);
                }
            }
        }

        current = predecessor.expect("No path found");
    }

    path.push(start);
    path.reverse();
    path
}

/*
function custom_dijkstra(graph, start, end):
    priority_queue = PriorityQueue()  
    priority_queue.add(start, cost=0, direction=None, steps=0)

    while not priority_queue.isEmpty():
        current, cost, direction, steps = priority_queue.pop()

        if current.position == end:
            return reconstruct_path(current)

        for neighbor in expandNode(current):
            if is_valid_transition(current, neighbor, direction, steps):
                new_cost = cost + edge_cost(current, neighbor)
                new_direction, new_steps = 
                    update_direction_and_steps(current, neighbor, direction, steps)
                priority_queue.add(
                    neighbor, 
                    cost=new_cost, 
                    direction=new_direction,
                    steps=new_steps)

    return failure

function is_valid_transition(current, neighbor, direction, steps):
    # Check if moving from current to neighbor is valid based on direction and steps
    # ...

function update_direction_and_steps(current, neighbor, direction, steps):
    # Determine new direction and step count after moving to neighbor
    # ...
     */


fn calc_dir(a: &Coord, b: &Coord) -> Vec2D {
    match (a.x.cmp(&b.x), a.y.cmp(&b.y)) {
        (Ordering::Equal,   Ordering::Less)     => NORTH,
        (Ordering::Equal,   Ordering::Greater)  => SOUTH,
        (Ordering::Less,    Ordering::Equal)    => EAST,
        (Ordering::Greater, Ordering::Equal)    => WEST,
         _                                      => STILL,
    }
}
     

fn custom_djikstra(
    map: &GridMap<u8>,
    graph: &DiGraph<Coord, usize>,
    node_indices: &NodeMap,
    start: &Coord,
    end: &Coord
) {
    if let (Some(&start_idx), Some(&end_idx)) = (
        node_indices.get(start), 
        node_indices.get(end)
    ) {
        let mut prioq = BinaryHeap::new();
        let mut visited = HashSet::new();
        prioq.push(Reverse(Agent::new(0,*start, start_idx, STILL, 0, *start)));

        while let Some(Reverse(agent)) = prioq.pop() {
            //println!("current pos: {:?}", agent.pos);
            if agent.pos == *end {
                return;
            }
            // Try to avoid death loops where agent fruitlessly revisits same states
            if visited.contains(&(agent.pos, agent.dir, agent.steps)) {
                continue;
            }
            visited.insert((agent.pos, agent.dir, agent.steps));

            // Iterate through edges, checking distances and directions
            // and if the edge continues in same dir, check if it would 
            // contain too many steps
            for edge in graph.edges(agent.idx) {
                let next_coord = graph.node_weight(edge.target()).unwrap(); 
                if *next_coord == agent.prev {
                    continue;
                }

                let new_dir = calc_dir(&agent.pos, next_coord);
                let distance = 
                    (next_coord.x - agent.pos.x).abs() +
                    (next_coord.y - agent.pos.y).abs();

                let new_steps = if new_dir == agent.dir {
                    agent.steps + distance as usize
                } else {
                    distance as usize
                };

                // Can move only 3 steps in the same direction
                if new_steps > 3 { continue; }
            
                let new_cost = agent.cost + edge.weight();
                let new_agent = Agent {
                    cost: new_cost,
                    pos: *next_coord,
                    idx: edge.target(),
                    dir: new_dir,
                    steps: new_steps,
                    prev: agent.pos,
                };
            
                prioq.push(Reverse(new_agent));
            }            
        }
        
    } else {
        panic!("Start or end node not found in the graph");
    }
}

fn silver(
    map: &GridMap<u8>,
    graph: &DiGraph<Coord, usize>,
    node_indices: &NodeMap
) -> usize { 
    let start: Coord = Coord::new(0,0);
    let end: Coord = Coord::new((map.get_height()-1) as isize, (map.get_width()-1) as isize);
    println!("start: {:?}, end: {:?}", start, end);
    custom_djikstra(map, graph, node_indices, &start, &end);
    0
}

/*
let node_costs = dijkstra(graph, start_idx, Some(end_idx), |e| *e.weight());
        //println!("costs:\n{:?}", node_costs);
        // Retrieve the cost to the end node
        if let Some(&cost) = node_costs.get(&end_idx) {
            let path = reconstruct_path(&node_costs, graph, start_idx, end_idx);
            for node in path {
                println!("node: {:?}", graph.node_weight(node).unwrap());
            }
            //return cost;
        } else {
            panic!("no path found between start and end when one should be found!");
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
        let test_data:Vec<String> = read_data_from_file("input/test/17.txt");
        let map:GridMap<u8> = GridMap::new(data_as_intmap(&test_data));
        let mut node_indices:NodeMap = HashMap::new();
        let graph:DiGraph<Coord, usize> = build_graph(&map, &mut node_indices); 
        
        assert_eq!(silver(&map, &graph, &node_indices), 102);
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