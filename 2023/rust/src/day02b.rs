#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(clippy::needless_return)]

use std::collections::HashMap;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::time::Instant;
use lazy_static::lazy_static;

lazy_static! {
    static ref REGEX_GAME_ID: regex::Regex = Regex::new(r"\d+").unwrap();
}

fn main() {
    let start = Instant::now();
    let data: Result<Vec<String>, io::Error> = read_file("input/02puzzle.txt");

    if let Ok(lines) = data {
        process(&lines);
    }
    let duration = start.elapsed();
    println!("Time elapsed in main() is: {:?}", duration);
}

fn read_file(file_name: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    let mut data: Vec<String> = vec![];

    for line in reader.lines() {
        let line = line?;
        data.push(line);
    }

    Ok(data)
}

fn process(data: &[String]) {    
    let mut sum:i64 = 0;

    for row in data {
        let mut parts_iter = row.split(':');

        let game_id = REGEX_GAME_ID.find(parts_iter.next().unwrap())
                        .unwrap().as_str().parse::<i64>().unwrap();

        sum += check_game(parts_iter.next().unwrap());        
    }
    println!("{}", sum);
}

// calculate power of max r*g*b
fn check_game(games: &str) -> i64 {
    let games_iter = games.split(';');
    let mut max_r:i64 = 0;
    let mut max_g:i64 = 0;
    let mut max_b:i64 = 0;

    for game in games_iter {
        let colors_iter = game.split(',');

        for color in colors_iter {
            let mut parts_iter = color.trim().split(' ');
            let num = parts_iter.next().unwrap().parse::<i64>().unwrap();
            let rgb = parts_iter.next().unwrap().chars().next().unwrap();

            if (rgb == 'r' && num > max_r) {
                max_r = num;
            } else if (rgb == 'g' && num > max_g) {
                max_g = num;
            } 
            if (rgb == 'b' && num > max_b) {
                max_b = num;
            }  
        }
    }

    return max_r * max_g * max_b;    
}