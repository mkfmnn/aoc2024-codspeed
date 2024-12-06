use std::hint::unreachable_unchecked;

use memchr::memchr;

const DIM: usize = 130;

pub fn part1(input: &str) -> usize {
    assert_eq!(input.len(), DIM * DIM + DIM);
    unsafe { part1_inner(input.as_bytes()) }
}

unsafe fn part1_inner(bytes: &[u8]) -> usize {
    let mut visited: [u8; DIM * (DIM + 1)] = [0; DIM * (DIM + 1)];
    assert_eq!(visited.len(), bytes.len());
    visited.copy_from_slice(bytes);
    let mut visited_count = 1;
    //let mut pos = bytes.iter().position(|&b| b == b'^').unwrap();
    let mut pos = memchr(b'^', bytes).expect("no starting position found");
    let mut dir = Dir::N;
    loop {
        let Some(next_pos) = dir.step(pos) else {
            return visited_count;
        };
        match *visited.get_unchecked(next_pos) {
            b'#' => {
                dir = dir.rotate();
            }
            b'.' => {
                *visited.get_unchecked_mut(next_pos) = b'^';
                visited_count += 1;
                pos = next_pos;
            }
            b'^' => {
                pos = next_pos;
            }
            _ => unreachable_unchecked(),
        }
    }
}

pub fn part2(input: &str) -> usize {
    assert_eq!(input.len(), DIM * DIM + DIM);
    unsafe { part2_inner(input.as_bytes()) }
}

unsafe fn part2_inner(bytes: &[u8]) -> usize {
    let mut visited = [None; DIM * (DIM + 1)];
    let mut obstacle_count = 0;
    //let mut pos = bytes.iter().position(|&b| b == b'^').unwrap();
    let mut pos = memchr(b'^', bytes).expect("no starting position found");
    let mut dir = Dir::N;
    loop {
        if visited.get_unchecked(pos).is_none() {
            *visited.get_unchecked_mut(pos) = Some(dir);
        }
        if let Some(next_pos) = dir.step(pos) {
            if *bytes.get_unchecked(next_pos) == b'#' {
                dir = dir.rotate();
            } else {
                // Before stepping, see if placing an obstacle at 'next' would send us into a loop
                // can't place an obstacle if it's a square we already visited
                if visited.get_unchecked(next_pos).is_none() {
                    if check_loop(&bytes, visited.clone(), pos, dir) {
                        obstacle_count += 1;
                    }
                }
                pos = next_pos;
            }
        } else {
            return obstacle_count;
        }
    }
}

unsafe fn check_loop(
    bytes: &[u8],
    mut visited: [Option<Dir>; DIM * (DIM + 1)],
    start_pos: usize,
    start_dir: Dir,
) -> bool {
    let mut pos = start_pos;
    let mut dir = start_dir;
    let obstacle = dir.step(pos).unwrap();
    dir = dir.rotate();
    loop {
        let Some(next_pos) = dir.step(pos) else {
            return false;
        };
        if *bytes.get_unchecked(next_pos) == b'#' || next_pos == obstacle {
            dir = dir.rotate();
        } else {
            // Before stepping, see if we're stepping onto a path that
            // we already traveled in the same direction
            if visited.get_unchecked(next_pos) == &Some(dir) {
                return true;
            }
            if visited.get_unchecked(pos).is_none() {
                *visited.get_unchecked_mut(pos) = Some(dir);
            }
            pos = next_pos;
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    fn rotate(self) -> Dir {
        match self {
            Dir::N => Dir::E,
            Dir::E => Dir::S,
            Dir::S => Dir::W,
            Dir::W => Dir::N,
        }
    }

    fn step(self, cur_pos: usize) -> Option<usize> {
        match self {
            Dir::N => (cur_pos >= DIM + 1).then(|| cur_pos - (DIM + 1)),
            Dir::E => (cur_pos % (DIM + 1) != DIM).then(|| cur_pos + 1),
            Dir::S => (cur_pos < (DIM - 1) * (DIM + 1)).then(|| cur_pos + (DIM + 1)),
            Dir::W => (cur_pos % (DIM + 1) != 0).then(|| cur_pos - 1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../data/input6.txt");

    #[test]
    fn test_part1() {
        assert_eq!(5153, part1(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(1711, part2(INPUT));
    }
}
