#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(clippy::needless_return)]
#![allow(clippy::needless_range_loop)]

use std::time::Instant;

fn main() {
    let start = Instant::now();
    process();
    let duration = start.elapsed();
    println!("Time elapsed in main() is: {:?}", duration);
}

fn process() {    
    let mut sum:i64 = 0;    

    calc_dist(7, 9);

    println!("part1 test: {}", calc_dist(7, 9) * calc_dist(15, 40) * calc_dist(30, 200)); 
    println!("part1: {}", 
        calc_dist(62, 553) * calc_dist(64, 1010) * 
        calc_dist(91, 1473) * calc_dist(90, 1074)); 
    println!("part2 test: {}", calc_dist(71530, 940200));
    println!("part2: {}", calc_dist(62649190, 553101014731074));
}

fn calc_dist(time: i64, limit: i64) -> usize {
    //println!("time {}, limit: {}", time, limit);
    let mut possible: Vec<i64> = vec![];

    let mut time_remaining = time;

    for speed in 0..time {
        if speed*time_remaining > limit {
            possible.push(speed*time_remaining);
        }
        // println!("{}", (speed*time_remaining));
        time_remaining -= 1;  
    }

    //println!("{:?}", possible);

    possible.len()
}