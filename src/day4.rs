use std::simd::{cmp::SimdPartialEq, u8x64};

const DIM: usize = 140;

const fn step(d: u8) -> isize {
    const IDIM: isize = DIM as isize;
    match d {
        0 => 1,         // E
        1 => IDIM + 2,  // SE
        2 => IDIM + 1,  // S
        3 => IDIM,      // SW
        4 => -1,        // W
        5 => -IDIM - 2, // NW
        6 => -IDIM - 1, // N
        7 => -IDIM,     // NE
        _ => unreachable!(),
    }
}

pub fn part1(input: &str) -> usize {
    let bytes = input.as_bytes();
    assert_eq!(DIM * (DIM + 1), bytes.len());
    let mut sum = 0;
    for i in 0..bytes.len() {
        if bytes[i] == b'X' {
            sum += check_dir::<0>(bytes, i)
                + check_dir::<1>(bytes, i)
                + check_dir::<2>(bytes, i)
                + check_dir::<3>(bytes, i)
                + check_dir::<4>(bytes, i)
                + check_dir::<5>(bytes, i)
                + check_dir::<6>(bytes, i)
                + check_dir::<7>(bytes, i);
        }
    }
    sum
}

fn check_dir<const D: u8>(bytes: &[u8], i: usize) -> usize {
    let step = step(D) as usize;
    unsafe {
        if bytes.get(i.wrapping_add(step.wrapping_mul(3))) == Some(&b'S')
            && *bytes.get_unchecked(i.wrapping_add(step.wrapping_mul(2))) == b'A'
            && *bytes.get_unchecked(i.wrapping_add(step)) == b'M'
        {
            1
        } else {
            0
        }
    }
}

pub fn part2(input: &str) -> usize {
    unsafe { part2_inner(input.as_bytes()) }
}

unsafe fn part2_inner(bytes: &[u8]) -> usize {
    let mut sum = 0;

    for i in (142..19598).step_by(64) {
        // range is happily divisible by 64
        sum += part2_check64(bytes, i) as usize;
    }
    sum
}

unsafe fn slice64(bytes: &[u8], i: usize) -> u8x64 {
    u8x64::from_slice(&bytes[i..i + 64])
}

unsafe fn part2_check64(bytes: &[u8], i: usize) -> u32 {
    let a = slice64(bytes, i);
    let d1 = slice64(bytes, i + DIM + 2);
    let d2 = slice64(bytes, i - DIM - 2);
    let d3 = slice64(bytes, i + DIM);
    let d4 = slice64(bytes, i - DIM);
    let is_a = a.simd_eq(u8x64::splat(b'A'));
    let ms = u8x64::splat(30);
    let d1_ok = (d1 ^ d2).simd_eq(ms);
    let d2_ok = (d3 ^ d4).simd_eq(ms);
    let all_ok = is_a & d1_ok & d2_ok;
    all_ok.to_bitmask().count_ones()
}

#[allow(dead_code)]
fn part2_check1(bytes: &[u8], i: usize) -> bool {
    unsafe {
        *bytes.get_unchecked(i) == b'A'
            && *bytes.get_unchecked(i + DIM + 2) ^ *bytes.get_unchecked(i - DIM - 2) == 30
            && *bytes.get_unchecked(i + DIM) ^ *bytes.get_unchecked(i - DIM) == 30
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("data/input4.txt").unwrap();
        assert_eq!(2549, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("data/input4.txt").unwrap();
        assert_eq!(2003, part2(&input));
    }
}
