use aoc_2024::{fully_run_day, DAYS};
fn main() {
    let mut total_time = std::time::Duration::ZERO;
    if let Some(day) = std::env::args().nth(1).and_then(|s| s.parse::<usize>().ok()) {
        let results = fully_run_day(day);
        println!();
        total_time += results.timing.gen + results.timing.part1 + results.timing.part2;
    } else {
        for i in 0..DAYS.len() {
            let results = fully_run_day(i + 1);
            println!();
            total_time += results.timing.gen + results.timing.part1 + results.timing.part2;
        }
    }

    println!("Total time: {:?}", total_time);
}
