use crate::Day;
use std::fmt::Display;

pub struct Day1;

impl Day for Day1 {
    type Parsed<'a> = (Vec<u32>, Vec<u32>);

    fn generator(input: &str) -> Self::Parsed<'_> {
        let (mut l, mut r): (Vec<_>, Vec<_>) = input
            .lines()
            .map(|line| {
                let (l, r) = line.split_once(' ').unwrap();
                (
                    l.trim().parse::<u32>().unwrap(),
                    r.trim().parse::<u32>().unwrap(),
                )
            })
            .unzip();
        l.sort_unstable();
        r.sort_unstable();
        (l, r)
    }

    fn part1(input: Self::Parsed<'_>) -> impl Display {
        let (l, r) = input;
        let mut total_diff = 0;
        for (&l, &r) in l.iter().zip(r.iter()) {
            total_diff += l.abs_diff(r);
        }
        total_diff
    }

    fn part2(input: Self::Parsed<'_>) -> impl Display {
        let (l, r) = input;
        let mut remaining_r = &r[..];
        let mut similarity = 0;
        for l in l.iter().copied() {
            // Faster to just linear search rather than binary search
            let start = remaining_r
                .iter()
                .position(|&r| r >= l)
                .unwrap_or(remaining_r.len());
            remaining_r = &remaining_r[start..];

            let count = remaining_r
                .iter()
                .position(|&r| r != l)
                .unwrap_or(remaining_r.len());
            similarity += l * count as u32;

            remaining_r = &remaining_r[count..];
        }

        similarity
    }
}

crate::codspeed_def!(Day1);
