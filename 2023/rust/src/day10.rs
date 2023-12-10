/*
 * 2023 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
 */
use lazy_static::lazy_static;
use petgraph::graph::{UnGraph, NodeIndex};
use petgraph::visit::DfsPostOrder;
use std::collections::{HashSet, HashMap, VecDeque};
use std::fs::{File, self};
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

struct PipeGraph {
    graph: Option<UnGraph<Coord, (), u32>>,
    path: Option<Vec<NodeIndex>>,
    map_width: i32,
    map_height: i32,
    chars: Option<Vec<Vec<char>>>,
}


impl PipeGraph {
    fn new(map_width: i32, map_height: i32) -> Self {
        PipeGraph {
            graph: None,
            path: None,
            map_width,
            map_height,
            chars: None,
        }
    }

    fn set_graph(&mut self, graph: UnGraph<Coord, (), u32>) {
        self.graph = Some(graph);
    }

    fn set_path(&mut self, path: Vec<NodeIndex>) {
        self.path = Some(path);
    }

    fn set_chars(&mut self, chars: &[Vec<char>]) {
        self.chars = Some(chars.to_vec());
    }
}


pub fn solve(data: Vec<String>) {
    let mut graph_data: PipeGraph = PipeGraph::new(
        data[0].len().try_into().unwrap(),
        data.len().try_into().unwrap()
    );
 
    let mut data_as_chars: Vec<Vec<char>> = vec![];
    for row in data {
        data_as_chars.push(row.chars().collect::<Vec<char>>());
    }
    graph_data.set_chars(&data_as_chars);

    // F: 7702 not correct
    // L J 7 all the same answer, 7702. 7700 too high also.
    // 6820 works
    if let Some(result) = silver(&data_as_chars, graph_data.map_width, graph_data.map_height) {
        println!("Silver: {}", (result.1.len()+1)/2);   

        graph_data.set_graph(result.0);
        graph_data.set_path(result.1);
        gold(&graph_data);
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

fn get_enhanced_char_representation(e: char, p: char) -> HashMap<char, Vec<Vec<char>>> {
    let mut representations = HashMap::new();
    representations.insert('L', vec![
        vec![e, p, e],
        vec![e, p, p],
        vec![e, e, e],
    ]);

    representations.insert('J', vec![
        vec![e, p, e],
        vec![p, p, e],
        vec![e, e, e],
    ]);

    representations.insert('F', vec![
        vec![e, e, e],
        vec![e, p, p],
        vec![e, p, e],
    ]);

    representations.insert('7', vec![
        vec![e, e, e],
        vec![p, p, e],
        vec![e, p, e],
    ]);

    representations.insert('|', vec![
        vec![e, p, e],
        vec![e, p, e],
        vec![e, p, e],
    ]);

    representations.insert('-', vec![
        vec![e, e, e],
        vec![p, p, p],
        vec![e, e, e],
    ]);

    representations
}

fn write_path_to_2d_map_file(graph_data: &PipeGraph, file_path: &str) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    let mut map = vec![vec![' '; (graph_data.map_width as usize) * 3]; (graph_data.map_height as usize) * 3];
    let representations = get_enhanced_char_representation(' ', 'X');

    if let Some(graph) = &graph_data.graph {
        if let Some(path) = &graph_data.path {
            if let Some(chars) = &graph_data.chars {
                // draw the path with enhanced 3x3 chars
                for node in path {
                    let coord: Coord = *graph.node_weight(*node).unwrap();
                    if let Some(representation) = representations.get(&chars[coord.y][coord.x]) {
                        for (dy, row) in representation.iter().enumerate() {
                            for (dx, &cell) in row.iter().enumerate() {
                                map[coord.y * 3 + dy][coord.x * 3 + dx] = cell;
                            }
                        }
                    }
                }
                let io_op_res = write_fill_to_file(&map, "3x3_enhanced_map.txt");
            }            
        }        
    }    

    Ok(())
}

fn flood_fill(grid: &mut Vec<Vec<char>>, start_x: usize, start_y: usize, fill_char: char) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut stack = vec![(start_x, start_y)];
    let mut count = 0;

    while let Some((x, y)) = stack.pop() {
        if x >= cols || y >= rows || grid[y][x] != ' ' {
            continue;
        }

        // Fill the cell and increment the count
        grid[y][x] = fill_char;
        count += 1;

        // Add neighboring cells to the stack
        if x > 0 { stack.push((x - 1, y)); }
        if x + 1 < cols { stack.push((x + 1, y)); }
        if y > 0 { stack.push((x, y - 1)); }
        if y + 1 < rows { stack.push((x, y + 1)); }
    }

    let io_op_res = write_fill_to_file(grid, "flood_filled_inside.txt");

    count
}

fn write_fill_to_file(grid: &Vec<Vec<char>>, file_path: &str) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    
    for row in grid {
        for cell in row {
            write!(file, "{}", cell)?;
        }
        writeln!(file)?;
    }
    Ok(())
}

// since pipe always overlaps the middle char of a 3x3 area, count areas
// where the middle char is empty
fn count_3x3_spaces(map: &[Vec<char>]) -> usize {
    let mut count = 0;
    for y in (0..map.len() - 2).step_by(3) {
        for x in (0..map[0].len() - 2).step_by(3) {
            if map[y + 1][x + 1] == ' ' {
                count += 1;
            }
        }
    }
    count
}


// used this to generate a 3x3 representation of the pipe map
// then used flood fill on outside to generate another txt
// counted empty spaces in steps of 3 from the flood filled map
fn gold(graph: &PipeGraph) {
    //let io_op_res = write_path_to_2d_map_file(graph, "gold_map_enhance_more2.txt");

    let mut char_count:i32 = 0;
    let input = fs::read_to_string("flood_filled.txt").unwrap();
    let data: Vec<&str> = input.lines().collect();
    let mut data_as_chars: Vec<Vec<char>> = vec![];

    for row in data {
        data_as_chars.push(row.chars().collect::<Vec<char>>());
    }

    //println!("filled count: {}", flood_fill(&mut data_as_chars, 214, 214, 'I'));

    let count = count_3x3_spaces(&data_as_chars);
    println!("Gold: {}", count); // 3033 "not the right answer" .. 337 new guess..
    
    //println!("gold: {}", image.chars().filter(|&c| c == e).count()); // 194 too low, 204 too low // 210 too low
    


}