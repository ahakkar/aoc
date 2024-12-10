mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;

mod utils;

use std::{fmt::{self, Display}, time::{Duration, Instant}};


// Preprocessing step
pub trait Fro {
    fn fro(input: &str) -> Self;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskResult {
    Usize(usize),
    String(String),
}

impl Display for TaskResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskResult::Usize(value) => write!(f, "{}", value),
            TaskResult::String(value) => write!(f, "{}", value.clone()), 
        }
    }
}

pub struct AocResult {
    pub silver: (TaskResult, Duration),
    pub gold:   (TaskResult, Duration),
    pub fro: Duration, // todo log preprocessing step also
}

pub trait Solution {
    fn silver(&self) -> TaskResult;
    fn gold(&self) -> TaskResult;
}

pub fn run_solution(solution: Box<dyn Solution>) -> AocResult {
    let mut start = Instant::now();   
    let silver =  solution.silver();
    let silver_duration = start.elapsed();

    start = Instant::now();   
    let gold =  solution.gold();
    let gold_duration = start.elapsed();

    AocResult { 
        silver: (silver, silver_duration),
        gold: (gold, gold_duration), 
        fro: Duration::new(0,0)
    }

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
        "08" => run_solution(Box::new(day_08::ResonantCollinearity::fro(input))),
        "09" => run_solution(Box::new(day_09::DiskFragmenter::fro(input))),
        "10" => run_solution(Box::new(day_10::HoofIt::fro(input))),
        _ => unreachable!(),
    }
}