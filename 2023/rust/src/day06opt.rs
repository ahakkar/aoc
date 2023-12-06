use std::time::Instant;

fn main() {
    let start = Instant::now();
    process();
    let duration = start.elapsed();
    println!("Time elapsed in main() is: {:?}", duration);
}

fn process() {   
    println!("part1: {}", 
        calc_dist(62, 553) * calc_dist(64, 1010) * 
        calc_dist(91, 1473) * calc_dist(90, 1074)); // 840336
    println!("part2: {}", calc_dist(62649190, 553101014731074)); // 41382569
}

fn calc_dist(time: i64, limit: i64) -> i64 {
    let s = ((time*time-4*limit) as f64).sqrt();
    let mut x0 = ((time as f64) - s) / 2.0;
    let mut x1 = ((time as f64) + s) / 2.0;

    // if for example x0 = 10 and x10= 20 (integer results)
    if x0 == x0.ceil() { x0 += 1.0; } else { x0 = x0.ceil(); }
    if x1 == x1.floor() { x1 -= 1.0 } else { x1 = x1.floor(); }

    x0 = x0.ceil();
    x1 = x1.floor();

    (x1-x0 + 1.0) as i64
}
