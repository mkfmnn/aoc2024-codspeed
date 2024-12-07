use memchr::memchr;
use rayon::prelude::*;

const DIM: usize = 130;
const BATCHES: usize = 1024;

pub fn part1(input: &str) -> usize {
    assert_eq!(input.len(), DIM * DIM + DIM);
    unsafe { part1_inner(input.as_bytes()) }
}

unsafe fn part1_inner(bytes: &[u8]) -> usize {
    let mut visited = [0u8; DIM * (DIM + 1)];
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
            _ => std::hint::unreachable_unchecked(),
        }
    }
}

pub fn part2(input: &str) -> usize {
    assert_eq!(input.len(), DIM * DIM + DIM);
    unsafe { part2_inner(input.as_bytes()) }
}

unsafe fn part2_inner(bytes: &[u8]) -> usize {
    let mut checkpoint = [0u8; DIM * (DIM + 1)];
    checkpoint.copy_from_slice(bytes);
    let mut visited = checkpoint.clone();
    let mut pos = memchr(b'^', bytes).expect("no starting position found");
    let mut checkpoint_pos = pos;
    let mut dir = Dir::N;
    let mut checkpoint_dir = dir;
    let mut candidate_obstacles = Vec::<usize>::with_capacity(BATCHES);
    let mut obstacles = 0;
    loop {
        let Some(next_pos) = dir.step(pos) else {
            break;
        };
        match *visited.get_unchecked(next_pos) {
            b'#' => {
                dir = dir.rotate();
            }
            b'.' => {
                *visited.get_unchecked_mut(next_pos) = dir.char();
                candidate_obstacles.push(next_pos);
                pos = next_pos;
            }
            _ => {
                pos = next_pos;
            }
        }
        if candidate_obstacles.len() == BATCHES {
            obstacles += candidate_obstacles
                .par_iter()
                .filter(|&&obs| check_loop(&checkpoint, checkpoint_pos, checkpoint_dir, obs))
                .count();
            candidate_obstacles.clear();
            checkpoint.copy_from_slice(&visited);
            checkpoint_pos = pos;
            checkpoint_dir = dir;
        }
    }

    obstacles
        + candidate_obstacles
            .par_iter()
            .filter(|&&obs| check_loop(&checkpoint, checkpoint_pos, checkpoint_dir, obs))
            .count()
}

fn check_loop(bytes: &[u8], mut pos: usize, mut dir: Dir, obstacle: usize) -> bool {
    let mut visited = [0u8; DIM * (DIM + 1)];
    visited.copy_from_slice(bytes);
    visited[obstacle] = b'#';
    loop {
        let Some(next_pos) = dir.step(pos) else {
            return false;
        };
        let next = unsafe { visited.get_unchecked_mut(next_pos) };
        match *next {
            b'#' => {
                dir = dir.rotate();
            }
            b'.' => {
                *next = dir.char();
                pos = next_pos;
            }
            _ => {
                if *next == dir.char() {
                    return true;
                }
                pos = next_pos;
            }
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

    fn char(self) -> u8 {
        match self {
            Dir::N => b'^',
            Dir::E => b'>',
            Dir::S => b'v',
            Dir::W => b'<',
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
