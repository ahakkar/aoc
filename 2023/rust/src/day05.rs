#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(clippy::needless_return)]
#![allow(clippy::needless_range_loop)]

use std::fs;
use std::fmt;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let input = fs::read_to_string("input/05simple.txt").unwrap();
    let data: Vec<&str> = input.lines().collect();

    process(&data);

    let duration = start.elapsed();
    println!("Time elapsed in main() is: {:?}", duration);
}

struct Range {
    dest_start: i64,
    sour_start: i64,
    range: i64,
    offset: i64,
}

impl fmt::Debug for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "dest_s: {}, sour_s: {}, range: {}, offset: {}"
                , self.dest_start, self.sour_start, self.range, self.offset)
    }
}

fn process(data: &[&str]) {  
    let mut i: usize = 3;
    let mut sum:i64 = 0;
    let mut range_vec: Vec<Range> = vec![];

    // seeds from 1st row and range data from rest
    let seeds = data[0]
        .split_once(": ").unwrap().1.split(' ')
        .filter_map(|n| n.parse::<i64>().ok())
        .collect::<Vec<_>>();   
    
    // collect ranges and transform seeds on empty row
    while i < data.len() {
        if data.get(i).unwrap() == &"" {
            transform_seeds(&seeds, &range_vec);
            range_vec.clear();
            i += 2;
            println!("---------------");
            continue;
        }

        let mut ti = data[i].split(' ')
            .filter_map(|n| n.parse::<i64>().ok())
            .collect::<Vec<_>>(); 

        range_vec.push(
            Range{
                dest_start: ti[0],
                sour_start: ti[1],
                range:      ti[2],
                offset:     ti[1] - ti[0]
             }
        ); 
        i += 1;
        
    }
    println!("answer: {}", sum);
}


fn transform_seeds(seeds: &[i64], range_vec: &[Range]) {
    for range in range_vec {
        println!("{:?}", range);
    }
}

