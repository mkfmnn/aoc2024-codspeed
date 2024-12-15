use core::str;

use bitvec::{array::BitArray, bitarr, BitArr};
use enumset::{enum_set, EnumSet, EnumSetType};

const DIM: usize = 10;
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
            Dir::W => -(LINE as isize) as usize,
            Dir::N => -1isize as usize,
            Dir::E => 1,
            Dir::S => LINE,
        });
        (newpos < LEN).then_some(newpos)
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
    for dir in [(-(LINE as isize)) as usize, (-1isize) as usize, 1, LINE] {
        let newpos = pos.wrapping_add(dir);
        if newpos >= LEN {
            perimeter += 1;
        } else if bytes[newpos] != char {
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
            let (area, perimiter) = fill2(bytes, &mut visited, pos, bytes[pos], enum_set!());
            // println!("{}: {}, {}", str::from_utf8(&[bytes[pos]]).unwrap(), area, perimiter);
            sum += area * perimiter;
        }
    }
    sum
}

fn fill2(
    bytes: &[u8],
    visited: &mut BitSet,
    pos: usize,
    char: u8,
    sides: EnumSet<Dir>,
) -> (usize, usize) {
    let mut perimeter = 0;
    let mut area = 1;
    visited.set(pos, true);
    let mut mysides = EnumSet::<Dir>::empty();
    for dir in Dir::ALL {
        let newpos = dir.add(pos);
        if newpos.is_none_or(|p| bytes[p] != char) {
            mysides.insert(dir);
            continue;
        }
    }
    for dir in Dir::ALL {
        if mysides.contains(dir) {
            if !sides.contains(dir) {
                perimeter += 1;
            }
        } else {
            let newpos = dir.add(pos).unwrap();
            if !visited[newpos] {
                let r = fill2(bytes, visited, newpos, char, mysides);
                area += r.0;
                perimeter += r.1;
            }
        }
    }
    (area, perimeter)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../data/input12.txt");

    #[test]
    fn test_part1() {
        assert_eq!(0, part1(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(0, part2(INPUT));
    }
}
