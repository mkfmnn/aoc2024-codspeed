use memchr::memchr;

const DIM: usize = 130;

pub fn part1(input: &str) -> usize {
    assert_eq!(input.len(), DIM * DIM + DIM);
    part1_inner(input.as_bytes())
}

fn part1_inner(bytes: &[u8]) -> usize {
    let mut visited = [false; DIM * (DIM + 1)];
    let mut visited_count = 0;
    let mut pos = memchr(b'^', bytes).expect("no starting position found");
    let mut dir = Dir::N;
    loop {
        if !visited[pos] {
            visited[pos] = true;
            visited_count += 1;
        }
        if let Some(next_pos) = dir.step(pos) {
            if bytes[next_pos] == b'#' {
                dir = dir.rotate();
            } else {
                pos = next_pos;
            }
        } else {
            return visited_count;
        }
    }
}

pub fn part2(input: &str) -> usize {
    assert_eq!(input.len(), DIM * DIM + DIM);
    part2_inner(input.as_bytes())
}

fn part2_inner(bytes: &[u8]) -> usize {
    0
}

#[derive(Clone, Copy, Debug)]
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
        assert_eq!(0, part2(INPUT));
    }
}
