pub mod util {
    pub mod direction;
    pub mod dsu;
    pub mod grid;
    pub mod gridmap;
    pub mod point2;
    pub mod point3;
    pub mod regex;
    pub mod utils;
}

pub mod y2015 {
    pub mod day_01;
    pub mod day_02;
    pub mod day_03;
    pub mod day_04;
    pub mod day_05;
    pub mod day_06;
    //pub mod day_07;
    //pub mod day_08;
    //pub mod day_09;
    //pub mod day_10;
    //pub mod day_11;
    //pub mod day_12;
    //pub mod day_13;
    //pub mod day_14;
    //pub mod day_15;
}

pub mod y2023 {
    pub mod day_01;
    pub mod day_02;
    pub mod day_03;
    pub mod day_04;
    pub mod day_05;
    pub mod day_06;
    pub mod day_07;
    pub mod day_08;
    pub mod day_09;
    // pub mod day_10;
    pub mod day_11;
    pub mod day_12;
    pub mod day_13;
    pub mod day_14;
    pub mod day_15;
    pub mod day_16;
    pub mod day_17;
    pub mod day_18;
    pub mod day_19;
    pub mod day_20;
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
    pub mod day_04;
    pub mod day_05;
    pub mod day_06;
    pub mod day_07;
    pub mod day_08;
    pub mod day_09;
    pub mod day_10;
    pub mod day_11;
    pub mod day_12;
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
        "2015" => match day {
            "01" => run_solution::<y2015::day_01::NotQuiteLisp>(input),
            "02" => run_solution::<y2015::day_02::IWasToldThereWouldBeNoMath>(input),
            "03" => {
                run_solution::<y2015::day_03::PerfectlySphericalHousesinaVacuum>(input)
            }
            "04" => run_solution::<y2015::day_04::TheIdealStockingStuffer>(input),
            "05" => run_solution::<y2015::day_05::DoesntHeHaveInternElvesForThis>(input),
            "06" => run_solution::<y2015::day_06::ProbablyaFireHazard>(input),
            _ => unreachable!(),
        },
        "2023" => match day {
            "01" => run_solution::<y2023::day_01::Trebuchet>(input),
            "02" => run_solution::<y2023::day_02::CubeConundrum>(input),
            "03" => run_solution::<y2023::day_03::GearRatios>(input),
            "04" => run_solution::<y2023::day_04::Scratchcards>(input),
            "05" => run_solution::<y2023::day_05::IfYouGiveASeedAFertilizer>(input),
            "06" => run_solution::<y2023::day_06::WaitForIt>(input),
            "07" => run_solution::<y2023::day_07::CamelCards>(input),
            "08" => run_solution::<y2023::day_08::HauntedWasteland>(input),
            "09" => run_solution::<y2023::day_09::MirageMaintenance>(input),
            // "10" => run_solution::<y2023::day_10::PipeMaze>(input),
            "11" => run_solution::<y2023::day_11::CosmicExpansion>(input),
            "12" => run_solution::<y2023::day_12::HotSprings>(input),
            "13" => run_solution::<y2023::day_13::PointofIncidence>(input),
            "14" => run_solution::<y2023::day_14::ParabolicReflectorDish>(input),
            "15" => run_solution::<y2023::day_15::LensLibrary>(input),
            "16" => run_solution::<y2023::day_16::TheFloorWillBeLava>(input),
            "17" => run_solution::<y2023::day_17::ClumsyCrucible>(input),
            "18" => run_solution::<y2023::day_18::LavaductLagoon>(input),
            "19" => run_solution::<y2023::day_19::Aplenty>(input),
            "20" => run_solution::<y2023::day_20::PulsePropagation>(input),
            _ => unreachable!(),
        },
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
            "04" => run_solution::<y2025::day_04::PrintingDepartment>(input),
            "05" => run_solution::<y2025::day_05::Cafeteria>(input),
            "06" => run_solution::<y2025::day_06::TrashCompactor>(input),
            "07" => run_solution::<y2025::day_07::Laboratories>(input),
            "08" => run_solution::<y2025::day_08::Playground>(input),
            "09" => run_solution::<y2025::day_09::MovieTheater>(input),
            "10" => run_solution::<y2025::day_10::Factory>(input),
            "11" => run_solution::<y2025::day_11::Reactor>(input),
            "12" => run_solution::<y2025::day_12::ChristmasTreeFarm>(input),
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}
