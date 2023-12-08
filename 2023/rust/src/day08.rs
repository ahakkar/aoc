use std::fs;
use std::time::Instant;
use std::collections::HashMap;
use num::integer::lcm;

fn main() {
    let start = Instant::now();
    let input = fs::read_to_string("input/08puzzle.txt").unwrap();
    //let input = fs::read_to_string("test_input/08test.txt").unwrap();
    let data: Vec<&str> = input.lines().collect();

    process(&data);

    let duration = start.elapsed();
    println!("Time elapsed in main() is: {:?}", duration);
}

fn build_tree(
    data: &[&str], 
    start_nodes: &mut Vec<String>
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
                String::from( l.trim_start_matches('(') ),
                String::from( r.trim_end_matches(')') ),
            ),
        );              
    }
    nodes
}

// Checks if node's 3rd char is 'char'
fn check_node(node: &str, char: &char) -> bool {
    node.chars().nth(2) == Some(*char) 
}

// traverse from start to end, counting iterations
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

/**
 * The tree contains repeating paths with specific intervals. For each
 * tree traversal, find the repeating interval. Their least common multiple
 * tells the total distance.
 */
fn gold(
    dirs: &str,
    nodes: &HashMap<String, (String, String)>,
    start_nodes: Vec<String>
) -> i64 {
    
    let mut endless_dir_iter = dirs.chars().cycle();
    let mut current_nodes = start_nodes;
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
                 _  => panic!(),
            }

            if check_node(node.as_str(), &'Z') && intervals[i] == 0 {
                intervals[i] = steps + 1;
            }
        }

        steps += 1;
    }
    println!("{:?}", intervals); // [20093, 12169, 13301, 20659, 16697, 17263]

    // Compute the LCM of the intervals
    intervals.into_iter().fold(1, lcm)
}


fn process(data: &[&str]) {   
    let mut start_nodes: Vec<String> = vec![]; 
    let nodes = build_tree(data, &mut start_nodes);  

    println!("Silver: {}", silver(data.first().unwrap(), &nodes)); // 16697
    println!("Gold: {}", gold(data.first().unwrap(), &nodes, start_nodes)); // 10 668 805 667 831

}