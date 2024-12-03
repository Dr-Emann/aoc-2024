use crate::Day;
use std::fmt::Display;
use regex::Regex;

pub struct Day3;

impl Day for Day3 {
    type Parsed<'a> = &'a str;

    fn generator(input: &str) -> Self::Parsed<'_> {
        input
    }

    fn part1(input: Self::Parsed<'_>) -> impl Display {
        let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        let mut result = 0;
        for cap in regex.captures_iter(input) {
            let a: i32 = cap[1].parse().unwrap();
            let b: i32 = cap[2].parse().unwrap();
            result += a * b;
        }
        result
    }

    fn part2(input: Self::Parsed<'_>) -> impl Display {
        "TODO"
    }
}

crate::codspeed_def!(Day3);
