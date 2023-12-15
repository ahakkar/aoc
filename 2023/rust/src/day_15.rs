/*
 * 2023 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/
use std::collections::BTreeMap;

struct Box {
    lenses: Vec<Lens>,
}

struct Lens {
    fl: usize,
    label: String,
    deleted: bool,
}
 
pub fn solve(data: Vec<String>) {
    //println!("Silver: {}", silver(&vec![String::from("HASH")]));
    println!("Silver: {}", silver(&data)); // 510801
    println!("Gold: {}", gold(&data)); // 212763
}

fn hash_str(str: &str) -> usize {
    str.chars().fold(0, |acc, c| (acc + (c as usize)) * 17 % 256)
}
 
fn silver(data: &[String]) -> usize {
    data[0].split(',').map(hash_str).sum()
}

fn gold(data: &[String]) -> usize {
    let mut shelf: BTreeMap<usize, Box> = BTreeMap::new();   

    for idx in 0..=255 {        
        shelf.insert(idx, Box{lenses: Vec::new()});
    }

    for hash in data[0].split(',') {
        let (label, op, fl) = tokenize(hash); 
        match op {
            '-' => subtract(&mut shelf, &label),
            '=' => add(&mut shelf, &label, &fl),
             _  => panic!("unsupported op"),
        }      
    } 
    summarize(shelf)
}

fn subtract(shelf: &mut BTreeMap<usize, Box>, label: &str) {
    if let Some(boxx) = shelf.get_mut(&hash_str(label)) {
        for lens in boxx.lenses.iter_mut() {
            if lens.label == label {       
                lens.deleted = true;
            }
        }
    }
}

fn add(shelf: &mut BTreeMap<usize, Box>, label: &str, fl: &usize) {
    if let Some(boxx) = shelf.get_mut(&hash_str(label)) {
        let mut lens_found = false;
        // check for existing lens
        for lens in boxx.lenses.iter_mut() {
            if lens.label == label && !lens.deleted {
                lens.fl = *fl;
                lens_found = true;
                break;                        
            }
        }
        // if not found, add a new one
        if !lens_found {
            boxx.lenses.push(Lens{fl: *fl, label: label.to_owned(), deleted: false});
        }
    }  
}

fn tokenize(hash: &str) -> (String, char, usize) {
    let mut label:String = String::new();
    let mut op:char = 'x';
    let mut fl:usize = usize::MAX;

    for c in hash.chars() { 
        if c != '=' && c != '-' {
            label.push(c);
        } else if c == '='{
            op = c;
            fl = hash.chars().last().unwrap().to_digit(10).unwrap() as usize;
            break;
        } else {
            op = c;
        }
    }
    (label, op, fl)
}

fn summarize(shelf: BTreeMap<usize, Box>) -> usize {
    let mut sum: usize = 0; 
    for boxx in shelf {
        let mut slot = 1;
        for lens in boxx.1.lenses {
            match lens.deleted {
                true => slot -= 1,
                false => sum += (boxx.0+1) * slot * lens.fl,
            }   
            slot += 1;
        }
    }
    sum
}

// run these with cargo test --bin main -- day_13::tests
#[cfg(test)]
mod tests {
    use crate::utils::read_data_from_file;
    use super::*;   

    #[test]
    fn test_test() {
        let test_data:Vec<String> = read_data_from_file("input/test/15.txt");
        assert_eq!(silver(&test_data), 1320);
        assert_eq!(gold(&test_data), 145);
    }

    #[test]
    fn test_silver() {
        let test_data:Vec<String> = read_data_from_file("input/real/15.txt");
        assert_eq!(silver(&test_data), 510801);
    }

    #[test]
    fn test_gold() {
        let test_data:Vec<String> = read_data_from_file("input/real/15.txt");
        assert_eq!(gold(&test_data), 212763);
    }
}