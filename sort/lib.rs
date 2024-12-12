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
mod day_11;
mod day_12;

mod utils;

use std::{
    fmt::{self, Display},
    time::{Duration, Instant},
};

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
    pub gold: (TaskResult, Duration),
    pub fro: Duration,
}

pub trait Solution {
    fn silver(&self) -> TaskResult;
    fn gold(&self) -> TaskResult;
}

pub fn run_solution<S: Fro + Solution + 'static>(input: &str) -> AocResult {
    let mut start = Instant::now();
    let solution = S::fro(input);
    let fro_duration = start.elapsed();

    let silver = solution.silver();
    let silver_duration = start.elapsed();

    start = Instant::now();
    let gold = solution.gold();
    let gold_duration = start.elapsed();

    AocResult {
        silver: (silver, silver_duration),
        gold: (gold, gold_duration),
        fro: fro_duration,
    }
}

/// Calls respective's day's struct, input is contents from a file read as str
pub fn solve(day: &str, input: &str) -> AocResult {
    match day {
        "01" => run_solution::<day_01::HistorianHysteria>(input),
        "02" => run_solution::<day_02::RedNosedReports>(input),
        "03" => run_solution::<day_03::MullItOver>(input),
        "04" => run_solution::<day_04::CeresSearch>(input),
        "05" => run_solution::<day_05::PrintQueue>(input),
        "06" => run_solution::<day_06::GuardGallivant>(input),
        "07" => run_solution::<day_07::BridgeRepair>(input),
        "08" => run_solution::<day_08::ResonantCollinearity>(input),
        "09" => run_solution::<day_09::DiskFragmenter>(input),
        "10" => run_solution::<day_10::HoofIt>(input),
        "11" => run_solution::<day_11::PlutonianPebbles>(input),
        "11" => run_solution::<day_12::GardenGroups>(input),
        _ => unreachable!(),
    }
}
