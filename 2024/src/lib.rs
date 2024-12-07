mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;

mod utils;

use std::time::{Duration, Instant};


// Preprocessing step
pub trait Fro {
    fn fro(input: &str) -> Self;
}

pub struct AocResult {
    pub silver: (usize, Duration),
    pub gold:   (usize, Duration),
}

pub trait Solution {
    fn silver(&self) -> usize;
    fn gold(&self) -> usize;
}

pub fn run_solution(solution: Box<dyn Solution>) -> AocResult {
    let mut start = Instant::now();   
    let silver =  solution.silver();
    let silver_duration = start.elapsed();

    start = Instant::now();   
    let gold =  solution.gold();
    let gold_duration = start.elapsed();

    AocResult { silver: (silver, silver_duration), gold: (gold, gold_duration) }

    //     print!(" {:<31}║", solution.silver().to_string().bright_magenta());
    // print!(" {:<31}║", solution.gold().to_string().bright_magenta());
}

/// Calls respective's day's struct, input is contents from a file read as str
pub fn solve(day: &str, input: &str) -> AocResult {
    match day {
        "01" => run_solution(Box::new(day_01::HistorianHysteria::fro(input))),
        "02" => run_solution(Box::new(day_02::RedNosedReports::fro(input))),
        "03" => run_solution(Box::new(day_03::MullItOver::fro(input))),
        "04" => run_solution(Box::new(day_04::CeresSearch::fro(input))),
        "05" => run_solution(Box::new(day_05::PrintQueue::fro(input))),
        "06" => run_solution(Box::new(day_06::GuardGallivant::fro(input))),
        "07" => run_solution(Box::new(day_07::BridgeRepair::fro(input))),
        _ => unreachable!(),
    }
}