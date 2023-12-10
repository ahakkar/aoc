#![allow(unused_assignments)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(clippy::needless_return)]
#![allow(clippy::needless_range_loop)]


/*
 * 2023 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
 */

use lazy_static::lazy_static;
use petgraph::data::Build;
use petgraph::graph::{UnGraph, NodeIndex};
use petgraph::visit::{Bfs, IntoNodeReferences};
use std::collections::{HashSet, HashMap};

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
    println!("Silver: {}", silver(&data));
    //println!("Gold: {}", gold(&data));
}

fn parse_node(char: &char) {
    match &char {
        '|' => (),
        '-' => (),
        'L' => (),
        'J' => (),
        '7' => (),
        'F' => (),
        'S' => (),
         _  => panic!("Unknown char while parsing node map"),
    }
}

fn fits_bounds(coord: &Coord, x_change: i32, y_change: i32, map_width: usize, map_height: usize) -> bool {
    let new_x = coord.x as i32 + x_change;
    let new_y = coord.y as i32 + y_change;

    new_x >= 0 &&
    new_y >= 0 &&
    (new_x as usize) < map_width &&
    (new_y as usize) < map_height
}


// O M G
fn get_neighbours(c: &Coord, char: &char, map_width: usize, map_height: usize) -> Vec<Coord> {
    let mut neighbours: Vec<Coord> = vec![];
    match &char {
        '|' => {
            if fits_bounds(c, 0, -1, map_width, map_height) {
                neighbours.push(Coord { x: c.x, y: c.y - 1 } );
            }
            if fits_bounds(c, 0, 1, map_width, map_height) {
                neighbours.push(Coord { x: c.x, y: c.y + 1 });
            }
        },
        '-' =>  {
            if fits_bounds(c, -1, 0, map_width, map_height) {
                neighbours.push(Coord { x: c.x - 1, y: c.y } );
            }
            if fits_bounds(c, 1, 0, map_width, map_height) {
                neighbours.push(Coord { x: c.x + 1, y: c.y });
            }
        },
        'L' => {
            if fits_bounds(c, 0, -1, map_width, map_height) {
                neighbours.push(Coord { x: c.x, y: c.y -1 } );
            }
            if fits_bounds(c, 0, 1, map_width, map_height) {
                neighbours.push(Coord { x: c.x + 1, y: c.y });
            }
        },
        'J' => {
            if fits_bounds(c, 0, -1, map_width, map_height) {
                neighbours.push(Coord { x: c.x, y: c.y - 1 } );
            }
            if fits_bounds(c, -1, 0, map_width, map_height) {
                neighbours.push(Coord { x: c.x - 1, y: c.y });
            }
        },
        '7' => {
            if fits_bounds(c, -1, 0, map_width, map_height) {
                neighbours.push(Coord { x: c.x - 1, y: c.y } );
            }
            if fits_bounds(c, 0, 1, map_width, map_height) {
                neighbours.push(Coord { x: c.x + 1, y: c.y });
            }
        },
        'F' => {
            if fits_bounds(c, 1, 0, map_width, map_height) {
                neighbours.push(Coord { x: c.x + 1, y: c.y } );
            }
            if fits_bounds(c, 0, 1, map_width, map_height) {
                neighbours.push(Coord { x: c.x, y: c.y + 1 });
            }
        },
        'S' => (),
         _  => panic!("Unknown char while parsing node map"),
    }   

    neighbours
}

fn silver(data: &Vec<String>) -> i64 {
    let mut sum: i64 = 0;    
    let mut graph = UnGraph::<Coord, ()>::new_undirected();
    let mut ctni: HashMap<Coord, NodeIndex> = HashMap::new();
    let mut start: Coord;
    let map_width: usize = data[0].len();
    let map_height: usize = data.len();

    // Add nodes
    for (y, row) in data.iter().enumerate() {
        for (x, char) in row.chars().enumerate() {
            if NODE_CHARS.contains(&char) {
                let coord = Coord{x, y};
                let node_index = graph.add_node(coord);
                ctni.insert(coord, node_index);
            }
            if char == 'S' { start = Coord{x, y}; }
        }
    }

    // Add edges between nodes 
    // UGLY AS A SIN, A HORRIBLE PYRAMID OF DOOM.. sorry :((((
    for (y, row) in data.iter().enumerate() {
        for (x, char) in row.chars().enumerate() {
            if NODE_CHARS.contains(&char) {
                let coord = Coord{x, y};
                if let Some(node_index) = ctni.get(&coord) {
                    for neighbour_coord in get_neighbours(&coord, &char, map_width, map_height) {
                        if let Some(neighbour_index) = ctni.get(&neighbour_coord) {
                            graph.add_edge(*node_index, *neighbour_index, ());
                        }
                    }
                }
            }
        }
    }
    println!("{:?}", graph);
    sum 
}

/* fn gold(data: &Vec<String>) -> i64 {
    let mut sum: i64 = 0;    

    for row in data {
           } 
    sum 
} */