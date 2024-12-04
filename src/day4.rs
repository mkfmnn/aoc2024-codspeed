use std::hint::black_box;

use memchr::memchr;

const DIM: usize = 140;

pub fn part1(input: &str) -> u32 {
    let matrix = Matrix(input.as_bytes());
    let mut visitor = Visitor {
        state: VisitorState::N,
        count: 0,
    };
    for y in 0..DIM {
        for x in 0..DIM {
            visitor.visit(matrix.get(x, y));
        }
        visitor.finish();
    }
    for x in 0..DIM {
        for y in 0..DIM {
            visitor.visit(matrix.get(x, y));
        }
        visitor.finish();
    }
    // NE diagonal
    for y in 0..DIM {
        for x in 0..=y {
            visitor.visit(matrix.get(x, y - x));
        }
        visitor.finish();
    }
    for x in 1..DIM {
        for y in 0..(DIM - x) {
            visitor.visit(matrix.get(x + y, DIM - 1 - y));
        }
        visitor.finish();
    }
    // SE diagonal
    for y in 0..DIM {
        for x in 0..(DIM - y) {
            visitor.visit(matrix.get(x, y + x));
        }
        visitor.finish();
    }
    for x in 1..DIM {
        for y in 0..(DIM - x) {
            visitor.visit(matrix.get(x + y, y));
        }
        visitor.finish();
    }

    visitor.count
}

pub fn part1a(input: &str) -> u32 {
    let matrix = Matrix(input.as_bytes());
    let mut visitor = Visitor {
        state: VisitorState::N,
        count: 0,
    };
    for y in 0..DIM {
        for x in 0..DIM {
            visitor.visit(matrix.get(x, y));
        }
        visitor.finish();
    }
    visitor.count
}

pub fn part1b(input: &str) -> usize {
    input.matches("XMAS").count() + input.matches("SAMX").count()
}

pub fn test3(input: &str) -> usize {
    const LEN: usize = (DIM + 1) * DIM;
    let buf = {
        let buf: [std::mem::MaybeUninit<u8>; LEN] =
            [const { std::mem::MaybeUninit::uninit() }; LEN];
        let mut buf2 = unsafe { std::mem::transmute::<_, [u8; LEN]>(buf) };
        buf2.copy_from_slice(input.as_bytes());
        buf2
    };
    black_box(&buf);
    0
}

pub fn test4(input: &str) -> usize {
    memchr(b'Z', input.as_bytes()).unwrap_or_default()
}

struct Matrix<'a>(&'a [u8]);

impl Matrix<'_> {
    // #[inline(always)]
    fn get(&self, x: usize, y: usize) -> u8 {
        self.0[y * (DIM + 1) + x]
    }
}

enum VisitorState {
    N,
    F1,
    F2,
    F3,
    R1,
    R2,
    R3,
}

struct Visitor {
    state: VisitorState,
    count: u32,
}

impl Visitor {
    fn visit(&mut self, c: u8) {
        use VisitorState::*;
        self.state = match (&self.state, c) {
            (F1, b'M') => F2,
            (F2, b'A') => F3,
            (F3, b'S') => {
                self.count += 1;
                R1
            }
            (R1, b'A') => R2,
            (R2, b'M') => R3,
            (R3, b'X') => {
                self.count += 1;
                F1
            }
            (_, b'X') => F1,
            (_, b'S') => R1,
            _ => N,
        }
    }

    fn finish(&mut self) {
        self.state = VisitorState::N;
    }
}

pub fn part2(_input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1partial() {
        let input = std::fs::read_to_string("data/input4.txt").unwrap();
        assert_eq!(part1a(&input) as usize, part1b(&input));
    }

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("data/input4.txt").unwrap();
        assert_eq!(2549, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("data/input4.txt").unwrap();
        assert_eq!(2003, part2(&input));
    }
}
