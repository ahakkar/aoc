#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(clippy::needless_return)]
#![allow(clippy::needless_range_loop)]

use std::fs;
use std::fmt;
use std::time::Instant;
use std::collections::HashMap;

fn main() {
    let start = Instant::now();
    let input = fs::read_to_string("input/08puzzle.txt").unwrap();
    let test_input = "RL

    AAA = (BBB, CCC)
    BBB = (DDD, EEE)
    CCC = (ZZZ, GGG)
    DDD = (DDD, DDD)
    EEE = (EEE, EEE)
    GGG = (GGG, GGG)
    ZZZ = (ZZZ, ZZZ)";

    let test_input2 = "LLR

    AAA = (BBB, BBB)
    BBB = (AAA, ZZZ)
    ZZZ = (ZZZ, ZZZ)
    ";

    let data: Vec<&str> = input.lines().collect();

    process(&data);

    let duration = start.elapsed();
    println!("Time elapsed in main() is: {:?}", duration);
}

fn build_tree(data: &[&str], start_nodes: &mut Vec<String>) -> HashMap<String, (String, String)> {
    let mut nodes: HashMap<String, (String, String)> = HashMap::new();        

    for row in 2..data.len() {
        let (parent, children) = data.get(row).unwrap().split_once(" = ").unwrap();
        let (l, r) = children.split_once(", ").unwrap();   

        if parent.chars().nth(2) == Some('A') {
            start_nodes.push(String::from(parent));
        }

        nodes.insert(
            String::from(parent.trim()),
            (
                String::from( l.trim_start_matches('(') ),
                String::from( r.trim_end_matches(')') ),
            ),
        );              
    }
    nodes
}

fn silver(dirs: &str, nodes: &HashMap<String, (String, String)>) -> i64 {
    let mut endless_dir_iter = dirs.chars().cycle();  
    let mut current_node:String = String::from("AAA");
    let mut dist: i64 = 0;

    while current_node != *"ZZZ" {        
        let n = nodes.get(&current_node).unwrap();
        match endless_dir_iter.next().unwrap() {
            'L' => current_node = n.0.clone(),
            'R' => current_node = n.1.clone(),
             _  => panic!(),
        };
        dist += 1;
    }
    dist
}

fn gold(dirs: &str, nodes: &HashMap<String, (String, String)>, start_nodes: &Vec<String>) -> i64 {
    let mut endless_dir_iter = dirs.chars().cycle();
    println!("{:?}", start_nodes);
    0
}

fn process(data: &[&str]) {   
    let mut start_nodes: Vec<String> = vec![]; 
    let nodes = build_tree(data, &mut start_nodes);  

    println!("Silver: {}", silver(data.first().unwrap(), &nodes)); // 16697
    println!("Gold: {}", gold(data.first().unwrap(), &nodes, &start_nodes));

}