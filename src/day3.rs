use crate::Day;
use memchr::memmem;
use std::fmt::Display;

pub struct Day3;

type Int = u32;

impl Day for Day3 {
    type Parsed<'a> = &'a str;

    fn generator(input: &str) -> Self::Parsed<'_> {
        input
    }

    fn part1(input: Self::Parsed<'_>) -> impl Display {
        let input = input.as_bytes();
        let mult_finder = memmem::Finder::new("mul(");
        let mut result = 0;
        for mul_idx in mult_finder.find_iter(input) {
            if let Some((x, y)) =
                parse_after_mul(&mut &input[mul_idx + mult_finder.needle().len()..])
            {
                result += x * y;
            }
        }
        result
    }

    fn part2(input: Self::Parsed<'_>) -> impl Display {
        let mut input = input.as_bytes();
        let do_finder = memmem::Finder::new("do()");
        let dont_finder = memmem::Finder::new("don't()");
        let mult_finder = memmem::Finder::new("mul(");

        let mut result = 0;
        loop {
            let next_dont = dont_finder.find(input).unwrap_or(input.len());
            let (mut inner_input, rest) = (
                &input[..next_dont],
                input
                    .get(next_dont + dont_finder.needle().len()..)
                    .unwrap_or(&[]),
            );
            while let Some(next_mul) = mult_finder.find(inner_input) {
                inner_input = &inner_input[next_mul + mult_finder.needle().len()..];
                if let Some((x, y)) = parse_after_mul(&mut inner_input) {
                    result += x * y;
                }
            }
            input = rest;
            let Some(next_do) = do_finder.find(input) else {
                break;
            };
            input = &input[next_do + do_finder.needle().len()..];
        }

        result
    }
}

fn parse_after_mul(input: &mut &[u8]) -> Option<(Int, Int)> {
    let mut it = input.iter();
    let mut x = 0;
    for &b in &mut it {
        match b {
            b',' => break,
            b'0'..=b'9' => x = x * 10 + Int::from(b - b'0'),
            _ => {
                *input = it.as_slice();
                return None;
            }
        }
    }
    let mut y = 0;
    // Found a comma
    for &b in &mut it {
        match b {
            b')' => break,
            b'0'..=b'9' => y = y * 10 + Int::from(b - b'0'),
            _ => {
                *input = it.as_slice();
                return None;
            }
        }
    }

    // found a closing parenthesis
    *input = it.as_slice();
    Some((x, y))
}

crate::codspeed_def!(Day3);
