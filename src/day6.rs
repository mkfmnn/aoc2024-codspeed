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
        for y in 0..DIM {
            let offset = y * LINE_LEN;
            let mut next_state = u32::MAX;
            map[offset << 2 | 3] = next_state;
            for x in 1..DIM {
                if bytes[offset + x - 1] == b'#' {
                    next_state = ((offset + x) << 2 | 0) as u32;
                }
                map[offset + x << 2 | 3] = next_state;
            }
            next_state = u32::MAX;
            map[offset + DIM - 1 << 2 | 1] = next_state;
            for x in (0..DIM - 1).rev() {
                if bytes[offset + x + 1] == b'#' {
                    next_state = ((offset + x) << 2 | 2) as u32;
                }
                map[offset + x << 2 | 1] = next_state;
            }
        }
        for x in 0..DIM {
            let mut next_state = u32::MAX;
            map[x << 2] = next_state;
            for y in 1..DIM {
                let offset = y * LINE_LEN;
                if bytes[offset + x - LINE_LEN] == b'#' {
                    next_state = ((offset + x) << 2 | 1) as u32;
                }
                map[offset + x << 2] = next_state;
            }
            next_state = u32::MAX;
            map[(LINE_LEN * (DIM - 1)) + x << 2 | 2] = next_state;
            for y in (0..DIM - 1).rev() {
                let offset = y * LINE_LEN;
                if bytes[offset + x + LINE_LEN] == b'#' {
                    next_state = ((offset + x) << 2 | 3) as u32;
                }
                map[offset + x << 2 | 2] = next_state;
            }
        }
        map
    };

    let mut visited = [0u64; 267];
    let mut obstacle_count = 0;
    //let mut pos = bytes.iter().position(|&b| b == b'^').unwrap();
    let mut pos = memchr(b'^', bytes).expect("no starting position found");
    let mut dir = Dir::N;
    loop {
        let Some(next_pos) = dir.step(pos) else {
            return obstacle_count;
        };
        if *bytes.get_unchecked(next_pos) == b'#' {
            dir = dir.rotate();
        } else {
            if increment(&mut visited, next_pos) {
                if check_loop(&map, pos, dir.rotate(), next_pos) {
                    obstacle_count += 1;
                }
            }
            pos = next_pos;
        }
    }
}

fn check_loop(map: &[u32; MAP_LEN * 4], pos: usize, dir: Dir, obstacle: usize) -> bool {
    let obstacle_x = obstacle % LINE_LEN;
    let obstacle_y = obstacle / LINE_LEN;
    let mut state = pos << 2 | dir.index() as usize;
    let mut prev_state = state;
    let mut i = 1usize;

    loop {
        let mut next_state = map[state as usize] as usize;
        // is the obstacle between the current position and the next one?
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
        if next_state == prev_state {
            return true;
        }
        state = next_state;
        if (i - 1) & i == 0 {
            prev_state = state;
        }
        i += 1;
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
