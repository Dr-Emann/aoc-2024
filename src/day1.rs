use crate::Day;
use std::fmt::Display;

pub struct Day1;

impl Day for Day1 {
    type Parsed<'a> = (Vec<u32>, Vec<u32>);

    fn generator(input: &str) -> Self::Parsed<'_> {
        let lines = input.bytes().filter(|&c| c == b'\n').count();
        let mut l = Vec::with_capacity(lines);
        let mut r = Vec::with_capacity(lines);
        let mut current_val = 0;
        let mut has_val = true;
        for b in input.bytes() {
            match b {
                b' ' => {
                    if has_val {
                        l.push(current_val);
                        has_val = false;
                        current_val = 0;
                    }
                }
                b'\n' => {
                    r.push(current_val);
                    current_val = 0;
                    has_val = true;
                }
                digit => {
                    debug_assert!(digit.is_ascii_digit());
                    current_val = current_val * 10 + (digit - b'0') as u32;
                }
            }
        }
        debug_assert_eq!(l.len(), r.len());
        debug_assert_eq!(l.len(), lines);
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
            // Faster to just linear search rather than binary search for AOC input sizes
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
