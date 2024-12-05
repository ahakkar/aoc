/*
 * 2024 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/
use regex::Regex;

pub fn solve(data: Vec<String>) {
    println!("Silver: {}", silver(&data));
    println!("Gold: {}", gold(&data));
}

fn parse_tuple(s: &str) -> usize {
    s
        .get(4..s.len() - 1)
        .unwrap()
        .split_once(',')
        .map(|(a, b)| 
            a.trim().parse::<usize>().unwrap() * 
            b.trim().parse::<usize>().unwrap())  
        .unwrap()
}

pub fn silver(data: &[String]) -> usize {
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();

    data.iter()
        .flat_map(|row| re.find_iter(row))
        .map(|m| parse_tuple(m.as_str()))
        .sum()
}

pub fn gold(data: &[String]) -> usize {
    let re = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don\'t\(\)").unwrap();
    let mut mode = true;

    data.iter()
        .flat_map(|row| re.find_iter(row))
        .filter_map(|s| {
            match s.as_str() {
                "don't()" => { mode = false; None },
                "do()"    => { mode = true; None },
                _ => if mode { Some(parse_tuple(s.into())) } else { None },
            }
        })
        .sum()
}

// cargo test --bin main -- day_XX::tests
#[cfg(test)]
mod tests {
    use crate::utils::read_data_from_file;
    use super::*;   

    #[test]
    fn test_test() {
        let silver_data:Vec<String> = read_data_from_file("input/test/03s.txt");
        let gold_data:Vec<String> = read_data_from_file("input/test/03g.txt");
        assert_eq!(silver(&silver_data), 161);
        assert_eq!(gold(&gold_data), 48);
    }

    #[test]
    fn test_silver() {
        let test_data:Vec<String> = read_data_from_file("input/real/03.txt");
        assert_eq!(silver(&test_data), 175615763);
    }

    #[test]
    fn test_gold() {
        let test_data:Vec<String> = read_data_from_file("input/real/03.txt");
        assert_eq!(gold(&test_data), 74361272);
    }
}