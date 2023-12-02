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

        if check_game(parts_iter.next().unwrap()) {
            sum += game_id;
        }
    }

    println!("{}", sum);
}

// only 12 red cubes, 13 green cubes, and 14 blue cubes?
fn check_game(games: &str) -> bool {
    let games_iter = games.split(';');
    for game in games_iter {
        let colors_iter = game.split(',');
        for color in colors_iter {
            let mut parts_iter = color.trim().split(' ');
            let num = parts_iter.next().unwrap().parse::<i64>().unwrap();
            let rgb = parts_iter.next().unwrap().chars().next().unwrap();

            if (rgb == 'r' && num > 12) ||
                (rgb == 'g' && num > 13) ||
                (rgb == 'b' && num > 14)
            {                    
                return false;
            }
            //println!("{} {}", num, rgb);
        }
    }
    
    return true;
}