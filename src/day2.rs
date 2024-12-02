use crate::Day;
use std::fmt::Display;

pub struct Day2;

type Int = u32;

fn is_safe_report(report: &[u32]) -> bool {
    let mut windows = report.windows(2);
    let Some(&[a, b]) = windows.next() else {
        return false;
    };
    if !(1..4).contains(&a.abs_diff(b)) {
        return false;
    }
    let increasing = b > a;
    for win in windows {
        let (a, b) = (win[0], win[1]);
        if (b > a) != increasing {
            return false;
        }
        if !(1..4).contains(&a.abs_diff(b)) {
            return false;
        }
    }
    true
}

impl Day for Day2 {
    type Parsed<'a> = Vec<Vec<Int>>;

    fn generator(input: &str) -> Self::Parsed<'_> {
        input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect()
            })
            .collect()
    }

    fn part1(input: Self::Parsed<'_>) -> impl Display {
        input
            .iter()
            .filter(|&report| is_safe_report(report))
            .count()
    }

    fn part2(input: Self::Parsed<'_>) -> impl Display {
        input
            .iter()
            .filter(|&report| {
                if is_safe_report(report) {
                    return true;
                }
                for i in 0..report.len() {
                    let mut report = report.clone();
                    report.remove(i);
                    if is_safe_report(&report) {
                        return true;
                    }
                }
                false
            })
            .count()
    }
}

crate::codspeed_def!(Day2);
