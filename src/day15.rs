#![allow(unused)]

use std::{collections::HashSet, io::BufRead};

use bitvec::{bitarr, BitArr};

const DIM: usize = 50;
type BitArr = BitArr!(for DIM*DIM);
type BitArr2 = BitArr!(for DIM*DIM*2);

#[derive(Clone, Copy, Debug)]
enum Dir {
    W,
    N,
    E,
    S,
}

impl Dir {
    fn parse(char: u8) -> Option<Dir> {
        match char {
            b'^' => Some(Dir::N),
            b'>' => Some(Dir::E),
            b'<' => Some(Dir::W),
            b'v' => Some(Dir::S),
            b'\n' => None,
            _ => unreachable!(),
        }
    }

    fn add(self, pos: (usize, usize)) -> (usize, usize) {
        match self {
            Dir::W => (pos.0 - 1, pos.1),
            Dir::N => (pos.0, pos.1 - 1),
            Dir::E => (pos.0 + 1, pos.1),
            Dir::S => (pos.0, pos.1 + 1),
        }
    }
}

trait At {
    fn at(&self, pos: (usize, usize)) -> bool;
}

impl At for BitArr {
    fn at(&self, pos: (usize, usize)) -> bool {
        self[pos.0 + pos.1 * DIM]
    }
}

impl At for BitArr2 {
    fn at(&self, pos: (usize, usize)) -> bool {
        self[pos.0 + pos.1 * DIM * 2]
    }
}

pub fn part1(input: &str) -> usize {
    part1_inner(input.as_bytes())
}

fn part1_inner(mut bytes: &[u8]) -> usize {
    let mut walls = bitarr![0; DIM*DIM];
    let mut boxes = bitarr![0; DIM*DIM];
    let mut pos = None;
    for y in 0..DIM {
        let split = bytes.split_at(DIM + 1);
        assert_eq!(split.0[DIM], b'\n');
        bytes = split.1;
        for x in 0..DIM {
            match split.0[x] {
                b'#' => {
                    walls.set(y * DIM + x, true);
                }
                b'O' => {
                    boxes.set(y * DIM + x, true);
                }
                b'@' => {
                    pos = Some((x, y));
                }
                b'.' => {}
                _ => unreachable!(),
            }
        }
    }
    let mut pos = pos.unwrap();
    'outer: for i in 0..bytes.len() {
        let Some(dir) = Dir::parse(bytes[i]) else {
            continue;
        };
        let next = dir.add(pos);
        let mut check = next;
        let mut moved = false;
        loop {
            if walls.at(check) {
                continue 'outer;
            } else if boxes.at(check) {
                moved = true;
                check = dir.add(check);
            } else {
                // empty spot
                if moved {
                    boxes.set(next.0 + next.1 * DIM, false);
                    boxes.set(check.0 + check.1 * DIM, true);
                }
                pos = next;
                break;
            }
        }
    }
    boxes.iter_ones().map(|i| (i % DIM) + 100 * (i / DIM)).sum()
}

pub fn part2(input: &str) -> usize {
    part2_inner(input.as_bytes())
}

fn part2_inner(mut bytes: &[u8]) -> usize {
    let mut walls = bitarr![0; DIM*DIM*2];
    let mut boxes = HashSet::<(usize, usize)>::new();
    let mut pos = None;
    for y in 0..DIM {
        let split = bytes.split_at(DIM + 1);
        assert_eq!(split.0[DIM], b'\n');
        bytes = split.1;
        for x in 0..DIM {
            match split.0[x] {
                b'#' => {
                    walls.set((y * DIM + x) * 2, true);
                    walls.set((y * DIM + x) * 2 + 1, true);
                }
                b'O' => {
                    boxes.insert((x * 2, y));
                }
                b'@' => {
                    pos = Some((x * 2, y));
                }
                b'.' => {}
                _ => unreachable!(),
            }
        }
    }
    let mut pos = pos.unwrap();
    // println!("{boxes:?}");

    'outer: for i in 0..bytes.len() {
        let Some(dir) = Dir::parse(bytes[i]) else {
            continue;
        };
        let next = dir.add(pos);
        // is there a wall at the position?
        if walls[next.1 * DIM * 2 + next.0] {
            continue;
        }
        // is there a box at the position?
        let b = boxes
            .get(&next)
            .or_else(|| boxes.get(&(next.0 - 1, next.1)));
        if let Some(&b) = b {
            if can_push(b, dir, &walls, &boxes) {
                push(b, dir, &walls, &mut boxes);
                pos = next;
            }
        } else {
            // spot is empty
            pos = next;
        }
    }
    boxes.into_iter().map(|(x, y)| x + 100 * y).sum()
}

fn can_push(b: (usize, usize), dir: Dir, walls: &BitArr2, boxes: &HashSet<(usize, usize)>) -> bool {
    match dir {
        Dir::W => {
            !walls.at((b.0 - 1, b.1))
                && (!boxes.contains(&(b.0 - 2, b.1)) || can_push((b.0 - 2, b.1), dir, walls, boxes))
        }
        Dir::E => {
            !walls.at((b.0 + 2, b.1))
                && (!boxes.contains(&(b.0 + 2, b.1)) || can_push((b.0 + 2, b.1), dir, walls, boxes))
        }
        Dir::N | Dir::S => {
            let next = dir.add(b);
            if walls.at(next) || walls.at((next.0 + 1, next.1)) {
                false
            } else {
                for i in [-1, 0, 1] {
                    let n = (next.0.wrapping_add_signed(i), next.1);
                    if boxes.contains(&n) && !can_push(n, dir, walls, boxes) {
                        return false;
                    }
                }
                true
            }
        }
    }
}

fn push(b: (usize, usize), dir: Dir, walls: &BitArr2, boxes: &mut HashSet<(usize, usize)>) {
    match dir {
        Dir::W => {
            if boxes.contains(&(b.0 - 2, b.1)) {
                push((b.0 - 2, b.1), dir, walls, boxes);
            }
            boxes.remove(&b);
            boxes.insert((b.0 - 1, b.1));
        }
        Dir::E => {
            if boxes.contains(&(b.0 + 2, b.1)) {
                push((b.0 + 2, b.1), dir, walls, boxes);
            }
            boxes.remove(&b);
            boxes.insert((b.0 + 1, b.1));
        }
        Dir::N | Dir::S => {
            let next = dir.add(b);
            for i in [-1, 0, 1] {
                let n = (next.0.wrapping_add_signed(i), next.1);
                if boxes.contains(&n) {
                    push(n, dir, walls, boxes);
                }
            }
            boxes.remove(&b);
            boxes.insert(next);
        }
    }
}

fn print(pos: (usize, usize), walls: &BitArr2, boxes: &HashSet<(usize, usize)>) {
    let mut out = String::new();
    for y in 0..DIM {
        for x in 0..DIM * 2 {
            if walls.at((x, y)) {
                out.push('#');
            } else if boxes.contains(&(x, y)) {
                out.push('[');
            } else if boxes.contains(&(x - 1, y)) {
                out.push(']');
            } else if pos == (x, y) {
                out.push('@');
            } else {
                out.push('.');
            }
        }
        out.push('\n');
    }
    print!("{out}");
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../data/input15.txt");

    #[test]
    fn test_part1() {
        assert_eq!(1476771, part1(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(1468005, part2(INPUT));
    }
}
