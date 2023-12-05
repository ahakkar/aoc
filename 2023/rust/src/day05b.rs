use std::fs;
use std::fmt;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let input = fs::read_to_string("input/05puzzle.txt").unwrap();
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
        write!(f, "|dest_s: {}, sour_s: {}, range: {}, offset: {}|"
                , self.dest_start, self.sour_start, self.range, self.offset)
    }
}

fn process(data: &[&str]) {  
    let mut i: usize = 3;
    let mut seeds: Vec<i64> = vec![];
    let mut range_vec: Vec<Range> = vec![];

    // seeds from 1st row and range data from rest
    let vec:Vec<i64> = data[0]
        .split_once(": ").unwrap().1.split(' ')
        .filter_map(|n| n.parse::<i64>().ok())
        .collect::<Vec<_>>();  

    for i in (0..vec.len()).step_by(2) { // brute force
        if i + 1 < vec.len() {
            for seed in vec[i]..=(vec[i] + vec[i + 1]) {
                seeds.push(seed);                
            }
        }
    }
    // collect ranges and transform seeds on empty row
    while i < data.len() {
        if data.get(i).unwrap() == &"" {
            transform_seeds(&mut seeds, &range_vec);
            range_vec.clear();
            i += 2;
            //println!("---------------");
            continue;
        }

        let ti = data[i].split(' ')
            .filter_map(|n| n.parse::<i64>().ok())
            .collect::<Vec<_>>(); 

        range_vec.push(
            Range{
                dest_start: ti[0],
                sour_start: ti[1],
                range:      ti[2],
                offset:     ti[0] - ti[1]
             }
        ); 
        i += 1;        
    }
    println!("answer: {}", seeds.iter().min().unwrap()); // 278755257
}

fn transform_seeds(seeds: &mut [i64], range_vec: &[Range]) {
    for seed in seeds.iter_mut() {
        for range in range_vec {
            if *seed >= range.sour_start &&
               *seed < (range.sour_start + range.range) {
               *seed += range.offset;
               break;
            } 
        }
    }
}

