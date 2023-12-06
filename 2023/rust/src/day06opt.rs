use std::time::Instant;

fn main() {
    let start = Instant::now();
    process();
    let duration = start.elapsed();
    println!("Time elapsed in main() is: {:?}", duration);
}

fn process() {       
    println!("part1 test: {}", calc_dist(30, 200)); // 9 but results in 10 :D
    println!("part1: {}", 
        calc_dist(62, 553) * calc_dist(64, 1010) * 
        calc_dist(91, 1473) * calc_dist(90, 1074)); // 840336
    println!("part2: {}", calc_dist(62649190, 553101014731074)); // 41382569
}

fn calc_dist(time: i64, limit: i64) -> i64 {
    let s = ((time*time-4*limit) as f64).sqrt();
    let mut x0 = (((time as f64) - s) / 2.0) as i64;
    let mut x1 = (((time as f64) + s) / 2.0) as i64;

    if x0 == 0 { x0 = 1; } else { x0 += 1; }
    if x1 == 0 { x1 -= 1}

    x1-x0 + 1    
}
