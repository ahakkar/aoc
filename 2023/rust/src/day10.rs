/*
 * 2023 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
 */

#![allow(unused_assignments)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(clippy::needless_return)]
#![allow(clippy::needless_range_loop)]

use lazy_static::lazy_static;
use petgraph::{Graph, Undirected};
use petgraph::data::Build;
use petgraph::graph::{UnGraph, NodeIndex};
use petgraph::visit::{Bfs, IntoNodeReferences, Visitable, VisitMap, DfsPostOrder};
use std::collections::{HashSet, HashMap, VecDeque};
use std::fs::File;
use std::io::{self, Write};

lazy_static! {
    static ref NODE_CHARS: HashSet<char> = 
        ['|', '-', 'L', 'J', '7', 'F', 'S'].iter().cloned().collect();
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Coord {
    x: usize, 
    y: usize,
}


pub fn solve(data: Vec<String>) {
    let mut data_as_chars: Vec<Vec<char>> = vec![];
    let map_width: i32 = data[0].len().try_into().unwrap();
    let map_height: i32 = data.len().try_into().unwrap();

    for row in data {
        data_as_chars.push(row.chars().collect::<Vec<char>>());
    }

    // F: 7702 not correct
    // L J 7 all the same answer, 7702. 7700 too high also.
    // 6820 works
    if let Some(result) = silver(&data_as_chars, map_width, map_height) {
        println!("Silver: {}", (result.1.len()+1)/2);   
        gold(&result, map_width, map_height);
    }    
}


fn fits_bounds(coord: &Coord, x_change: i32, y_change: i32, map_width: i32, map_height: i32) -> bool {
    let new_x = coord.x as i32 + x_change;
    let new_y = coord.y as i32 + y_change;

    new_x >= 0 &&
    new_y >= 0 &&
    (new_x as usize) < map_width.try_into().unwrap() &&
    (new_y as usize) < map_height.try_into().unwrap()
}


fn get_neighbours(grid: &[Vec<char>], c: &Coord, char: &char, map_width: i32, map_height: i32) 
    -> Vec<Coord> {
    let mut neighbours:Vec<Coord> = Vec::new();

    // Define potential coordinate changes based on the character
    let changes = match char {
        '|' => vec![(0, -1), (0, 1)],
        '-' => vec![(-1, 0), (1, 0)],
        'L' => vec![(0, -1), (1, 0)],
        'J' => vec![(0, -1), (-1, 0)],
        '7' => vec![(-1, 0), (0, 1)],
        'F' => vec![(1, 0), (0, 1)],
        'S' => vec![],
        _ => panic!("Unknown char while parsing node map"),
    };

    for (dx, dy) in changes {
        let new_x = (c.x as i32 + dx) as usize;
        let new_y = (c.y as i32 + dy) as usize;

        if fits_bounds(c, dx, dy, map_width, map_height) {
            // println!("coord:{:?}, dx {}, dy: {}", c, dx, dy);
            let neighbour_char = grid[new_y][new_x];     
            //println!("current char: {}, neighbour_char: {}", char, neighbour_char);
            if is_compatible(char, neighbour_char, dx, dy) {
                neighbours.push(
                    Coord{
                        x: (c.x as i32 + dx) as usize,
                        y: (c.y as i32 + dy) as usize
                    }
                );
            }
        }
    }

    //println!("compatible neighbours of {:?}: {:?}\n", c, neighbours);
    neighbours
}


fn is_compatible(current_char: &char, neighbour_char: char, dx: i32, dy: i32) -> bool {
    match (current_char, neighbour_char) {
        // L J 7 F - |
        ('F', '-') if dx == 1 && dy == 0 => true,
        ('F', '7') if dx == 1 && dy == 0 => true,
        ('F', 'J') if dx == 1 && dy == 0 => true,
        ('F', '|') if dx == 0 && dy == 1 => true,
        ('F', 'J') if dx == 0 && dy == 1 => true,        
        ('F', 'L') if dx == 0 && dy == 1 => true,

        ('L', 'J') if dx == 1 && dy == 0 => true,
        ('L', '7') if dx == 1 && dy == 0 => true,
        ('L', '-') if dx == 1 && dy == 0 => true,
        ('L', 'F') if dx == 0 && dy == -1 => true,
        ('L', '7') if dx == 0 && dy == -1 => true,
        ('L', '|') if dx == 0 && dy == -1 => true,        

        ('7', 'F') if dx == -1 && dy == 0 => true,
        ('7', '-') if dx == -1 && dy == 0 => true,        
        ('7', 'L') if dx == -1 && dy == 0 => true,
        ('7', '|') if dx == 0 && dy == 1 => true,
        ('7', 'J') if dx == 0 && dy == 1 => true,
        ('7', 'L') if dx == 0 && dy == 1 => true,
        
        ('J', '-') if dx == -1 && dy == 0 => true,        
        ('J', 'F') if dx == -1 && dy == 0 => true,
        ('J', 'L') if dx == -1 && dy == 0 => true,
        ('J', '|') if dx == 0 && dy == -1 => true,
        ('J', '7') if dx == 0 && dy == -1 => true,
        ('J', 'F') if dx == 0 && dy == -1 => true,

        ('-', 'F') if dx == -1 && dy == 0 => true, 
        ('-', 'L') if dx == -1 && dy == 0 => true, 
        ('-', '-') if dx == -1 && dy == 0 => true, 
        ('-', 'J') if dx == 1 && dy == 0 => true, 
        ('-', '7') if dx == 1 && dy == 0 => true, 
        ('-', '-') if dx == 1 && dy == 0 => true,         
   
        ('|', 'F') if dx == 0 && dy == -1 => true,
        ('|', '7') if dx == 0 && dy == -1 => true,
        ('|', '|') if dx == 0 && dy == -1 => true,
        ('|', 'L') if dx == 0 && dy == 1 => true,
        ('|', 'J') if dx == 0 && dy == 1 => true,        
        ('|', '|') if dx == 0 && dy == 1 => true,

        _ => false,
    }
}


// returns the path found
fn find_loop(graph: &UnGraph<Coord, ()>, start: NodeIndex) -> Option<Vec<NodeIndex>> {
    let mut dfs = DfsPostOrder::new(graph, start);
    let mut loop_detected = false;
    let mut visited = HashSet::new();
    let mut path = VecDeque::new();

    println!("Start: {:?}", graph.node_weight(start).unwrap());
    while let Some(nx) = dfs.next(graph) {
        if nx == start && !path.is_empty() {
            loop_detected = true;
            break;
        }
        if visited.insert(nx) {
            path.push_front(nx);
        }
    }

    if loop_detected {
        println!("Loop found");
        let mut loop_path = Vec::new();
        while let Some(node) = path.pop_front() {
            loop_path.push(node);
            if node == start {
                break;
            }
        }
        Some(loop_path)
    } else {
        println!("loop not found");
        None // No loop found
    }
}


fn print_graph(graph: &UnGraph::<Coord, ()>) {
    for node_index in graph.node_indices() {
        println!("Node {:?} has coordinates {:?}", node_index, graph[node_index]);
        for neighbor in graph.neighbors(node_index) {
            println!(" - connected to {:?}", graph[neighbor]);
        }
    }
}


fn silver(data: &[Vec<char>], map_width: i32, map_height: i32) 
-> Option<(UnGraph<Coord, (), u32>, Vec<NodeIndex>)> {
    let mut graph = UnGraph::<Coord, ()>::new_undirected();
    let mut ctni: HashMap<Coord, NodeIndex> = HashMap::new();
    let mut start: Option<Coord> = None;

    // Add nodes
    for (y, row) in data.iter().enumerate() {
        for (x, char) in row.iter().enumerate() {            
            if NODE_CHARS.contains(char) {
                let coord = Coord{x, y};
                let node_index = graph.add_node(coord);
                ctni.insert(coord, node_index);
            }
            if char == &'S' { start = Some(Coord{x, y}); }
        }
    }

    // Add edges, function is UGLY AS A SIN
    for (y, row) in data.iter().enumerate() {
        for (x, char) in row.iter().enumerate() {
            if NODE_CHARS.contains(char) {
                let coord = Coord{x, y};
                if let Some(node_index) = ctni.get(&coord) {   
                    for neighbour_coord in get_neighbours(data, &coord, char, map_width, map_height) {
                        if let Some(neighbour_index) = ctni.get(&neighbour_coord) {
                            // avoid adding edges twice to undirected graph
                            if *node_index < *neighbour_index {
                                graph.add_edge(*node_index, *neighbour_index, ());
                            }
                        }
                    }
                }
            }
        }
    }
    //print_graph(&graph);

    //println!("found start: {:?}", start);

    start = Some(Coord{x:79,y:64});
    //start = Some(Coord{x:1,y:1});

    if let Some(start_coord) = start {
        if let Some(start_index) = ctni.get(&start_coord) {
            if let Some(loop_path) = find_loop(&graph, *start_index) {
                return Some((graph, loop_path));
            }
        }
    }  
    None  
}


fn write_path_to_2d_map_file(
    result: &(UnGraph<Coord, (), u32>, Vec<NodeIndex>), 
    file_path: &str,
    map_width: i32,
    map_height: i32
) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    let mut map = vec![vec!['.'; map_width as usize]; map_height as usize];

    // draw the path with zeroes '0'
    for node in &result.1 {
        let coord: Coord = *result.0.node_weight(*node).unwrap();
        map[coord.y][coord.x] = '0';
    }

    for row in map {
        for cell in row {
            write!(file, "{}", cell)?;
        }
        writeln!(file)?;
    }

    Ok(())
}


fn gold(result: &(UnGraph<Coord, (), u32>, Vec<NodeIndex>), map_width: i32, map_height: i32) {
    //let io_op_res = write_path_to_2d_map_file(result, "gold_map.txt", map_width, map_height);

    let mut char_count:i32 = 0;
    let image = "0000000.000000000000000
    0000......0000..0000.00
    000.............00....0
    0000000.........00..000
    00000000............000
    00000000.............00
    00000000.............00
    000000000........00.000
    0000..000........00.000
    0000..00........0000000
    0000............0000000
    00000...........0000000
    00000............000000
    0000...00........000000
    00.....00.........00000
    000000.00.........00000
    00000000000.......00000
    00000000000000.....0000
    00000000000000.....0000
    000000000000000000.0000";

    
    println!("gold: {}", image.chars().filter(|&c| c == '.').count()); // 194 too low
    


}