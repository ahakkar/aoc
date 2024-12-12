/*
 * 2023 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref REGEX_GAME_ID: regex::Regex = Regex::new(r"\d+").unwrap();
}

pub fn solve(data: Vec<String>) {    
    println!("Silver: {}", silver(&data)); // 2683
    println!("Gold: {}", gold(&data)); // 49710
}

fn silver(data: &[String]) -> usize {
    let mut sum:usize = 0;

    for row in data {
        let mut parts_iter = row.split(':');

        let game_id = REGEX_GAME_ID.find(parts_iter.next().unwrap())
                        .unwrap().as_str().parse::<usize>().unwrap();

        if check_silver_game(parts_iter.next().unwrap()) {
            sum += game_id;
        }
    }
    sum
}

fn gold(data: &Vec<String>) -> usize {
    let mut sum: usize = 0;    

    for row in data {
        let parts_iter = row.split(':');
        sum += check_gold_game(parts_iter.last().unwrap());        
    }
    sum 
}

// only 12 red cubes, 13 green cubes, and 14 blue cubes?
fn check_silver_game(games: &str) -> bool {
    let games_iter = games.split(';');
    for game in games_iter {
        let colors_iter = game.split(',');
        for color in colors_iter {
            let mut parts_iter = color.trim().split(' ');
            let num = parts_iter.next().unwrap().parse::<usize>().unwrap();
            let rgb = parts_iter.next().unwrap().chars().next().unwrap();

            if (rgb == 'r' && num > 12) ||
                (rgb == 'g' && num > 13) ||
                (rgb == 'b' && num > 14)
            {                    
                return false;
            }
        }
    }    
    true
}

// calculate power of max r*g*b
fn check_gold_game(games: &str) -> usize {
    let games_iter = games.split(';');
    let mut max_r:usize = 0;
    let mut max_g:usize = 0;
    let mut max_b:usize = 0;

    for game in games_iter {
        let colors_iter = game.split(',');

        for color in colors_iter {
            let mut parts_iter = color.trim().split(' ');
            let num = parts_iter.next().unwrap().parse::<usize>().unwrap();
            let rgb = parts_iter.next().unwrap().chars().next().unwrap();

            if rgb == 'r' && num > max_r {
                max_r = num;
            } else if rgb == 'g' && num > max_g {
                max_g = num;
            } 
            if rgb == 'b' && num > max_b {
                max_b = num;
            }  
        }
    }
    max_r * max_g * max_b    
}

// run these with cargo test --bin main -- day_XX::tests
#[cfg(test)]
mod tests {
    use crate::utils::read_data_from_file;
    use super::*;   

    #[test]
    fn test_silver() {
        let test_data:Vec<String> = read_data_from_file("input/real/02.txt");
        assert_eq!(silver(&test_data), 2683);
    }

    #[test]
    fn test_gold() {
        let test_data:Vec<String> = read_data_from_file("input/real/02.txt");
        assert_eq!(gold(&test_data), 49710);
    }
}