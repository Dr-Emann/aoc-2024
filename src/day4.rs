use crate::Day;
use memchr::{memchr, memchr_iter};
use std::fmt::Display;

pub struct Day4;

type Int = u32;

#[derive(Debug, Clone)]
pub struct Grid<'a> {
    width: usize,
    data: &'a [u8],
}

impl Grid<'_> {
    #[inline]
    fn stride(&self) -> usize {
        self.width + 1
    }

    fn dxy_to_offset(&self, dx: isize, dy: isize) -> isize {
        dy * self.stride() as isize + dx
    }

    fn has_xmas_at(&self, offset: usize, dx: isize, dy: isize) -> bool {
        debug_assert_eq!(self.data.get(offset), Some(&b'X'));
        let (x, _) = self.xy_from_offset(offset);
        let dxy = self.dxy_to_offset(dx, dy);

        // Check we can even fit XMAS
        if x.checked_add_signed(dx * 3)
            .map_or(true, |x| x >= self.width)
        {
            return false;
        }
        let Some(m_offset) = offset.checked_add_signed(dxy) else {
            debug_assert!(false);
            return false;
        };
        let Some(a_offset) = m_offset.checked_add_signed(dxy) else {
            debug_assert!(false);
            return false;
        };
        let Some(s_offset) = a_offset.checked_add_signed(dxy) else {
            debug_assert!(false);
            return false;
        };
        self.data.get(s_offset) == Some(&b'S')
            && self.data.get(a_offset) == Some(&b'A')
            && self.data.get(m_offset) == Some(&b'M')
    }

    fn has_mas_x_at(&self, offset: usize) -> bool {
        debug_assert_eq!(self.data.get(offset), Some(&b'A'));
        let get_offset = |dx, dy| {
            let diff = self.dxy_to_offset(dx, dy);
            let new_offset = offset.checked_add_signed(diff)?;
            self.data
                .get(new_offset)
                .copied()
                .filter(|&v| v == b'M' || v == b'S')
        };

        let (x, _) = self.xy_from_offset(offset);
        if x == 0 || x >= self.width - 1 {
            return false;
        }

        let Some(up_left) = get_offset(-1, -1) else {
            return false;
        };
        let Some(up_right) = get_offset(1, -1) else {
            return false;
        };
        let Some(down_left) = get_offset(-1, 1) else {
            return false;
        };
        let Some(down_right) = get_offset(1, 1) else {
            return false;
        };

        // Already verified the value is 'M' or 'S'
        up_left != down_right && up_right != down_left
    }

    fn xy_from_offset(&self, offset: usize) -> (usize, usize) {
        (offset % self.stride(), offset / self.stride())
    }
}

impl Day for Day4 {
    type Parsed<'a> = Grid<'a>;

    fn generator(input: &str) -> Self::Parsed<'_> {
        let data = input.as_bytes();
        let width = memchr(b'\n', data).unwrap();
        Grid { width, data }
    }

    fn part1(input: Self::Parsed<'_>) -> impl Display {
        let mut xmas_count = 0;
        for offset in memchr_iter(b'X', input.data) {
            for dx in -1..2 {
                for dy in -1..2 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    xmas_count += Int::from(input.has_xmas_at(offset, dx, dy));
                }
            }
        }
        xmas_count
    }

    fn part2(input: Self::Parsed<'_>) -> impl Display {
        let mut xmas_count = 0;
        for offset in memchr_iter(b'A', input.data) {
            xmas_count += Int::from(input.has_mas_x_at(offset));
        }
        xmas_count
    }
}

crate::codspeed_def!(Day4);
