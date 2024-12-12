
/*
 * 2023 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
*/
use std::collections::HashMap;

const OFFSET: usize = 1;

const EMPTY: i8 = 0; // Representing '.'
const CUBE:  i8 = 1;  // Representing '#'
const BALL:  i8 = 2;  // Representing 'O'

const NORTH: (i8,i8) = (0,-1);
const EAST:  (i8,i8) = (1,0);
const SOUTH: (i8,i8) = (0,1);
const WEST:  (i8,i8) = (-1,0);

struct CubeGrid {
    rows: Vec<u128>,
}

impl CubeGrid {
    fn new(height: usize) -> Self {
        let rows = vec![0; height];
        Self { rows }
    }

    fn set_obstacle(&mut self, x: usize, y: usize) {
        self.rows[y] |= 1 << x;
    }

    fn is_obstacle(&self, x: usize, y: usize) -> bool {
        (self.rows[y] & (1 << x)) != 0
    }
}

pub fn solve(data: Vec<String>) {
    println!("Silver: {}", silver(&data)); // 108614
    println!("Gold: {}", gold(&data)); // 96447
}

fn parse_map(data: &[String]) ->
(Vec<Vec<i8>>, CubeGrid) {
    let rows = data.len();
    let cols = data[0].len();
    let mut cg: CubeGrid = CubeGrid::new(rows);    
    let mut map: Vec<Vec<i8>> = vec![vec![0; cols]; rows];

    for (y, row) in data.iter().enumerate() {
        for (x, char) in row.chars().enumerate() {
            match char {
                '.' => map[y][x] = EMPTY,
                '#' => { map[y][x] = CUBE; cg.set_obstacle(x, y); },
                'O' => map[y][x] = BALL,
                 _  => panic!("unknown char at {}, {}", x, y),
            }
        }
    }
    (map, cg)
}

// A terrible monster from beyond..
fn move_balls(map: &mut [Vec<i8>], cg: &CubeGrid, dir: &(i8, i8)) {
    let w = map[0].len() as i8;
    let h = map.len() as i8;
    let (dx, dy) = dir;

    // Determine the iteration order based on direction
    let (start_x, end_x, step_x) = if dx > &0 { (w - 1, -1, -1) } else { (0, w, 1) };
    let (start_y, end_y, step_y) = if dy > &0 { (h - 1, -1, -1) } else { (0, h, 1) };

    let mut y = start_y;
    while y != end_y {
        let mut x = start_x;
        while x != end_x {
            if map[y as usize][x as usize] == BALL {
                let mut nx = x;
                let mut ny = y;

                // Move the ball as far as possible
                while (ny + dy >= 0) && (ny + dy < h) && (nx + dx >= 0) && (nx + dx < w) &&
                      (map[(ny + dy) as usize][(nx + dx) as usize] == EMPTY) &&
                      !cg.is_obstacle((nx + dx) as usize, (ny + dy) as usize) {
                    nx += dx;
                    ny += dy;
                }

                // Update the map if the ball has moved
                if (nx != x) || (ny != y) {
                    map[ny as usize][nx as usize] = BALL;
                    map[y as usize][x as usize] = EMPTY;
                }
            }
            x += step_x;
        }
        y += step_y;
    }
}

fn count_weight(map: &[Vec<i8>]) -> usize {
    let mut sum: usize = 0;  
    for (y, row) in map.iter().rev().enumerate() {        
        let result = row.iter().filter(|&&n| n == BALL).count() * (y + OFFSET);
          sum += result;
    }    
    sum
}

fn find_cycle(map: &mut [Vec<i8>], cg: &CubeGrid, n: &usize) -> usize {
    let mut seen_maps = HashMap::new();
    seen_maps.insert(map.to_vec(), 0);

    // Find start and end for a cycle
    let (start, end) = loop {
        for next_dir in [NORTH, WEST, SOUTH, EAST] {
            move_balls(map, cg, &next_dir);
        }

        let end = seen_maps.len();
        if let Some(start) = seen_maps.insert(map.to_vec(), end) {
            break (start, end);
        }
    };

    // simulate the result
    let target_index = ((n - start) % (end - start)) + start;
    let mut found_value = None;

    for (key, index) in seen_maps.iter() {
        if *index == target_index {
            found_value = Some(key);
            break;
        }
    }
    // Return the final weight
    count_weight(found_value.unwrap())
}

fn silver(data: &[String]) -> usize {      
    let (mut map, cg) = parse_map(data);
    move_balls(&mut map, &cg, &NORTH);
    count_weight(&map) 
}

fn gold(data: &[String]) -> usize {   
    let (mut map, cg) = parse_map(data);
    find_cycle(&mut map, &cg, &1_000_000_000)
}

// run these with cargo test --bin main -- day_13::tests
#[cfg(test)]
mod tests {
    use crate::utils::read_data_from_file;
    use super::*;   

    #[test]
    fn test_test() {
        let test_data:Vec<String> = read_data_from_file("input/test/14.txt");
        assert_eq!(silver(&test_data), 136);
        assert_eq!(gold(&test_data), 64);
    }

    #[test]
    fn test_silver() {
        let test_data:Vec<String> = read_data_from_file("input/real/14.txt");
        assert_eq!(silver(&test_data), 108614);
    }

    #[test]
    fn test_gold() {
        let test_data:Vec<String> = read_data_from_file("input/real/14.txt");
        assert_eq!(gold(&test_data), 96447);
    }
}