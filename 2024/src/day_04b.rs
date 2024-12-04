/*
 * 2024 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

use super::utils::*;

pub fn silver(data: &[String]) -> usize {
    let mut sum: usize = 0;    
    let map:GridMap<char> = GridMap::new(data_as_chars(data)); 
    let height = data.len();
    let width  = data[0].len();

    const PATTERN1: [char; 4] = ['X', 'M', 'A', 'S'];
    const PATTERN2: [char; 4] = ['S', 'A', 'M', 'X'];

    for y in 0..height {
        for x in 0..width {    
            if let Some(&idx) = map.get_idx(x, y) {
                if idx != 'X' && idx != 'S' {
                    continue;
                }

                for dir in [Direction::North, Direction::NorthEast, 
                            Direction::East, Direction::SouthEast]
                {
                    let mut loc = Coord::new(x as isize, y as isize);
                    let mut pat1_match = true;
                    let mut pat2_match = true;

                    for i in 0..4 {  
                        // Reduce unneccessary loops                 
                        if  loc.x < 0 || loc.x >= width as isize || 
                            loc.y < 0 || loc.y >= height as isize
                        {
                            pat1_match = false;
                            pat2_match = false;
                            break;
                        }

                        // Reduce unneccessary loops  
                        let c = map.get_cell(&loc).unwrap_or('_');
                        if c != PATTERN1[i] { pat1_match = false }
                        if c != PATTERN2[i] { pat2_match = false }            
                        if !pat1_match && !pat2_match { break }
    
                        loc = loc.neighbour(dir);
                    }

                    if pat1_match || pat2_match {
                        sum += 1;
                    }                
                }
            }
        }
    }

    sum
}

pub fn gold(data: &[String]) -> usize {
    let mut sum: usize = 0;    
    let map:GridMap<char> = GridMap::new(data_as_chars(data)); 

    for y in 1..(data.len() - 1) {
        for x in 1..(data[0].len() - 1) {
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
    sum
}

// cargo test --bin main -- day_XX::tests
#[cfg(test)]
mod tests {
    use crate::utils::read_data_from_file;
    use super::*;   

    #[test]
    fn test_test() {
        let test_data:Vec<String> = read_data_from_file("input/test/04.txt");
        assert_eq!(silver(&test_data), 18);
        assert_eq!(gold(&test_data), 9);
    }

    #[test]
    fn test_silver() {
        let test_data:Vec<String> = read_data_from_file("input/real/04.txt");
        assert_eq!(silver(&test_data), 2718);
    }

    #[test]
    fn test_gold() {
        let test_data:Vec<String> = read_data_from_file("input/real/04.txt");
        assert_eq!(gold(&test_data), 2046);
    }
}