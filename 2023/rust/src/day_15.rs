/*
 * 2023 Advent of Code with Rust
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

use std::collections::VecDeque;

 
pub fn solve(data: Vec<String>) {
    //println!("Silver: {}", silver(&vec![String::from("HASH")]));
    println!("Silver: {}", silver(&data)); // 510801
    println!("Gold: {}", gold(&data));
 }
 
fn silver(data: &[String]) -> usize {
    let mut sum: usize = 0;
    let mut cv: usize = 0;
    let hashes = data[0].split(',');

    for hash in hashes {
        cv = 0;
        for c in hash.chars() {            
            cv += c as usize;
            cv *= 17;
            cv %= 256;
        }
        sum += cv;        
    }
    sum 
}

fn gold(data: &Vec<String>) -> usize {
    let mut sum: usize = 0; 
    let mut deq = VecDeque::from([-1, 0, 1]);       
    let hashes = data[0].split(',');

    for hash in hashes {
        let mut h:String = String::new();
        let mut op:char = 'x';
        let mut num:u8 = u8::MAX;

        for c in hash.chars() {
            if c != '=' && c != '-' {
                h.push(c);
            } else if c == '='{
                op = c;
                num = hash.chars().last().unwrap().to_digit(10).unwrap() as u8;
                break;
            } else {
                op = c;
                break;
            }
        }
        println!("result: h: {}, op: {}, num: {}", h, op, num);
    } 
    sum 
}