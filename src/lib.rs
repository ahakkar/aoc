pub mod util {
    pub mod grid;
    pub mod point;
    pub mod regex;
    pub mod utils;
}

pub mod y2024 {
    pub mod day_01;
    pub mod day_02;
    pub mod day_03;
    pub mod day_04;
    pub mod day_05;
    pub mod day_06;
    pub mod day_07;
    pub mod day_08;
    pub mod day_09;
    pub mod day_10;
    pub mod day_11;
    pub mod day_12;
    pub mod day_13;
    pub mod day_14;
    pub mod day_15;
}

pub mod y2025 {
    pub mod day_01;
    pub mod day_02;
    pub mod day_03;
}

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

impl From<usize> for TaskResult {
    fn from(v: usize) -> Self {
        TaskResult::Usize(v)
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
pub fn solve(year: &str, day: &str, input: &str) -> AocResult {
    match year {
        "2024" => match day {
            "01" => run_solution::<y2024::day_01::HistorianHysteria>(input),
            "02" => run_solution::<y2024::day_02::RedNosedReports>(input),
            "03" => run_solution::<y2024::day_03::MullItOver>(input),
            "04" => run_solution::<y2024::day_04::CeresSearch>(input),
            "05" => run_solution::<y2024::day_05::PrintQueue>(input),
            "06" => run_solution::<y2024::day_06::GuardGallivant>(input),
            "07" => run_solution::<y2024::day_07::BridgeRepair>(input),
            "08" => run_solution::<y2024::day_08::ResonantCollinearity>(input),
            "09" => run_solution::<y2024::day_09::DiskFragmenter>(input),
            "10" => run_solution::<y2024::day_10::HoofIt>(input),
            "11" => run_solution::<y2024::day_11::PlutonianPebbles>(input),
            "12" => run_solution::<y2024::day_12::GardenGroups>(input),
            "13" => run_solution::<y2024::day_13::ClawContraption>(input),
            "14" => run_solution::<y2024::day_14::RestroomRedoubt>(input),
            "15" => run_solution::<y2024::day_15::WarehouseWoes>(input),
            _ => unreachable!(),
        },
        "2025" => match day {
            "01" => run_solution::<y2025::day_01::SecretEntrance>(input),
            "02" => run_solution::<y2025::day_02::GiftShop>(input),
            "03" => run_solution::<y2025::day_03::Lobby>(input),
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}
