#![allow(unused_parens)]
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    let data: Result<Vec<String>, io::Error> = read_file("input/01puzzle.txt");

    if let Ok(lines) = data {
        process(&lines);
    }
}

fn read_file(file_name: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    let mut data: Vec<String> = vec![];

    for line in reader.lines() {
        let line = line?;
        data.push(line);
    }

    Ok(data)
}

fn process(data: &[String]) {
    let mut sum:i64 = 0;

    for row in data {
        let mut first: char = 'E';
        let mut second: char = 'E';

        for char in row.chars() {
            if char.is_ascii_digit() && first == 'E' {
                first = char;
            } else if char.is_ascii_digit() {
                second = char;
            }
        }

        if first != 'E' && second == 'E' {
            second = first;
        }

        //println!("{}, {}", first, second);

        sum += format!("{}{}", first, second).parse::<i64>().unwrap();
    }

    println!("{}", sum);

}