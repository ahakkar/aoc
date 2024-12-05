mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;

mod utils;

// Preprocessing step
pub trait Fro {
    fn fro(input: &str) -> Self;
}

pub trait Solution {
    fn silver(&self) -> usize;
    fn gold(&self) -> usize;
}

pub fn run_solution(solution: Box<dyn Solution>) {
    println!("Silver: {}", solution.silver());
    println!("Gold: {}", solution.gold());
}

/// Calls respective's day's struct, input is contents from a file read as str
pub fn solve(day: &str, input: &str) {
    match day {
        "01" => run_solution(Box::new(day_01::HistorianHysteria::fro(input))),
        "02" => run_solution(Box::new(day_02::RedNosedReports::fro(input))),
        "03" => run_solution(Box::new(day_03::MullItOver::fro(input))),
        "04" => run_solution(Box::new(day_04::CeresSearch::fro(input))),
        "05" => run_solution(Box::new(day_05::PrintQueue::fro(input))),
        _ => unreachable!(),
    }
}