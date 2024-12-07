use std::collections::{hash_set::Entry, HashSet};

use memchr::memchr;

const DIM: usize = 130;
const LINE_LEN: usize = DIM + 1;
const MAP_LEN: usize = DIM * LINE_LEN;

pub fn part1(input: &str) -> usize {
    assert_eq!(input.len(), MAP_LEN);
    unsafe { part1_inner(input.as_bytes()) }
}

unsafe fn part1_inner(bytes: &[u8]) -> usize {
    let mut visited = [0u64; 267];
    let mut visited_count = 0;
    //let mut pos = bytes.iter().position(|&b| b == b'^').unwrap();
    let mut pos = memchr(b'^', bytes).expect("no starting position found");
    let mut dir = Dir::N;
    loop {
        if increment(&mut visited, pos) {
            visited_count += 1;
        }
        if let Some(next_pos) = dir.step(pos) {
            if *bytes.get_unchecked(next_pos) == b'#' {
                dir = dir.rotate();
            } else {
                pos = next_pos;
            }
        } else {
            return visited_count;
        }
    }
}

unsafe fn increment(arr: &mut [u64; 267], i: usize) -> bool {
    let offset = i / 64;
    let bit = 1 << (i % 64);
    let cell = arr.get_unchecked_mut(offset);
    if (*cell & bit) == 0 {
        *cell |= bit;
        return true;
    } else {
        return false;
    }
}

pub fn part2(input: &str) -> usize {
    assert_eq!(input.len(), MAP_LEN);
    unsafe { part2_inner(input.as_bytes()) }
}

unsafe fn part2_inner(bytes: &[u8]) -> usize {
    let map = {
        let mut map = [0u32; MAP_LEN * 4]; // MaybeUninit?
        for i in 0..map.len() {
            let dir = Dir::from((i & 3) as u8);
            let mut pos = i >> 2;
            map[i] = if bytes[pos] == b'\n' {
                0
            } else {
                loop {
                    let Some(next_pos) = dir.step(pos) else {
                        break u32::MAX;
                    };
                    if bytes[next_pos] == b'#' {
                        let mut next_dir = dir.rotate();
                        if next_dir.step(pos).is_some_and(|p| bytes[p] == b'#') {
                            next_dir = next_dir.rotate();
                        }
                        break next_dir.index() as u32 | (pos << 2) as u32;
                    }
                    pos = next_pos;
                }
            }
        }
        map
    };
    // std::hint::black_box(map);

    let mut visited = [0u64; 267];
    let mut obstacle_count = 0;
    //let mut pos = bytes.iter().position(|&b| b == b'^').unwrap();
    let mut pos = memchr(b'^', bytes).expect("no starting position found");
    let mut dir = Dir::N;
    let mut visited_set = HashSet::<u32>::new();
    loop {
        let Some(next_pos) = dir.step(pos) else {
            return obstacle_count;
        };
        if *bytes.get_unchecked(next_pos) == b'#' {
            dir = dir.rotate();
        } else {
            if increment(&mut visited, next_pos) {
                if check_loop(&map, pos, dir.rotate(), next_pos, &mut visited_set) {
                    //println!("obstacle: {},{}", next_pos % LINE_LEN, next_pos / LINE_LEN);
                    obstacle_count += 1;
                }
                visited_set.clear();
            }
            pos = next_pos;
            //println!("visit: {},{}", next_pos % LINE_LEN, next_pos / LINE_LEN);
        }
    }
}

fn check_loop(
    map: &[u32; MAP_LEN * 4],
    pos: usize,
    dir: Dir,
    obstacle: usize,
    visited: &mut HashSet<u32>,
) -> bool {
    let obstacle_x = obstacle % LINE_LEN;
    let obstacle_y = obstacle / LINE_LEN;
    let mut state = pos << 2 | dir.index() as usize;
    loop {
        let mut next_state = map[state as usize] as usize;
        // is the obstacle between the current position and the next one?
        // let x = (state >> 2) % LINE_LEN;
        // let y = (state >> 2) / LINE_LEN;
        match state % 4 {
            0 => {
                if (state >> 2) % LINE_LEN == obstacle_x
                    && (state >> 2) / LINE_LEN > obstacle_y
                    && (next_state == u32::MAX as usize
                        || (next_state >> 2) / LINE_LEN <= obstacle_y)
                {
                    next_state = (obstacle + LINE_LEN) << 2 | 1;
                }
            }
            1 => {
                if (state >> 2) / LINE_LEN == obstacle_y
                    && (state >> 2) % LINE_LEN < obstacle_x
                    && (next_state == u32::MAX as usize
                        || (next_state >> 2) % LINE_LEN >= obstacle_x)
                {
                    next_state = (obstacle - 1) << 2 | 2;
                }
            }
            2 => {
                if (state >> 2) % LINE_LEN == obstacle_x
                    && (state >> 2) / LINE_LEN < obstacle_y
                    && (next_state == u32::MAX as usize
                        || (next_state >> 2) / LINE_LEN >= obstacle_y)
                {
                    next_state = (obstacle - LINE_LEN) << 2 | 3;
                }
            }
            3 => {
                if (state >> 2) / LINE_LEN == obstacle_y
                    && (state >> 2) % LINE_LEN > obstacle_x
                    && (next_state == u32::MAX as usize
                        || (next_state >> 2) % LINE_LEN <= obstacle_x)
                {
                    next_state = (obstacle + 1) << 2 | 0;
                }
            }
            _ => unreachable!(),
        }
        if next_state == u32::MAX as usize {
            return false;
        }
        match visited.entry(next_state as u32) {
            Entry::Occupied(_) => return true,
            e @ Entry::Vacant(_) => e.insert(),
        };
        state = next_state;
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
    const ALL: [Dir; 4] = [Dir::N, Dir::E, Dir::S, Dir::W];

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
            Dir::N => (cur_pos >= LINE_LEN).then(|| cur_pos - LINE_LEN),
            Dir::E => (cur_pos % LINE_LEN != DIM).then(|| cur_pos + 1),
            Dir::S => (cur_pos < (DIM - 1) * LINE_LEN).then(|| cur_pos + LINE_LEN),
            Dir::W => (cur_pos % LINE_LEN != 0).then(|| cur_pos - 1),
        }
    }

    fn index(self) -> u8 {
        match self {
            Dir::N => 0,
            Dir::E => 1,
            Dir::S => 2,
            Dir::W => 3,
        }
    }

    fn from(idx: u8) -> Dir {
        match idx {
            0 => Dir::N,
            1 => Dir::E,
            2 => Dir::S,
            3 => Dir::W,
            _ => unreachable!(),
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
