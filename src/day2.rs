use crate::Day;
use arrayvec::ArrayVec;
use std::fmt::Display;

pub struct Day2;

type Int = u8;
const MAX_VALUES_PER_REPORT: usize = 16;
type Report = ArrayVec<Int, MAX_VALUES_PER_REPORT>;

fn is_safe_report(report: &[Int]) -> bool {
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
    type Parsed<'a> = Vec<Report>;

    fn generator(input: &str) -> Self::Parsed<'_> {
        let mut dst = Vec::with_capacity(2000);
        let mut current = ArrayVec::new();
        let mut current_val = 0;
        for b in input.bytes() {
            match b {
                b' ' => {
                    current.push(current_val);
                    current_val = 0;
                }
                b'\n' => {
                    current.push(current_val);
                    dst.push(current);
                    current = ArrayVec::new();
                    current_val = 0;
                }
                digit => {
                    current_val = current_val * 10 + (digit - b'0') as Int;
                }
            }
        }
        dst
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
