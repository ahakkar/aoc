/*
 * 2023 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
 */

 pub fn solve(data: Vec<String>) {
    println!("Silver: {}", silver(&data));
    println!("Gold: {}", gold(&data));
}

fn silver(data: &Vec<String>) -> i64 {
    let mut sum: i64 = 0;    

    for row in data {
        let mut cum: i64 = 0;
        let mut series: Vec<i64> = vec!();        
        let mut nums:Vec<i64> = row
            .split_ascii_whitespace()
            .map(|n| n.parse::<i64>().unwrap())
            .collect();

        series.push(*nums.last().unwrap());

        while nums.iter().sum::<i64>() != 0 {
            let mut new_nums: Vec<i64> = vec!();
            for i in 0..nums.len() - 1 {
                new_nums.push(nums[i+1] - nums[i]);
            }            
            series.push(*new_nums.last().unwrap());                        
            nums = new_nums;      
        }     

        for i in (0..series.len()).rev() { 
            cum += series[i];
        }
        sum += cum;
    }
    sum 
    // 1887980197
    // 1889613394 too high
}

fn gold(data: &Vec<String>) -> i64 {
    let mut sum: i64 = 0;    

    for row in data {
        let mut prev: i64 = 0;
        let mut series: Vec<Vec<i64>> = vec!();        
        let mut nums:Vec<i64> = row
            .split_ascii_whitespace()
            .map(|n| n.parse::<i64>().unwrap())
            .collect();

        series.push((*nums).to_vec());
        while nums.iter().sum::<i64>() != 0 {
            let mut new_nums: Vec<i64> = vec!();
            for i in 0..nums.len() - 1 {
                new_nums.push(nums[i+1] - nums[i]);
            }            
            series.push((*new_nums).to_vec());                        
            nums = new_nums;   
        }   
        
        //println!("series: {:?}\n", series);
         
        //print!("cum: ");
        for i in (0..series.len()).rev() {
            let b = series[i].first().unwrap();            
            //println!("{} - {} = {}", b, prev, b - prev);
            prev = b - prev;
        } 
        sum += prev;    
    } 
    sum 
    // 1887980197
    // 1889613394 too high
}