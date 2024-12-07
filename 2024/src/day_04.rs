/*
 * 2024 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

use regex::Regex;

use crate::{utils::{data_as_chars, Coord, Direction, GridMap}, Fro, Solution, TaskResult};

pub struct CeresSearch {
    data: Vec<String>
}

impl Fro for CeresSearch {
    fn fro(data: &str) -> Self{
        Self { data: data.split('\n').map(|line| line.to_string()).collect() }
    }
}

impl Solution for CeresSearch {
    fn silver(&self) -> TaskResult {
        let f = Regex::new(r"XMAS").unwrap();
        let b = Regex::new(r"SAMX").unwrap();
        let mut sum: usize = 0;    
        let horz = self.data.join(" ");
        let (vert, dg_r, dg_l) = Self::flatten_data(&self.data);

        sum += f.find_iter(&horz).count();
        sum += b.find_iter(&horz).count();    
        sum += f.find_iter(&vert).count();
        sum += b.find_iter(&vert).count();   
        sum += f.find_iter(&dg_r).count();
        sum += b.find_iter(&dg_r).count();  
        sum += f.find_iter(&dg_l).count();
        sum += b.find_iter(&dg_l).count();  

        TaskResult::Usize(sum)
    }

    fn gold(&self) -> TaskResult {
        let mut sum: usize = 0;    
        let map:GridMap<char> = GridMap::new(data_as_chars(&self.data)); 

        for y in 1..(self.data.len() - 1) {
            for x in 1..(self.data[0].len() - 1) {
                if map.get_idx(x, y) != Some(&'A') { continue }
                let idx = Coord::new(x as isize, y as isize);

                let nw = map.get_neighbor(&idx, Direction::NorthWest);
                let ne = map.get_neighbor(&idx, Direction::NorthEast);
                let sw = map.get_neighbor(&idx, Direction::SouthWest);
                let se = map.get_neighbor(&idx, Direction::SouthEast);

                if (nw == Some('M') && se == Some('S') ||
                    nw == Some('S') && se == Some('M'))
                    && 
                (ne == Some('M') && sw == Some('S') ||
                    ne == Some('S') && sw == Some('M'))
                {
                    sum += 1;
                } 
            }
        }
        TaskResult::Usize(sum)
    }
}

impl CeresSearch {
    fn get_char(data: &[String], y: usize, x: usize) -> char {
        data
            .get(y)
            .and_then(|row| row.get(x..x+1))
            .unwrap()
            .chars()
            .next()
            .unwrap()
    }

    fn flatten_data(data: &[String]) -> (String, String, String) {
        let mut vert = String::new();
        let mut r_dg = String::new();
        let mut l_dg = String::new();
        let (rows, cols) = (data.len(), data[0].len());

        for x in 0..cols {
            for y in 0..rows {
                vert.push(Self::get_char(data, y, x));
            }
            vert.push(' ');
        }

        // Right diagonal
        for col in 0..cols {
            let mut x = col;
            let mut y = 0;
            while x < cols && y < rows {
                r_dg.push(Self::get_char(data, y, x));
                x += 1;
                y += 1;
            }
            r_dg.push(' ');
        }
        for row in 1..rows {
            let mut x = 0;
            let mut y = row;
            while x < cols && y < rows {
                r_dg.push(Self::get_char(data, y, x));
                x += 1;
                y += 1;
            }
            r_dg.push(' ');
        }


        // Left diagonal
        for col in (0..cols).rev() {
            let mut x = col;
            let mut y = 0;
            while x < cols && y < rows {
                l_dg.push(Self::get_char(data, y, x));
                if x == 0 { break; }
                x -= 1;
                y += 1;
            }
            l_dg.push(' ');
        }
        for row in 1..rows {
            let mut x = cols - 1;
            let mut y = row;
            while x < cols && y < rows {
                l_dg.push(Self::get_char(data, y, x));
                if x == 0 { break; }
                x -= 1;
                y += 1;
            }
            l_dg.push(' ');
        }

        (vert, r_dg, l_dg)
    }
}

// cargo test --bin main -- day_XX::tests
#[cfg(test)]
mod tests {
    use crate::utils::read_data_from_file;   
    use super::*;   

    #[test]
    fn test_test() {  
        let test_data = read_data_from_file("input/test/04.txt"); 
        let queue = CeresSearch::fro(&test_data);        
    
        assert_eq!(queue.silver(), TaskResult::Usize(18));
        assert_eq!(queue.gold(), TaskResult::Usize(9));
    }

    #[test]
    fn test_real() {
        let real_data = read_data_from_file("input/real/04.txt");
        let queue = CeresSearch::fro(&real_data);        
    
        assert_eq!(queue.silver(), TaskResult::Usize(2718));
        assert_eq!(queue.gold(), TaskResult::Usize(2046));

    }
}