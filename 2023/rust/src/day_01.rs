/*
 * 2023 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

use std::collections::HashMap;
use regex::Regex;

pub fn solve(data: Vec<String>) {    
    println!("Silver: {}", silver(&data));
    println!("Gold: {}", gold(&data));
}

fn silver(data: &[String]) -> usize {
    let mut sum:usize = 0;

    for row in data {
        let mut first: char = 'E';
        let mut second: char = 'E';

        for char in row.chars() {
            if char.is_ascii_digit() && first == 'E' {
                first = char;
            } else if char.is_ascii_digit() {
                second = char;
            }
        }

        if first != 'E' && second == 'E' {
            second = first;
        }  

        sum += format!("{}{}", first, second).parse::<usize>().unwrap();
    }
    sum 
}

fn gold(data: &[String]) -> usize {
    let mut sum:usize = 0;

    let terms:HashMap<&str, char> = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9')
        ]);
    let regex_first = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let regex_second = Regex::new(r"(\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)").unwrap();
    
    for row in data.iter() {
        //print!("{}: {:?}", i, row);

        // NOT SAFE BUT WHATEVER
        let first_str = regex_first.find(row).unwrap().as_str();
        let reversed_row:String = row.chars().rev().collect();
        let second_str = regex_second.find(&reversed_row).unwrap().as_str();

        let mut first: char = first_str.chars().next().unwrap();
        let mut second: char = second_str.chars().next().unwrap();

        // replace with actual char if needed
        if first_str.len() > 1 {
            first = terms[first_str];
        } 
        if second_str.len() > 1 {
            let original_str:String = second_str.chars().rev().collect();
            second = terms[&original_str.as_str()];
        } 

        //print!(" {}, {}", first, second);

        let coord = format!("{}{}", first, second).parse::<usize>().unwrap();
        //println!(" = {}", coord);
        sum += coord;

    }
    sum
}

// run these with cargo test --bin main -- day_XX::tests
#[cfg(test)]
mod tests {
    use crate::utils::read_data_from_file;
    use super::*;   

    #[test]
    fn test_silver() {
        let test_data:Vec<String> = read_data_from_file("input/real/01.txt");
        assert_eq!(silver(&test_data), 55017);
    }

    #[test]
    fn test_gold() {
        let test_data:Vec<String> = read_data_from_file("input/real/01.txt");
        assert_eq!(gold(&test_data), 53539);
    }
}