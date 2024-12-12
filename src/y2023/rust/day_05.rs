/*
 * 2023 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

#[derive(Debug)]
struct Range {
    sour_start: i64,
    range: i64,
    offset: i64,
}

pub fn solve(data: Vec<String>) {    
    println!("Silver: {}", silver(&data));
    println!("Gold: {}", gold(&data));
}

fn silver(data: &[String]) -> usize {
    let mut i: usize = 3;
    let mut range_vec: Vec<Range> = vec![];

    // seeds from 1st row and range data from rest
    let mut seeds:Vec<i64> = data[0]
        .split_once(": ").unwrap().1.split(' ')
        .filter_map(|n| n.parse::<i64>().ok())
        .collect::<Vec<_>>();   
    
    // collect ranges and transform seeds on empty row
    while i < data.len() {
        if data.get(i).unwrap() == "" {
            transform_seeds(&mut seeds, &range_vec);
            range_vec.clear();
            i += 2;
            continue;
        }

        let ti = data[i].split(' ')
            .filter_map(|n| n.parse::<i64>().ok())
            .collect::<Vec<_>>(); 

        range_vec.push(
            Range{         
                sour_start: ti[1],
                range:      ti[2],
                offset:     ti[0] - ti[1]
             }
        ); 
        i += 1;        
    }
    *seeds.iter().min().unwrap() as usize
}

fn gold(data: &Vec<String>) -> usize {
    let mut i: usize = 3;
    let mut seeds: Vec<i64> = vec![];
    let mut range_vec: Vec<Range> = vec![];

    // seeds from 1st row and range data from rest
    let vec:Vec<i64> = data[0]
        .split_once(": ").unwrap().1.split(' ')
        .filter_map(|n| n.parse::<i64>().ok())
        .collect::<Vec<_>>();  

    for i in (0..vec.len()).step_by(2) { // brute force, 120 seconds
        if i + 1 < vec.len() {
            for seed in vec[i]..=(vec[i] + vec[i + 1]) {
                seeds.push(seed);                
            }
        }
    }
    // collect ranges and transform seeds on empty row
    while i < data.len() {
        if data.get(i).unwrap() == "" {
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
                sour_start: ti[1],
                range:      ti[2],
                offset:     ti[0] - ti[1]
             }
        ); 
        i += 1;        
    }
    *seeds.iter().min().unwrap() as usize
}

fn transform_seeds(seeds: &mut [i64], range_vec: &[Range]) {
    for seed in seeds.iter_mut() {
        // should probably fix the range comparisons instead of using break;
        for range in range_vec {
            if *seed >= range.sour_start &&
               *seed <= (range.sour_start + range.range) {
               *seed += range.offset;
               break;
            }
        }
    }
}

// run these with cargo test --bin main -- day_XX::tests
#[cfg(test)]
mod tests {
    use crate::utils::read_data_from_file;
    use super::*;   

    #[test]
    fn test_silver() {
        let test_data:Vec<String> = read_data_from_file("input/real/05.txt");
        assert_eq!(silver(&test_data), 278755257);
    }

    #[test]
    fn test_gold() { // takes 120 seconds
        let test_data:Vec<String> = read_data_from_file("input/real/05.txt");
        assert_eq!(gold(&test_data), 26829166);
    }
}