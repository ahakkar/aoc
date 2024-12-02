/*
 * 2024 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(clippy::needless_return)]
#![allow(clippy::needless_range_loop)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use super::utils::*;

fn char_score(char: &char) -> usize {
    match char {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
         _ => 0,
    }
}

fn chars_match(a: char, b: char) -> bool {
    matches!((a, b), ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>'))
}

pub fn silver(data: &[String]) -> usize {
    data
        .iter()
        .map(|s| { 
            let mut stack: Vec<char> = vec![]; 
            let mut invalid_char: Option<char> = None;

            for b in s.chars().collect::<Vec<char>>() {
                match b {
                    '(' | '[' | '{' | '<' => stack.push(b),
                    ')' | ']' | '}' | '>' => {
                        if let Some(a) = stack.pop() {                         
                            if !chars_match(a, b) {
                                invalid_char = Some(b);
                                break;
                            }
                        } else {           
                            invalid_char = Some(b);
                            break;
                        }
                    }
                    _ => panic!(),
                }
            } 
            if let Some(ch) = invalid_char { char_score(&ch) } else { 0 }
                     
        })
        .sum()
}

pub fn gold(data: &[String]) -> usize {
    let mut sum: usize = 0;    

    sum 
}

// run these with cargo test --bin main -- day_XX::tests
#[cfg(test)]
mod tests {
    use crate::utils::read_data_from_file;
    use super::*;   

    #[test]
    fn test_test() {
        let test_data:Vec<String> = read_data_from_file("input/test/10.txt");
        assert_eq!(silver(&test_data), 26397);
        assert_eq!(gold(&test_data), 288957);
    }

    #[test]
    fn test_silver() {
        let test_data:Vec<String> = read_data_from_file("input/real/10.txt");
        assert_eq!(silver(&test_data), 436497);
    }

    #[test]
    fn test_gold() {
        let test_data:Vec<String> = read_data_from_file("input/real/10.txt");
        assert_eq!(gold(&test_data), 212763);
    }
}