use std::fmt;

pub mod day1;
pub mod day2;
pub mod day3;

pub trait Day {
    type Parsed<'a>: Clone;

    fn generator(input: &str) -> Self::Parsed<'_>;

    fn part1(input: Self::Parsed<'_>) -> impl fmt::Display;
    fn part2(input: Self::Parsed<'_>) -> impl fmt::Display;
}

#[derive(Debug, Clone)]
pub struct Timing {
    pub gen: std::time::Duration,
    pub part1: std::time::Duration,
    pub part2: std::time::Duration,
}

#[derive(Debug, Clone)]
pub struct DayResults {
    pub timing: Timing,
    pub part1: String,
    pub part2: String,
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

pub fn run_day<D: Day>(input: &str) -> DayResults {
    let (parsed, gen_time) = time(|| D::generator(input));

    let (part1, part1_time) = time(|| D::part1(parsed.clone()));
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

pub const DAYS: &[fn(&str) -> DayResults] = &[
    run_day::<day1::Day1>,
    run_day::<day2::Day2>,
    run_day::<day3::Day3>,
];

pub fn fully_run_day(day_num: usize) -> DayResults {
    let path = format!("input/2024/day{}.txt", day_num);
    let input = std::fs::read_to_string(&path).expect("Failed to read path");
    let results = DAYS[day_num - 1](&input);
    println!("Day {}: {results}", day_num);
    results
}

fn part_1_impl<'a, D: Day + 'a>(input: &'a str) -> impl fmt::Display + 'a {
    let parsed = D::generator(input);
    D::part1(parsed)
}

fn part_2_impl<'a, D: Day + 'a>(input: &'a str) -> impl fmt::Display + 'a {
    let parsed = D::generator(input);
    D::part2(parsed)
}

macro_rules! codspeed_def {
    ($day_ty:ty) => {
        pub fn part1(input: &str) -> impl ::std::fmt::Display + '_ {
            $crate::part_1_impl::<$day_ty>(input)
        }

        pub fn part2(input: &str) -> impl ::std::fmt::Display + '_ {
            $crate::part_2_impl::<$day_ty>(input)
        }
    };
}

use codspeed_def;