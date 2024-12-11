use std::simd::{mask8x64, u8x64};

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
    part2_inner(input.as_bytes())
}

fn part2_inner(bytes: &[u8]) -> usize {
    let dim = (bytes.len() as f64).sqrt() as usize;
    assert!(dim < 64);
    let line = dim + 1;
    let mut sum = 0;
    for i in 0..bytes.len() {
        if bytes[i] == b'0' {
            sum += part2_recurse(bytes, line, i);
        }
    }
    sum
}

#[inline(always)]
fn part2_recurse_check(
    bytes: &[u8],
    newpos: usize,
    char: u8,
    sum: &mut usize,
    s: &mut ArrayVec<(u16, u8), 30>,
) {
    if unsafe { *bytes.get_unchecked(newpos) } != char {
        return;
    }
    if char == b'9' {
        *sum += 1;
        return;
    }
    unsafe {
        s.push_unchecked((newpos as u16, char));
    }
}

fn part2_recurse(bytes: &[u8], line: usize, startpos: usize) -> usize {
    let mut s = ArrayVec::<(u16, u8), 30>::new();
    unsafe {
        s.push_unchecked((startpos as u16, b'0'));
    }
    let mut sum = 0;
    while !s.is_empty() {
        let (pos, char) = s.pop().unwrap();
        let pos = pos as usize;
        let nextc = char + 1;
        if pos < line {
            if pos >= 1 {
                part2_recurse_check(bytes, pos - 1, nextc, &mut sum, &mut s);
            }
            part2_recurse_check(bytes, pos + 1, nextc, &mut sum, &mut s);
            part2_recurse_check(bytes, pos + line, nextc, &mut sum, &mut s);
        } else if pos + line >= bytes.len() {
            if pos + 1 < bytes.len() {
                part2_recurse_check(bytes, pos + 1, nextc, &mut sum, &mut s);
            }
            part2_recurse_check(bytes, pos - 1, nextc, &mut sum, &mut s);
            part2_recurse_check(bytes, pos - line, nextc, &mut sum, &mut s);
        } else {
            part2_recurse_check(bytes, pos - line, nextc, &mut sum, &mut s);
            part2_recurse_check(bytes, pos - 1, nextc, &mut sum, &mut s);
            part2_recurse_check(bytes, pos + 1, nextc, &mut sum, &mut s);
            part2_recurse_check(bytes, pos + line, nextc, &mut sum, &mut s);
        }
    }
    sum
}

pub fn simd(input: &str) -> usize {
    unsafe { simd_inner(input.as_bytes()) }
}

unsafe fn simd_inner(bytes: &[u8]) -> usize {
    let dim: usize = (bytes.len() as f32).sqrt().to_int_unchecked();
    assert!(dim < 64);
    let bytes_ptr = bytes.as_ptr();
    let mut map = ArrayVec::<u8x64, 64>::new();
    let load_mask = mask8x64::from_bitmask((1 << dim) - 1);
    //println!("{load_mask:?}");

    for i in 0..dim {
        map.push(u8x64::load_select_ptr(
            bytes_ptr.add(i * (dim + 1)),
            load_mask,
            u8x64::splat(0),
        ));
    }
    //println!("{map:?}");

    std::hint::black_box(&map);
    0
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

    #[test]
    fn test_simd() {
        assert_eq!(0, simd(INPUT));
    }
}
