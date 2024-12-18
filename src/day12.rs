use core::str;

use bitvec::{bitarr, BitArr};
use enumset::{EnumSet, EnumSetType};

const DIM: usize = 140;
const LINE: usize = DIM + 1;
const LEN: usize = DIM * LINE;
type BitSet = BitArr!(for LEN);

#[derive(EnumSetType, Debug)]
enum Dir {
    W,
    N,
    E,
    S,
}

impl Dir {
    const ALL: EnumSet<Dir> = EnumSet::<Dir>::all();

    fn add(&self, pos: usize) -> Option<usize> {
        let newpos = pos.wrapping_add(match *self {
            Dir::N => -(LINE as isize) as usize,
            Dir::W => -1isize as usize,
            Dir::E => 1,
            Dir::S => LINE,
        });
        (newpos < LEN).then_some(newpos)
    }

    fn next(&self) -> Dir {
        match *self {
            Dir::W => Dir::N,
            Dir::N => Dir::E,
            Dir::E => Dir::S,
            Dir::S => Dir::W,
        }
    }
}

pub fn part1(input: &str) -> usize {
    part1_inner(input.as_bytes())
}

fn part1_inner(bytes: &[u8]) -> usize {
    assert_eq!(bytes.len(), LEN);
    let mut visited: BitSet = bitarr!(0; LEN);
    let mut sum = 0;
    for pos in 0..bytes.len() {
        if !visited[pos] && pos % LINE != DIM {
            let (area, perimiter) = fill(bytes, &mut visited, pos, bytes[pos]);
            // println!("{}: {}, {}", str::from_utf8(&[bytes[pos]]).unwrap(), area, perimiter);
            sum += area * perimiter;
        }
    }
    sum
}

fn fill(bytes: &[u8], visited: &mut BitSet, pos: usize, char: u8) -> (usize, usize) {
    let mut perimeter = 0;
    let mut area = 1;
    visited.set(pos, true);
    for dir in Dir::ALL {
        let Some(newpos) = dir.add(pos) else {
            perimeter += 1;
            continue;
        };
        if bytes[newpos] != char {
            perimeter += 1;
        } else if !visited[newpos] {
            let r = fill(bytes, visited, newpos, char);
            area += r.0;
            perimeter += r.1;
        }
    }
    (area, perimeter)
}

pub fn part2(input: &str) -> usize {
    part2_inner(input.as_bytes())
}

fn part2_inner(bytes: &[u8]) -> usize {
    assert_eq!(bytes.len(), LEN);
    let mut visited: BitSet = bitarr!(0; LEN);
    let mut sum = 0;
    for pos in 0..bytes.len() {
        if !visited[pos] && pos % LINE != DIM {
            let (area, corners) = fill2(bytes, &mut visited, pos, bytes[pos]);
            //println!("{}: {}, {}", str::from_utf8(&[bytes[pos]]).unwrap(), area, corners);
            sum += area * corners;
        }
    }
    sum
}

fn fill2(bytes: &[u8], visited: &mut BitSet, pos: usize, char: u8) -> (usize, usize) {
    let mut corners = 0;
    let mut area = 1;
    visited.set(pos, true);
    for dir in Dir::ALL {
        let s1 = dir.add(pos).is_some_and(|p| bytes[p] == char);
        let s2 = dir.next().add(pos).is_some_and(|p| bytes[p] == char);
        if !s1 && !s2 {
            corners += 1;
        }
        let Some(newpos) = dir.add(pos) else {
            continue;
        };
        if s1 && s2 && bytes[dir.next().add(newpos).unwrap()] != char {
            corners += 1;
        }
        if !visited[newpos] && bytes[newpos] == char {
            let r = fill2(bytes, visited, newpos, char);
            area += r.0;
            corners += r.1;
        }
    }
    (area, corners)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../data/input12.txt");

    #[test]
    fn test_part1() {
        assert_eq!(1464678, part1(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(877492, part2(INPUT));
    }
}
