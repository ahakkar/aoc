
/*
 * 2023 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
 */

 #![allow(unused_parens)]
 #![allow(unused_imports)]
 #![allow(unused_variables)]
 #![allow(unused_mut)]
 #![allow(clippy::needless_return)]
 #![allow(clippy::needless_range_loop)]
 #![allow(dead_code)]
 #![allow(unused_assignments)]

use binary_tree::count;

use super::utils::print_map;

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
    fn new(width: usize, height: usize) -> Self {
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
    println!("Silver: {}", silver(&data)); // 108614 ok
    //println!("Gold: {}", gold(&data));
}

fn process_data<F>(data: &[String], processor: F) -> isize
where
    F: Fn(&[String]) -> isize,
{
    data.split(|row| row.is_empty())
        .map(processor)
        .sum()
}

// Move balls in the given direction, stopping at obstacles and map edges.
/* fn move_balls(map: &mut [Vec<i8>], cg: &CubeGrid, dir: (i8,i8)) {
    let w = map[0].len() as i8;/*  */
    let h = map.len() as i8;
    let (dx, dy) = dir;

    for y in 0..h {
        for x in 0..w {
            let nx = x+dx;
            let ny = y+dy;
            if  (ny >= 0 && ny < h) && (nx >= 0 && nx < w) &&
                map[y as usize][x as usize] == BALL && 
                map[ny as usize][nx as usize] == EMPTY 
            {
                map[ny as usize][nx as usize] = BALL;
                map[y as usize][x as usize] = EMPTY;
            } 
        }
    }
}
 */

// Move balls in the given direction, stopping at obstacles and map edges.
fn move_balls(map: &mut [Vec<i8>], cg: &CubeGrid, dir: (i8, i8)) {
    let w = map[0].len() as i8;
    let h = map.len() as i8;
    let (dx, dy) = dir;

    for y in 0..h {
        for x in 0..w {
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
        }
    }
}

fn count_weight(map: &[Vec<i8>]) -> usize {
    let mut sum: usize = 0;  
    for (y, row) in map.iter().rev().enumerate() {        
        let result = row.iter().filter(|&&n| n == BALL).count() * (y + OFFSET);
        //println!("y: {}, {:?}, result: {}", y+1, row, result);
        sum += result;
    }
    
    sum
}

fn silver(data: &Vec<String>) -> usize {      
    let rows = data.len();
    let cols = data[0].len();
    let mut cg: CubeGrid = CubeGrid::new(cols, rows);
    let mut map: Vec<Vec<i8>> = vec![vec![0; cols]; rows];
    

    // parse data...
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

    //print_map(&map);
    println!();

    // move balls around
    move_balls(&mut map, &cg, NORTH);
    //print_map(&map);

    // count load of balls on north edge
    count_weight(&map) 
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
    }

/*     #[test]
    fn test_silver() {
        let test_data:Vec<String> = read_data_from_file("input/real/13.txt");
        assert_eq!(process_data(&test_data, silver), 27502);
    }

    #[test]
    fn test_gold() {
        let test_data:Vec<String> = read_data_from_file("input/real/13.txt");
        assert_eq!(process_data(&test_data, gold), 31947);
    } */
}