use std::simd::{cmp::SimdPartialEq, mask8x64, num::SimdUint, u8x64};

use arrayvec::ArrayVec;

pub fn part1(input: &str) -> usize {
    part1_inner(input.as_bytes())
}

fn part1_inner(bytes: &[u8]) -> usize {
    let dim = (bytes.len() as f64).sqrt() as usize;
    assert!(dim < 64);
    let line = dim + 1;
    let mut sum = 0;
    for i in 0..bytes.len() {
        if bytes[i] == b'0' {
            let mut visited = [0u64; 64];
            sum += part1_recurse(bytes, &mut visited, line, i);
        }
    }
    sum
}

#[inline(always)]
fn part1_recurse_check(bytes: &[u8], newpos: usize, char: u8, s: &mut ArrayVec<(u16, u8), 30>) {
    if unsafe { *bytes.get_unchecked(newpos) } != char {
        return;
    }
    unsafe {
        s.push_unchecked((newpos as u16, char));
    }
}

fn part1_recurse(bytes: &[u8], visited: &mut [u64], line: usize, startpos: usize) -> usize {
    let mut s = ArrayVec::<(u16, u8), 30>::new();
    unsafe {
        s.push_unchecked((startpos as u16, b'0'));
    }
    let mut sum = 0;
    while !s.is_empty() {
        let (pos, char) = s.pop().unwrap();
        let pos = pos as usize;
        let (v1, v2) = (pos / 64, pos % 64);
        unsafe {
            std::hint::assert_unchecked(v1 < visited.len());
        }
        if visited[v1] & (1 << v2) != 0 {
            continue;
        }
        visited[v1] |= 1 << v2;
        if char == b'9' {
            sum += 1;
            continue;
        }
        let nextc = char + 1;
        if pos < line {
            if pos >= 1 {
                part1_recurse_check(bytes, pos - 1, nextc, &mut s);
            }
            part1_recurse_check(bytes, pos + 1, nextc, &mut s);
            part1_recurse_check(bytes, pos + line, nextc, &mut s);
        } else if pos + line >= bytes.len() {
            if pos + 1 < bytes.len() {
                part1_recurse_check(bytes, pos + 1, nextc, &mut s);
            }
            part1_recurse_check(bytes, pos - 1, nextc, &mut s);
            part1_recurse_check(bytes, pos - line, nextc, &mut s);
        } else {
            part1_recurse_check(bytes, pos - line, nextc, &mut s);
            part1_recurse_check(bytes, pos - 1, nextc, &mut s);
            part1_recurse_check(bytes, pos + 1, nextc, &mut s);
            part1_recurse_check(bytes, pos + line, nextc, &mut s);
        }
    }
    sum
}

pub fn part2(input: &str) -> usize {
    unsafe { simd_inner(input.as_bytes()) }
}

#[inline(always)]
unsafe fn dosimd(
    map: &ArrayVec<u8x64, 64>,
    input: &ArrayVec<u8x64, 64>,
    output: &mut ArrayVec<u8x64, 64>,
    dim: usize,
    c: u8,
) {
    output.clear();
    for i in 0..dim {
        let mask = map[i].simd_eq(u8x64::splat(c));
        let mut out = mask.select(input[i].rotate_elements_left::<1>(), u8x64::splat(0));
        out += mask.select(input[i].rotate_elements_right::<1>(), u8x64::splat(0));
        if i > 0 {
            out += mask.select(input[i - 1], u8x64::splat(0));
        }
        if i + 1 < dim {
            out += mask.select(input[i + 1], u8x64::splat(0));
        }
        output.push_unchecked(out);
    }
}

unsafe fn simd_inner(bytes: &[u8]) -> usize {
    let dim: usize = (bytes.len() as f32).sqrt().to_int_unchecked();
    assert!(dim < 64);
    let bytes_ptr = bytes.as_ptr();
    let mut map = ArrayVec::<u8x64, 64>::new();
    let load_mask = mask8x64::from_bitmask((1 << dim) - 1);
    for i in 0..dim {
        map.push(u8x64::load_select_ptr(
            bytes_ptr.add(i * (dim + 1)),
            load_mask,
            u8x64::splat(0),
        ));
    }
    let mut a = ArrayVec::<u8x64, 64>::new();
    let mut b = ArrayVec::<u8x64, 64>::new();
    for i in 0..dim {
        let out = map[i]
            .simd_eq(u8x64::splat(b'9'))
            .select(u8x64::splat(1), u8x64::splat(0));
        b.push_unchecked(out);
    }
    dosimd(&map, &b, &mut a, dim, b'8');
    dosimd(&map, &a, &mut b, dim, b'7');
    dosimd(&map, &b, &mut a, dim, b'6');
    dosimd(&map, &a, &mut b, dim, b'5');
    dosimd(&map, &b, &mut a, dim, b'4');
    dosimd(&map, &a, &mut b, dim, b'3');
    dosimd(&map, &b, &mut a, dim, b'2');
    dosimd(&map, &a, &mut b, dim, b'1');
    dosimd(&map, &b, &mut a, dim, b'0');
    // hopefully not more than 255 per row! gotta risk it
    a.iter().map(|r| r.reduce_sum() as usize).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../data/input10.txt");

    #[test]
    fn test_part1() {
        assert_eq!(698, part1(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(1436, part2(INPUT));
    }
}
