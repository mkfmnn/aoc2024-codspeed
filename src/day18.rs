use std::ops::Index;

use arrayvec::ArrayVec;
use bitvec::{bitarr, BitArr};
use pathfinding::prelude::dijkstra;

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

fn successors(blocks: &Bitmap) -> impl FnMut(&Pos) -> Vec<(Pos, usize)> + use<'_> {
    |n| {
        let mut v = vec![];
        for dir in [(0, 1), (-1, 0), (0, -1), (1, 0)] {
            let next = (
                n.0.wrapping_add_signed(dir.0),
                n.1.wrapping_add_signed(dir.1),
            );
            if next.0 >= DIM as u8 || next.1 >= DIM as u8 {
                continue;
            }
            if blocks[next] {
                continue;
            }
            v.push((next, 1));
        }
        v
    }
}

fn parse(line: &str) -> Pos {
    let (a, b) = line.split_once(',').unwrap();
    (a.parse().unwrap(), b.parse().unwrap())
}

pub fn part1(input: &str) -> usize {
    let mut blocks = Bitmap(bitarr![0; DIM*DIM]);
    for line in input.lines().take(1024) {
        let pos = parse(line);
        blocks.set(pos, true);
    }
    dijkstra(&(0, 0), successors(&blocks), |&n| n == GOAL)
        .unwrap()
        .1
}

pub fn part2(input: &str) -> String {
    let blocks: ArrayVec<_, 4000> = input.lines().map(parse).collect();
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
