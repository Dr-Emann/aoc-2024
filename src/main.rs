use std::fmt;

mod day1;
mod day2;

trait Day {
    type Parsed<'a>;

    fn generator(input: &str) -> Self::Parsed<'_>;

    fn part1(input: &Self::Parsed<'_>) -> impl std::fmt::Display;
    fn part2(input: Self::Parsed<'_>) -> impl std::fmt::Display;
}

#[derive(Debug, Clone)]
struct Timing {
    gen: std::time::Duration,
    part1: std::time::Duration,
    part2: std::time::Duration,
}

#[derive(Debug, Clone)]
struct DayResults {
    timing: Timing,
    part1: String,
    part2: String,
}

impl fmt::Display for DayResults {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Gen: ({:?})\nPart 1: {} ({:?})\nPart 2: {} ({:?})",
            self.timing.gen,
            self.part1,
            self.timing.part1,
            self.part2,
            self.timing.part2
        )
    }
}

#[inline]
fn time<O>(f: impl FnOnce() -> O) -> (O, std::time::Duration) {
    let start = std::time::Instant::now();
    let res = f();
    let elapsed = start.elapsed();
    (res, elapsed)
}

fn run_day<D: Day>(input: &str) -> DayResults {
    let (parsed, gen_time) = time(|| D::generator(input));

    let (part1, part1_time) = time(|| D::part1(&parsed));
    let part1_str = part1.to_string();
    drop(part1);

    let (part2, part2_time) = time(|| D::part2(parsed));
    let part2_str = part2.to_string();
    drop(part2);

    DayResults {
        timing: Timing {
            gen: gen_time,
            part1: part1_time,
            part2: part2_time,
        },
        part1: part1_str,
        part2: part2_str,
    }
}

const DAYS: &[fn(&str) -> DayResults] = &[
    run_day::<day1::Day1>,
    run_day::<day2::Day2>,
];

fn fully_run_day(day_num: usize) -> DayResults {
    let path = format!("input/2024/day{}.txt", day_num);
    let input = std::fs::read_to_string(&path).expect("Failed to read path");
    let results = DAYS[day_num - 1](&input);
    println!("Day {}: {results}", day_num);
    results
}

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
