use crate::Day;
use arrayvec::ArrayVec;
use std::fmt::Display;

pub struct Day2;

type Int = u8;
const MAX_VALUES_PER_REPORT: usize = 16;
type Report = ArrayVec<Int, MAX_VALUES_PER_REPORT>;

fn is_safe_report(report: &[Int]) -> bool {
    let (&[x, y], rest) = report.split_first_chunk().unwrap();
    let increasing = y > x;
    if !(1..4).contains(&x.abs_diff(y)) {
        return false;
    }
    first_unsafety(y, increasing, rest).is_none()
}

fn first_unsafety(prev_value: Int, increasing: bool, remaining_report: &[Int]) -> Option<usize> {
    let range = 1..4;
    let mut last = prev_value;
    for (i, &v) in remaining_report.iter().enumerate() {
        let diff = if increasing {
            v.wrapping_sub(last)
        } else {
            last.wrapping_sub(v)
        };
        if !range.contains(&diff) {
            return Some(i);
        }
        last = v;
    }
    None
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
        if input.as_bytes().last() != Some(&b'\n') {
            current.push(current_val);
            dst.push(current);
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
                let (&[x, y], rest) = report.split_first_chunk().unwrap();
                if !(1..4).contains(&x.abs_diff(y)) {
                    let (&z, rest) = rest.split_first().unwrap();
                    // try omitting x:
                    {
                        let increasing = z > y;
                        if (1..4).contains(&y.abs_diff(z))
                            && first_unsafety(z, increasing, rest).is_none()
                        {
                            return true;
                        }
                    }
                    // try omitting y
                    {
                        let increasing = z > x;
                        if (1..4).contains(&x.abs_diff(z))
                            && first_unsafety(z, increasing, rest).is_none()
                        {
                            return true;
                        }
                    }
                    return false;
                }
                let increasing = y > x;
                let Some(i) = first_unsafety(y, increasing, rest) else {
                    return true;
                };
                let i = i + 2;
                // Try omitting the previous value
                {
                    let last_value = report[i - 2];
                    let increasing = if i == 2 {
                        report[i] > last_value
                    } else {
                        increasing
                    };
                    if first_unsafety(last_value, increasing, &report[i..]).is_none() {
                        return true;
                    }
                }
                // Try omitting the value
                {
                    let last_value = report[i - 1];
                    if first_unsafety(last_value, increasing, &report[i + 1..]).is_none() {
                        return true;
                    }
                }
                false
            })
            .count()
    }
}

crate::codspeed_def!(Day2);
