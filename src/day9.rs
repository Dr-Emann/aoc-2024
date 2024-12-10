use crate::Day;
use arrayvec::ArrayVec;
use std::fmt::Display;

type Int = u64;
type Offset = u32;

pub struct Day9;

impl Day for Day9 {
    type Parsed<'a> = &'a [u8];

    fn generator(input: &str) -> Self::Parsed<'_> {
        &input.as_bytes()[..input.len() - 1]
    }

    fn part1(input: Self::Parsed<'_>) -> impl Display {
        let mut it = input.iter().copied().map(|b| b - b'0');
        let mut hash = 0;

        let mut current_value = 0;
        let mut current_index = 0;
        let mut end_value = (input.len() / 2) as Int;
        let mut end_len = it.next_back().unwrap();

        'outer: loop {
            let Some(start_len) = it.next() else { break };
            hash += hash_range(current_value, current_index, start_len);
            current_index += Offset::from(start_len);
            current_value += 1;
            let Some(mut start_gap) = it.next() else {
                break;
            };
            while end_len < start_gap {
                hash += hash_range(end_value, current_index, end_len);
                current_index += Offset::from(end_len);
                start_gap -= end_len;
                end_value -= 1;
                _ = it.next_back();
                let Some(new_end_len) = it.next_back() else {
                    end_len = start_len;
                    end_value = current_value;
                    break 'outer;
                };
                end_len = new_end_len;
            }
            hash += hash_range(end_value, current_index, start_gap);
            end_len -= start_gap;
            current_index += Offset::from(start_gap);
        }
        hash += hash_range(end_value, current_index, end_len);

        hash
    }

    fn part2(input: Self::Parsed<'_>) -> impl Display {
        let mut gap_offsets_by_size: [ArrayVec<Offset, { 1024 * 2 }>; 9] =
            [const { ArrayVec::new_const() }; 9];
        let mut hash = 0;
        let mut it = input.iter().copied().map(|b| b - b'0');
        let mut end_offset;
        {
            let mut it = it.clone();
            let mut current_offset = 0;
            let mut current_value = 0;
            loop {
                let Some(val_len) = it.next() else { break };
                hash += hash_range(current_value, current_offset, val_len);
                current_offset += Offset::from(val_len);
                current_value += 1;
                let Some(gap) = it.next() else { break };
                if gap == 0 {
                    continue;
                }
                gap_offsets_by_size[usize::from(gap - 1)].push(current_offset);
                current_offset += Offset::from(gap);
            }
            end_offset = current_offset;
            for gap_offsets in gap_offsets_by_size.iter_mut() {
                gap_offsets.reverse();
            }
        }

        let mut end_value = (input.len() / 2) as Int;
        loop {
            let Some(end_len) = it.next_back() else { break };
            end_offset -= Offset::from(end_len);

            let result = gap_offsets_by_size
                .iter_mut()
                .enumerate()
                .map(|(i, gap_offsets)| {
                    let gap_len = i as u8 + 1;
                    (gap_len, gap_offsets)
                })
                .skip(usize::from(end_len - 1))
                .filter(|(_, gap_offsets)| gap_offsets.last().map_or(false, |&x| x < end_offset))
                .min_by_key(|(_, gap_offsets)| gap_offsets.last().copied().unwrap_or(Offset::MAX));
            if let Some((new_gap_len, min_gap_offset)) = result {
                let new_offset = min_gap_offset.pop().unwrap();
                hash -= hash_range(end_value, end_offset, end_len);
                hash += hash_range(end_value, new_offset, end_len);

                let remaining_gap = new_gap_len - end_len;
                if remaining_gap > 0 {
                    let gap_offsets = &mut gap_offsets_by_size[usize::from(remaining_gap - 1)];
                    let new_offset = new_offset + Offset::from(end_len);
                    // Get first index where !(x > new_offset)
                    let to_insert_at = gap_offsets.partition_point(|&x| x > new_offset);
                    gap_offsets.insert(to_insert_at, new_offset);
                }
            }

            let Some(end_gap) = it.next_back() else { break };
            end_value -= 1;
            end_offset -= Offset::from(end_gap);
        }

        hash
    }
}

fn hash_range(value: Int, start: Offset, len: u8) -> Int {
    if len == 0 {
        return 0;
    }
    let max = Int::from(start) + Int::from(len - 1);
    let avg = Int::from(start) + Int::from(max);
    let mut range_sum = avg * (Int::from(len) / 2);
    if len % 2 != 0 {
        range_sum += Int::from(avg / 2);
    }

    value * range_sum
}

crate::codspeed_def!(Day9);

#[test]
fn hash_range_simple() {
    assert_eq!(hash_range(1, 1, 1), 1);
    assert_eq!(hash_range(1, 1, 100), 5050);
    assert_eq!(hash_range(2, 1, 100), 10100);
    assert_eq!(hash_range(1, 1, 101), 5151);
    assert_eq!(hash_range(1, 50, 50), 3725);
}

#[test]
fn example_part1() {
    const EXAMPLE: &str = "2333133121414131402\n";
    assert_eq!(part1(EXAMPLE).to_string(), "1928");
}

#[test]
fn example_part2() {
    const EXAMPLE: &str = "2333133121414131402\n";
    assert_eq!(part2(EXAMPLE).to_string(), "2858");
}
