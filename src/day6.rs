#![allow(dead_code)]

use crate::Day;
use arrayvec::ArrayVec;
use std::fmt::Display;

pub struct Day6;

const MAX_DIM: usize = 150;
type BitsetWord = u64;
const BITSET_LEN_BITS: usize = MAX_DIM * MAX_DIM;
const BITSET_LEN_WORDS: usize = BITSET_LEN_BITS.div_ceil(BitsetWord::BITS as usize);

#[derive(Debug, Clone)]
struct Bitset(ArrayVec<BitsetWord, BITSET_LEN_WORDS>);

impl Bitset {
    fn set(&mut self, i: usize) {
        let (idx, bit) = (i / BitsetWord::BITS as usize, i % BitsetWord::BITS as usize);
        while self.0.len() <= idx {
            self.0.push(0);
        }
        self.0[idx] |= 1 << bit;
    }

    fn get(&self, i: usize) -> bool {
        let (idx, bit) = (i / BitsetWord::BITS as usize, i % BitsetWord::BITS as usize);

        if idx >= self.0.len() {
            return false;
        }
        self.0[idx] & (1 << bit) != 0
    }

    fn remove(&mut self, i: usize) {
        let (idx, bit) = (i / BitsetWord::BITS as usize, i % BitsetWord::BITS as usize);
        if idx < self.0.len() {
            self.0[idx] &= !(1 << bit);
        }
    }

    fn take_first_set(&mut self) -> Option<usize> {
        for (i, word) in self.0.iter_mut().enumerate() {
            if *word != 0 {
                let bit = word.trailing_zeros();
                *word &= !(1 << bit);
                return Some(i * BitsetWord::BITS as usize + bit as usize);
            }
        }
        None
    }

    fn count_ones(&self) -> usize {
        self.0.iter().map(|&word| word.count_ones() as usize).sum()
    }
}

#[derive(Debug, Clone)]
pub struct Grid {
    cells: Bitset,
    width: usize,
    start: (usize, usize),
    start_direction: Direction,
}

impl Grid {
    fn idx(&self, (x, y): (usize, usize)) -> usize {
        y * self.width + x
    }

    fn idx_to_point(&self, idx: usize) -> (usize, usize) {
        (idx % self.width, idx / self.width)
    }

    fn get(&self, point: (usize, usize)) -> Option<bool> {
        if point.0 >= self.width || point.1 >= self.width {
            None
        } else {
            Some(self.cells.get(self.idx(point)))
        }
    }

    fn set(&mut self, point: (usize, usize)) {
        self.cells.set(self.idx(point));
    }

    fn remove(&mut self, point: (usize, usize)) {
        self.cells.remove(self.idx(point));
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn rotated_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    fn next_point(self, (x, y): (usize, usize)) -> Option<(usize, usize)> {
        Some(match self {
            Self::Up => (x, y.checked_sub(1)?),
            Self::Down => (x, y.checked_add(1)?),
            Self::Left => (x.checked_sub(1)?, y),
            Self::Right => (x.checked_add(1)?, y),
        })
    }
}

impl Day for Day6 {
    type Parsed<'a> = Grid;

    fn generator(input: &str) -> Self::Parsed<'_> {
        let input = input.as_bytes();
        let mut cells = Bitset(ArrayVec::new());
        let mut start_idx = 0;
        let mut direction = Direction::Up;

        let width = input.iter().position(|&b| b == b'\n').unwrap();
        for (i, &b) in input.iter().enumerate() {
            let actual_i = i - i / (width + 1);
            match b {
                b'#' => cells.set(actual_i),
                b'^' => {
                    start_idx = actual_i;
                    direction = Direction::Up;
                }
                b'>' => {
                    start_idx = actual_i;
                    direction = Direction::Right;
                }
                b'v' => {
                    start_idx = actual_i;
                    direction = Direction::Down;
                }
                b'<' => {
                    start_idx = actual_i;
                    direction = Direction::Left;
                }
                _ => {}
            }
        }

        Grid {
            cells,
            width,
            start: (start_idx % width, start_idx / width),
            start_direction: direction,
        }
    }

    fn part1(input: Self::Parsed<'_>) -> impl Display {
        let visited_locations = find_visited_locations(&input);

        visited_locations.count_ones()
    }

    fn part2(_input: Self::Parsed<'_>) -> impl Display {
        /*
        let mut visited_locations = find_visited_locations(&input);

        let mut loops = 0;
        while let Some(idx) = visited_locations.take_first_set() {
            let point = input.idx_to_point(idx);
            input.set(point);
            loops += u32::from(run_simulation(&input) == SimResult::Loop);
            input.remove(point);
        }

         */
        "TODO"
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum SimResult {
    Loop,
    Exit,
}

fn run_simulation(input: &Grid) -> SimResult {
    let mut current_point = input.start;
    let mut direction = input.start_direction;
    // Bit set for each direction seen at that location
    let mut pos_directions = [0u8; MAX_DIM * MAX_DIM];

    loop {
        let idx = input.idx(current_point);
        let mask = 1 << direction as u8;
        if pos_directions[idx] & mask != 0 {
            return SimResult::Loop;
        }
        pos_directions[idx] |= mask;
        let Some(next_point) = direction.next_point(current_point) else {
            return SimResult::Exit;
        };
        match input.get(next_point) {
            Some(false) => {
                current_point = next_point;
            }
            Some(true) => {
                direction = direction.rotated_right();
                let Some(next_point) = direction.next_point(current_point) else {
                    return SimResult::Exit;
                };
                current_point = next_point;
            }
            None => {
                return SimResult::Exit;
            }
        }
    }
}

fn find_visited_locations(input: &Grid) -> Bitset {
    let mut visited_locations = Bitset(ArrayVec::new());

    let mut current_point = input.start;
    let mut direction = input.start_direction;

    loop {
        visited_locations.set(input.idx(current_point));
        let Some(next_point) = direction.next_point(current_point) else {
            break;
        };
        match input.get(next_point) {
            Some(false) => {
                current_point = next_point;
            }
            Some(true) => {
                direction = direction.rotated_right();
                let Some(next_point) = direction.next_point(current_point) else {
                    break;
                };
                current_point = next_point;
            }
            None => {
                break;
            }
        }
    }
    visited_locations
}

crate::codspeed_def!(Day6);

#[test]
fn example_part_1() {
    let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";
    let parsed = Day6::generator(input);
    assert_eq!(Day6::part1(parsed).to_string(), "41");
}
