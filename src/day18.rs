use std::{mem::swap, ops::Index};

use arrayvec::ArrayVec;
use bitvec::{bitarr, BitArr};

const DIM: usize = 71;
const GOAL: (u8, u8) = ((DIM - 1) as u8, (DIM - 1) as u8);
type Pos = (u8, u8);
pub struct Bitmap(BitArr!(for DIM*DIM));

impl Bitmap {
    fn set(&mut self, index: Pos, value: bool) {
        self.0
            .set(index.0 as usize + (index.1 as usize) * DIM, value);
    }
}

impl Index<Pos> for Bitmap {
    type Output = bool;

    fn index(&self, index: Pos) -> &Self::Output {
        &self.0[index.0 as usize + (index.1 as usize) * DIM]
    }
}

pub fn part1(input: &str) -> usize {
    let mut blocks = Bitmap(bitarr![0; DIM*DIM]);
    let mut visited = Bitmap(bitarr![0; DIM*DIM]);
    for pos in parse_input(input).take(1024) {
        blocks.set(pos, true);
    }
    let mut frontier = Vec::with_capacity(DIM * 2);
    let mut next_frontier = Vec::with_capacity(DIM * 2);
    frontier.push((0, 0));
    let mut i = 1;
    loop {
        debug_assert!(!frontier.is_empty());
        for &pos in &frontier {
            if pos.0 != 0 {
                visit1(
                    (pos.0 - 1, pos.1),
                    &blocks,
                    &mut visited,
                    &mut next_frontier,
                );
            }
            if pos.1 != 0 {
                visit1(
                    (pos.0, pos.1 - 1),
                    &blocks,
                    &mut visited,
                    &mut next_frontier,
                );
            }
            if pos.0 + 1 != DIM as u8 {
                visit1(
                    (pos.0 + 1, pos.1),
                    &blocks,
                    &mut visited,
                    &mut next_frontier,
                );
            }
            if pos.1 + 1 != DIM as u8 {
                visit1(
                    (pos.0, pos.1 + 1),
                    &blocks,
                    &mut visited,
                    &mut next_frontier,
                );
            }
        }
        if visited[GOAL] {
            return i;
        }
        i += 1;
        frontier.clear();
        swap(&mut frontier, &mut next_frontier);
    }
}

#[inline(always)]
fn visit1(n: (u8, u8), blocks: &Bitmap, visited: &mut Bitmap, next_frontier: &mut Vec<(u8, u8)>) {
    if !visited[n] && !blocks[n] {
        visited.set(n, true);
        next_frontier.push(n);
    }
}

pub fn part2(input: &str) -> String {
    let blocks: ArrayVec<_, 4000> = parse_input(input).collect();
    let mut bitmap = Bitmap(bitarr![0; DIM*DIM]);
    let mut visited = Bitmap(bitarr![0; DIM*DIM]);
    for &pos in &blocks {
        bitmap.set(pos, true);
    }
    visit((0, 0), &mut visited, &bitmap);
    for pos in blocks.into_iter().rev() {
        bitmap.set(pos, false);
        if visited_adjacent(pos, &visited) {
            visit(pos, &mut visited, &bitmap);
            if visited[GOAL] {
                return format!("{},{}", pos.0, pos.1);
            }
        }
    }
    unreachable!();
}

fn visited_adjacent(pos: (u8, u8), visited: &Bitmap) -> bool {
    pos.0 != 0 && visited[(pos.0 - 1, pos.1)]
        || pos.1 != 0 && visited[(pos.0, pos.1 - 1)]
        || pos.0 + 1 != DIM as u8 && visited[(pos.0 + 1, pos.1)]
        || pos.1 + 1 != DIM as u8 && visited[(pos.0, pos.1 + 1)]
}

#[inline(always)]
fn maybe_visit(pos: Pos, visited: &mut Bitmap, bitmap: &Bitmap) {
    if !visited[pos] && !bitmap[pos] {
        visit(pos, visited, bitmap);
    }
}

fn visit(pos: Pos, visited: &mut Bitmap, bitmap: &Bitmap) {
    visited.set(pos, true);
    if pos.0 != 0 {
        maybe_visit((pos.0 - 1, pos.1), visited, bitmap);
    }
    if pos.1 != 0 {
        maybe_visit((pos.0, pos.1 - 1), visited, bitmap);
    }
    if pos.0 + 1 != DIM as u8 {
        maybe_visit((pos.0 + 1, pos.1), visited, bitmap);
    }
    if pos.1 + 1 != DIM as u8 {
        maybe_visit((pos.0, pos.1 + 1), visited, bitmap);
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = (u8, u8)> + use<'_> {
    InputIter {
        input: input.as_bytes(),
    }
}

#[inline(always)]
fn parse2(input: &[u8], offset: usize) -> u8 {
    input[offset]
        .wrapping_mul(10)
        .wrapping_add(input[offset + 1])
        .wrapping_sub(16)
}

struct InputIter<'a> {
    input: &'a [u8],
}
impl Iterator for InputIter<'_> {
    type Item = (u8, u8);

    fn next(&mut self) -> Option<Self::Item> {
        if self.input.len() >= 8 {
            if self.input[5] == b'\n' {
                let n = (u64::from_le_bytes(self.input[0..8].try_into().unwrap()) & 0xffff00ffff)
                    - 0x3030003030;
                let n = (n * 2561) >> 8;
                let n = n.to_le_bytes();
                self.input = &self.input[6..];
                Some((n[0], n[3]))
            } else if self.input[4] == b'\n' {
                if self.input[1] == b',' {
                    let r = (self.input[0] - b'0', parse2(self.input, 2));
                    self.input = &self.input[5..];
                    Some(r)
                } else {
                    let r = (parse2(self.input, 0), self.input[3] - b'0');
                    self.input = &self.input[5..];
                    Some(r)
                }
            } else {
                debug_assert_eq!(self.input[3], b'\n');
                let r = (self.input[0] - b'0', self.input[2] - b'0');
                self.input = &self.input[4..];
                Some(r)
            }
        } else {
            if self.input.is_empty() {
                None
            } else {
                let mut a = 0;
                let mut b = 0;
                let mut i = 0;
                while self.input[i] != b',' {
                    a = a * 10 + (self.input[i] - b'0');
                    i += 1;
                }
                i += 1;
                while self.input[i] != b'\n' {
                    b = b * 10 + (self.input[i] - b'0');
                    i += 1;
                }
                self.input = &self.input[i + 1..];
                Some((a, b))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../data/input18.txt");

    #[test]
    fn test_part1() {
        assert_eq!(284, part1(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!("51,50", part2(INPUT));
    }
}
