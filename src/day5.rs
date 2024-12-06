use crate::Day;
use arrayvec::ArrayVec;
use core::fmt;
use std::fmt::Display;

pub struct Day5;

type Int = u8;
type BitsetWord = u64;
// TODO: Try using Int::MAX to avoid bounds checks
const MAX_INT: Int = 99;
const BITSET_LEN_WORDS: usize =
    (MAX_INT as usize + BitsetWord::BITS as usize) / BitsetWord::BITS as usize;
const MAX_UPDATE_SIZE: usize = 64;

#[derive(Copy, Clone, PartialEq, Eq)]
struct Bitset([BitsetWord; BITSET_LEN_WORDS]);

impl Bitset {
    const fn new() -> Self {
        Self([0; BITSET_LEN_WORDS])
    }

    fn set(&mut self, n: Int) {
        let (idx, bit) = (
            n as usize / BitsetWord::BITS as usize,
            n as usize % BitsetWord::BITS as usize,
        );
        self.0[idx] |= 1 << bit;
    }

    fn get(&self, n: Int) -> bool {
        let (idx, bit) = (
            n as usize / BitsetWord::BITS as usize,
            n as usize % BitsetWord::BITS as usize,
        );
        self.0[idx] & (1 << bit) != 0
    }

    pub fn remove(&mut self, i: Int) {
        let (idx, bit) = (
            i as usize / BitsetWord::BITS as usize,
            i as usize % BitsetWord::BITS as usize,
        );
        self.0[idx] &= !(1 << bit);
    }

    fn first_set(&self) -> Option<Int> {
        for (i, &word) in self.0.iter().enumerate() {
            if word != 0 {
                let bit = word.trailing_zeros();
                return Some((i * BitsetWord::BITS as usize + bit as usize) as Int);
            }
        }
        None
    }

    fn intersects_any(&self, other: &Self) -> bool {
        for (lhs, &rhs) in self.0.iter().zip(other.0.iter()) {
            if lhs & rhs != 0 {
                return true;
            }
        }
        false
    }

    fn remove_all(&mut self, other: &Self) {
        for (lhs, &rhs) in self.0.iter_mut().zip(other.0.iter()) {
            *lhs &= !rhs;
        }
    }

    fn count_ones(&self) -> usize {
        self.0.iter().map(|&word| word.count_ones() as usize).sum()
    }
}

impl fmt::Debug for Bitset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entries((0..=MAX_INT).filter(|&i| self.get(i)))
            .finish()
    }
}

impl std::ops::BitAnd for Bitset {
    type Output = Self;

    fn bitand(mut self, rhs: Self) -> Self {
        for (lhs, &rhs) in self.0.iter_mut().zip(rhs.0.iter()) {
            *lhs &= rhs;
        }
        self
    }
}

impl std::ops::BitOrAssign for Bitset {
    fn bitor_assign(&mut self, rhs: Self) {
        for (lhs, &rhs) in self.0.iter_mut().zip(rhs.0.iter()) {
            *lhs |= rhs;
        }
    }
}

/// [i] is a bitset of all the numbers that must come after i
#[derive(Clone, Debug)]
struct RequiredAfter([Bitset; MAX_INT as usize + 1]);

impl RequiredAfter {
    const fn new() -> Self {
        Self([Bitset::new(); MAX_INT as usize + 1])
    }

    fn add_require(&mut self, i: Int, required: Int) {
        self.0[i as usize].set(required);
    }

    fn requirements_of(&self, i: Int) -> &Bitset {
        &self.0[i as usize]
    }
}

#[derive(Clone, Debug)]
pub struct Input {
    required_after: RequiredAfter,
    updates: Vec<ArrayVec<Int, MAX_UPDATE_SIZE>>,
}

impl Day for Day5 {
    type Parsed<'a> = Input;

    fn generator(input: &str) -> Self::Parsed<'_> {
        let input = input.as_bytes();

        let mut result = Input {
            required_after: RequiredAfter::new(),
            updates: Vec::with_capacity(256),
        };

        let mut end = 0;
        for (i, chunk) in input.chunks_exact(6).enumerate() {
            let [a, b, pipe, c, d, _] = chunk.try_into().unwrap();
            if a == b'\n' {
                end = i * 6;
                break;
            }
            debug_assert_eq!(pipe, b'|');

            let first = (a - b'0') * 10 + (b - b'0');
            let second = (c - b'0') * 10 + (d - b'0');
            result.required_after.add_require(first, second);
        }
        debug_assert_ne!(end, 0);
        debug_assert_eq!(input[end], b'\n');

        let mut current_val = 0;
        let mut current = ArrayVec::new();
        for b in input[end + 1..].iter().copied() {
            match b {
                b',' => {
                    current.push(current_val);
                    current_val = 0;
                }
                b'\n' => {
                    current.push(current_val);
                    current_val = 0;
                    result.updates.push(current);
                    current = ArrayVec::new();
                }
                digit => {
                    current_val = current_val * 10 + (digit - b'0') as Int;
                }
            }
        }

        result
    }

    fn part1(input: Self::Parsed<'_>) -> impl Display {
        let mut result = 0;
        for update in &input.updates {
            if !is_valid_update(&input.required_after, update) {
                continue;
            }
            result += u32::from(update[update.len() / 2]);
        }
        result
    }

    fn part2(input: Self::Parsed<'_>) -> impl Display {
        let mut result = 0;
        for update in &input.updates {
            let mut seen_bitset = Bitset::new();
            let mut valid = true;
            for &val in update {
                if seen_bitset.intersects_any(input.required_after.requirements_of(val)) {
                    valid = false;
                }
                seen_bitset.set(val);
            }
            if valid {
                continue;
            }
            let recursive_requires = create_recursive_requires(&input.required_after, seen_bitset);
            let sorted_midpoint = update
                .iter()
                .copied()
                .find(|&val| {
                    recursive_requires.requirements_of(val).count_ones() == update.len() / 2
                })
                .unwrap_or_else(|| panic!("update: {update:?}"));
            result += u32::from(sorted_midpoint);
        }
        result
    }
}

fn is_valid_update(required_after: &RequiredAfter, update: &[Int]) -> bool {
    let mut seen_bitset = Bitset::new();
    for &val in update {
        if seen_bitset.intersects_any(&required_after.0[val as usize]) {
            return false;
        }
        seen_bitset.set(val);
    }
    true
}

fn create_recursive_requires(required_after: &RequiredAfter, mask: Bitset) -> RequiredAfter {
    let mut result = RequiredAfter::new();
    let mut present_in_recursive = Bitset::new();

    let mut mask_iter = mask;
    while let Some(i) = mask_iter.first_set() {
        recursive_requires(
            required_after,
            &mut present_in_recursive,
            &mut result,
            mask,
            i,
        );
        mask_iter.remove(i);
    }

    result
}

fn recursive_requires(
    required_after: &RequiredAfter,
    present_in_recursive: &mut Bitset,
    recursive_req: &mut RequiredAfter,
    mask: Bitset,
    i: Int,
) -> Bitset {
    if present_in_recursive.get(i) {
        return *recursive_req.requirements_of(i);
    }
    let mut requirements = *required_after.requirements_of(i) & mask;
    let mut result = requirements;
    present_in_recursive.set(i);
    while let Some(j) = requirements.first_set() {
        let sub_requirements =
            recursive_requires(required_after, present_in_recursive, recursive_req, mask, j);
        result |= sub_requirements;
        requirements.remove_all(&sub_requirements);
        requirements.remove(j);
    }
    recursive_req.0[i as usize] = result;
    result
}

crate::codspeed_def!(Day5);

#[test]
fn check_part1_example() {
    let example = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

    assert_eq!(part1(example).to_string(), "143");
}
