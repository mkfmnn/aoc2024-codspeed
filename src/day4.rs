use memchr::memchr;

const DIM: usize = 140;

pub fn part1(input: &str) -> usize {
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
    count: usize,
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

/*
enum Dir {
    E,
    SE,
    S,
    SW,
    W,
    NW,
    N,
    NE,
}

impl Dir {
    const ALL: [Dir; 8] = [
        Dir::E,
        Dir::SE,
        Dir::S,
        Dir::SW,
        Dir::W,
        Dir::NW,
        Dir::N,
        Dir::NE,
    ];

    fn step(&self) -> isize {
        const IDIM: isize = DIM as isize;
        match self {
            Dir::E => 1,
            Dir::SE => IDIM + 2,
            Dir::S => IDIM + 1,
            Dir::SW => IDIM,
            Dir::W => -1,
            Dir::NW => -IDIM - 2,
            Dir::N => -IDIM - 1,
            Dir::NE => -IDIM,
        }
    }
}
*/

fn step(d: u8) -> isize {
    const IDIM: isize = DIM as isize;
    match d {
        0 => 1,
        1 => IDIM + 2,
        2 => IDIM + 1,
        3 => IDIM,
        4 => -1,
        5 => -IDIM - 2,
        6 => -IDIM - 1,
        7 => -IDIM,
        _ => unreachable!(),
    }
}

pub fn test1(input: &str) -> usize {
    let mut sum = 0;
    let mut bytes = input.as_bytes();
    while let Some(pos) = memchr(b'S', bytes) {
        sum += 1;
        bytes = &bytes[pos + 1..];
    }
    sum
}

pub fn test2(input: &str) -> usize {
    const LEN: usize = DIM * (DIM + 1);
    let bytes = input.as_bytes();
    assert_eq!(LEN, bytes.len());
    let mut sum = 0;
    for i in 0..(bytes.len() as isize) {
        if bytes[i as usize] == b'S' {
            sum += 1;
        }
    }
    sum
}

pub fn part2(input: &str) -> usize {
    unsafe { part2_inner(input) }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn part2_inner(input: &str) -> usize {
    const LEN: usize = DIM * (DIM + 1);
    let bytes = input.as_bytes();
    assert_eq!(LEN, bytes.len());
    let mut sum = 0;
    for i in 0..(bytes.len() as isize) {
        if bytes[i as usize] == b'S' {
            for d in 0..8 {
                let step = step(d);
                let max_step = i + step * 3;
                if !(0..(LEN as isize)).contains(&max_step) {
                    continue;
                }
                unsafe {
                    if (*bytes.get_unchecked((i + step) as usize) == b'A')
                        && (*bytes.get_unchecked((i + step * 2) as usize) == b'M')
                        && (*bytes.get_unchecked((max_step) as usize) == b'X')
                    {
                        sum += 1;
                    }
                }
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test1() {
        let input = std::fs::read_to_string("data/input4.txt").unwrap();
        assert_eq!(test1(&input), test2(&input));
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
