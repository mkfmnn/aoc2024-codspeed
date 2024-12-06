// #![feature(array_chunks)]
// #![feature(array_windows)]
// #![feature(portable_simd)]
use std::cmp::Ordering;
use std::simd::prelude::*;

static mut RULES: [bool; 0x10000] = [false; 0x10000];

pub fn part1(input: &str) -> usize {
    part1_inner(input.as_bytes())
}

fn part1_inner(bytes: &[u8]) -> usize {
    let mut pages = Vec::<SortablePage>::with_capacity(24);
    parse_pages_simd(parse_rules_simd(bytes), &mut pages)
}

fn part1_checkpages(pages: &Vec<SortablePage>) -> usize {
    if pages.array_windows().all(|[a, b]| a < b) {
        pages[pages.len() / 2].0 as usize
    } else {
        0
    }
}

pub fn part2(input: &str) -> usize {
    part2_inner(input.as_bytes())
}

fn part2_inner(bytes: &[u8]) -> usize {
    let mut bytes = parse_rules_scalar(bytes);
    let mut pages = Vec::<SortablePage>::with_capacity(24);
    let mut sum = 0;
    while !bytes.is_empty() {
        pages.clear();
        bytes = parse_pages_scalar(bytes, &mut pages);
        sum += part2_checkpages(&mut pages);
    }
    sum
}

fn part2_checkpages(pages: &mut Vec<SortablePage>) -> usize {
    if pages.is_sorted() {
        return 0;
    }
    pages.sort();
    pages[pages.len() / 2].0 as usize
}

#[inline]
fn parse_2digit(bytes: &[u8]) -> u8 {
    unsafe {
        bytes
            .get_unchecked(0)
            .wrapping_mul(10)
            .wrapping_add(*bytes.get_unchecked(1))
            .wrapping_sub(16)
    }
}

fn parse_rules_scalar<'a>(bytes: &'a [u8]) -> &'a [u8] {
    #[allow(static_mut_refs)]
    let rules = unsafe { &mut RULES };
    let mut i = 0;
    while bytes[i] != b'\n' {
        unsafe {
            let a = parse_2digit(&bytes[i..i + 2]);
            let b = parse_2digit(&bytes[i + 3..i + 5]);
            *rules.get_unchecked_mut((b as usize) << 8 | a as usize) = true;
        }
        i += 6;
    }
    &bytes[i + 1..]
}

fn parse_rules_simd<'a>(mut bytes: &'a [u8]) -> &'a [u8] {
    #[allow(static_mut_refs)]
    let rules = unsafe { &mut RULES };
    loop {
        let p1 = u8x32::from_slice(bytes);
        let p2 = u8x32::from_slice(&bytes[32..]);
        let p3 = u8x32::from_slice(&bytes[64..]);
        let tens = simd_swizzle!(
            simd_swizzle!(
                p1,
                p2,
                [
                    0, 3, 6, 9, 12, 15, 18, 21, 24, 27, 30, 33, 36, 39, 42, 45, 48, 51, 54, 57, 60,
                    63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
                ]
            ),
            p3,
            [
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 34,
                37, 40, 43, 46, 49, 52, 55, 58, 61
            ]
        );
        let ones = simd_swizzle!(
            simd_swizzle!(
                p1,
                p2,
                [
                    1, 4, 7, 10, 13, 16, 19, 22, 25, 28, 31, 34, 37, 40, 43, 46, 49, 52, 55, 58,
                    61, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
                ]
            ),
            p3,
            [
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 32, 35,
                38, 41, 44, 47, 50, 53, 56, 59, 62
            ]
        );
        let parsed = tens * u8x32::splat(10) + ones - u8x32::splat(b'0'.wrapping_mul(11));
        let term = tens.simd_eq(u8x32::splat(b'\n'));
        if term.any() {
            let parsed = term.select(u8x32::splat(0), parsed);
            let mut len = 0;
            for &[a, b] in parsed.as_array().array_chunks() {
                if a == 0 {
                    return &bytes[len + 1..];
                }
                unsafe {
                    *rules.get_unchecked_mut((b as usize) << 8 | a as usize) = true;
                }
                len += 6;
            }
            unreachable!();
        } else {
            for &[a, b] in parsed.as_array().array_chunks() {
                unsafe {
                    *rules.get_unchecked_mut((b as usize) << 8 | a as usize) = true;
                }
            }
            bytes = &bytes[96..];
        }
    }
}

#[derive(Debug)]
struct SortablePage(u8);

impl Ord for SortablePage {
    fn cmp(&self, other: &Self) -> Ordering {
        let a = self.0;
        let b = other.0;
        let o = unsafe {
            #[allow(static_mut_refs)]
            match *RULES.get_unchecked((a as usize) << 8 | b as usize) {
                true => Ordering::Greater,
                false => Ordering::Less,
            }
        };
        o
    }
}

impl PartialOrd for SortablePage {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for SortablePage {
    fn eq(&self, _other: &Self) -> bool {
        unreachable!()
    }
}

impl Eq for SortablePage {}

fn parse_pages_scalar<'a>(bytes: &'a [u8], pages: &mut Vec<SortablePage>) -> &'a [u8] {
    let mut i = 0;
    loop {
        unsafe {
            let a = parse_2digit(&bytes[i..i + 2]);
            pages.push(SortablePage(a));
            i += 3;
            if *bytes.get_unchecked(i - 1) == b'\n' {
                break;
            }
        }
    }
    &bytes[i..]
}

fn parse_pages_simd<'a>(mut bytes: &'a [u8], pages: &mut Vec<SortablePage>) -> usize {
    let mut sum = 0;
    while bytes.len() >= 96 {
        let p1 = u8x32::from_slice(bytes);
        let p2 = u8x32::from_slice(&bytes[32..]);
        let p3 = u8x32::from_slice(&bytes[64..]);
        let tens = simd_swizzle!(
            simd_swizzle!(
                p1,
                p2,
                [
                    0, 3, 6, 9, 12, 15, 18, 21, 24, 27, 30, 33, 36, 39, 42, 45, 48, 51, 54, 57, 60,
                    63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
                ]
            ),
            p3,
            [
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 34,
                37, 40, 43, 46, 49, 52, 55, 58, 61
            ]
        );
        let ones = simd_swizzle!(
            simd_swizzle!(
                p1,
                p2,
                [
                    1, 4, 7, 10, 13, 16, 19, 22, 25, 28, 31, 34, 37, 40, 43, 46, 49, 52, 55, 58,
                    61, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
                ]
            ),
            p3,
            [
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 32, 35,
                38, 41, 44, 47, 50, 53, 56, 59, 62
            ]
        );
        let delim = simd_swizzle!(
            simd_swizzle!(
                p1,
                p2,
                [
                    2, 5, 8, 11, 14, 17, 20, 23, 26, 29, 32, 35, 38, 41, 44, 47, 50, 53, 56, 59,
                    62, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
                ]
            ),
            p3,
            [
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 33, 36,
                39, 42, 45, 48, 51, 54, 57, 60, 63
            ]
        );
        let parsed = tens * u8x32::splat(10) + ones - u8x32::splat(b'0'.wrapping_mul(11));
        let term = delim.simd_eq(u8x32::splat(b'\n'));
        for (&a, b) in parsed.as_array().iter().zip(term.to_array()) {
            pages.push(SortablePage(a));
            if b {
                sum += part1_checkpages(pages);
                pages.clear();
            }
        }
        bytes = &bytes[96..];
    }
    while !bytes.is_empty() {
        bytes = parse_pages_scalar(&bytes, pages);
        sum += part1_checkpages(pages);
        pages.clear();
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("data/input5.txt").unwrap();
        assert_eq!(5087, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("data/input5.txt").unwrap();
        assert_eq!(4971, part2(&input));
    }
}
