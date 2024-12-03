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

fn char_score(char: Option<char>) -> usize {
    match char {
        Some(')') => 3,
        Some(']') => 57,
        Some('}') => 1197,
        Some('>') => 25137,
        Some(_) | None => 0,
    }
}

fn chars_match(a: char, b: char) -> bool {
    matches!((a, b), ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>'))
}

fn is_matching(
    stack: &mut Vec<char>,
    invalid_char: &mut Option<char>,
    b: char
) -> bool {

    match b {
        '(' | '[' | '{' | '<' => { stack.push(b); true },
        ')' | ']' | '}' | '>' => {
            if let Some(a) = stack.pop() {
                if !chars_match(a, b) {
                    *invalid_char = Some(b);  
                    return false;                  
                }
            } else {
                *invalid_char = Some(b);   
                return false;                 
            }
            true
        }
        _ => panic!("Unexpected character: {}", b),
    }
}

pub fn silver(data: &[String]) -> usize {
    data
        .iter()
        .map(|string| { 
            let mut stack: Vec<char> = vec![]; 
            let mut invalid_char: Option<char> = None;

            for b in string.chars() {               
                if!(is_matching(&mut stack, &mut invalid_char, b)) { break }   
            }            
            char_score(invalid_char)                     
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
        assert_eq!(gold(&test_data), 0); // 288957
    }

    #[test]
    fn test_silver() {
        let test_data:Vec<String> = read_data_from_file("input/real/10.txt");
        assert_eq!(silver(&test_data), 436497);
    }

    #[test]
    fn test_gold() {
        let test_data:Vec<String> = read_data_from_file("input/real/10.txt");
        assert_eq!(gold(&test_data), 0);
    }
}