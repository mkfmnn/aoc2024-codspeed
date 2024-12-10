use arrayvec::ArrayVec;

pub fn part1(input: &str) -> usize {
    part1_inner(input.as_bytes())
}

fn part1_inner(bytes: &[u8]) -> usize {
    let dim = (bytes.len() as f64).sqrt() as usize;
    assert!(dim < 64);
    let line = dim + 1;
    let mut sum = 0;
    for i in 0..bytes.len() {
        if bytes[i] == b'0' {
            let mut visited = [0u64; 64];
            sum += part1_recurse(bytes, &mut visited, line, i, b'0');
        }
    }
    sum
}

#[inline(always)]
fn part1_recurse_check(
    bytes: &[u8],
    visited: &mut [u64],
    line: usize,
    newpos: usize,
    char: u8,
) -> usize {
    if unsafe { *bytes.get_unchecked(newpos) } != char {
        return 0;
    }
    part1_recurse(bytes, visited, line, newpos, char)
}

fn part1_recurse(bytes: &[u8], visited: &mut [u64], line: usize, pos: usize, char: u8) -> usize {
    let (v1, v2) = (pos / 64, pos % 64);
    unsafe {
        std::hint::assert_unchecked(v1 < visited.len());
    }
    if visited[v1] & (1 << v2) != 0 {
        return 0;
    }
    visited[v1] |= 1 << v2;
    if char == b'9' {
        return 1;
    }
    let nextc = char + 1;
    let mut sum = 0;
    if pos < line {
        if pos >= 1 {
            sum += part1_recurse_check(bytes, visited, line, pos - 1, nextc);
        }
        sum += part1_recurse_check(bytes, visited, line, pos + 1, nextc);
        sum += part1_recurse_check(bytes, visited, line, pos + line, nextc);
    } else if pos + line >= bytes.len() {
        if pos + 1 < bytes.len() {
            sum += part1_recurse_check(bytes, visited, line, pos + 1, nextc);
        }
        sum += part1_recurse_check(bytes, visited, line, pos - 1, nextc);
        sum += part1_recurse_check(bytes, visited, line, pos - line, nextc);
    } else {
        sum += part1_recurse_check(bytes, visited, line, pos - line, nextc);
        sum += part1_recurse_check(bytes, visited, line, pos - 1, nextc);
        sum += part1_recurse_check(bytes, visited, line, pos + 1, nextc);
        sum += part1_recurse_check(bytes, visited, line, pos + line, nextc);
    }
    sum
}

pub fn part2(input: &str) -> usize {
    part2_inner(input.as_bytes())
}

fn part2_inner(bytes: &[u8]) -> usize {
    let dim = (bytes.len() as f64).sqrt() as usize;
    assert!(dim < 64);
    let line = dim + 1;
    let mut sum = 0;
    for i in 0..bytes.len() {
        if bytes[i] == b'0' {
            sum += part2_recurse(bytes, line, i);
        }
    }
    sum
}

#[inline(always)]
fn part2_recurse_check(
    bytes: &[u8],
    newpos: usize,
    char: u8,
    sum: &mut usize,
    s: &mut ArrayVec<(u16, u8), 32>,
) {
    if unsafe { *bytes.get_unchecked(newpos) } != char {
        return;
    }
    if char == b'9' {
        *sum += 1;
        return;
    }
    unsafe {
        s.push_unchecked((newpos as u16, char));
    }
}

fn part2_recurse(bytes: &[u8], line: usize, startpos: usize) -> usize {
    let mut s = ArrayVec::<(u16, u8), 32>::new();
    unsafe {
        s.push_unchecked((startpos as u16, b'0'));
    }
    let mut sum = 0;
    while !s.is_empty() {
        let (pos, char) = s.pop().unwrap();
        let pos = pos as usize;
        let nextc = char + 1;
        // push all adjacent onto the stack
        if pos < line {
            if pos >= 1 {
                part2_recurse_check(bytes, pos - 1, nextc, &mut sum, &mut s);
            }
            part2_recurse_check(bytes, pos + 1, nextc, &mut sum, &mut s);
            part2_recurse_check(bytes, pos + line, nextc, &mut sum, &mut s);
        } else if pos + line >= bytes.len() {
            if pos + 1 < bytes.len() {
                part2_recurse_check(bytes, pos + 1, nextc, &mut sum, &mut s);
            }
            part2_recurse_check(bytes, pos - 1, nextc, &mut sum, &mut s);
            part2_recurse_check(bytes, pos - line, nextc, &mut sum, &mut s);
        } else {
            part2_recurse_check(bytes, pos - line, nextc, &mut sum, &mut s);
            part2_recurse_check(bytes, pos - 1, nextc, &mut sum, &mut s);
            part2_recurse_check(bytes, pos + 1, nextc, &mut sum, &mut s);
            part2_recurse_check(bytes, pos + line, nextc, &mut sum, &mut s);
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../data/input10.txt");

    #[test]
    fn test_part1() {
        assert_eq!(698, part1(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(1436, part2(INPUT));
    }
}
