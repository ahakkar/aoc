use regex::Regex;
use std::fs;
use std::time::Instant;
use lazy_static::lazy_static;

lazy_static! {
    static ref RE_ID: regex::Regex = Regex::new(r"\d{1,3}").unwrap();
    static ref RE_SYMBOL: regex::Regex = Regex::new(r"[*]").unwrap();
}

fn main() {
    let start = Instant::now();

    let input = fs::read_to_string("input/03puzzle.txt").unwrap();
    let data: Vec<&str> = input.lines().collect();
    process(&data);
    
    let duration = start.elapsed();
    println!("Time elapsed in main() is: {:?}", duration);
}

// find gears and check if they have neighbouring numbers
fn process(data: &[&str]) {    
    let mut sum:i64 = 0;
    for (row, row_str) in data.iter().enumerate() {
        for gear in RE_SYMBOL.find_iter(row_str) {
            sum += check_neighbours(data, &row, &gear.start());            
        }
    }  
    println!("{}", sum); // 84900879
}

/**
 *   0  1  2  3  4  5  6  7
 *   8  9 10  * 12 13 14 15 
 *  16 17 18 19 20 21 22 23
 */
fn check_neighbours(data: &[&str], row: &usize, col: &usize) -> i64 {
    let start = col.saturating_sub(3);
    let end = start + 7;

    let mut neighbours:String = String::new();    
    let mut n1: i64 = 0;
    let mut n2: i64 = 0;

    // iterate through rows -1, 0, 1
    for i in 0..3 {
        neighbours += &data[row + i -1][start..end];
        neighbours.push('|');
    } 

    // check if any numbers are next to the gear in the flattened view
    for num in RE_ID.find_iter(&neighbours) {  
        if (num.start()..num.end())
            .any(|index| [2,3,4,10,12,18,19,20].contains(&index))
        {
            let num_value = num.as_str().parse::<i64>().unwrap_or(-1);
            match (n1, n2) {
                (0, _) => n1 = num_value,
                (_, 0) => n2 = num_value,
                _ => (),
            }
        }
    }

    n1*n2
}