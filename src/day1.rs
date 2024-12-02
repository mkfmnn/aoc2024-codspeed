use std::{collections::HashMap, iter::zip};

pub fn part1(input: &str) -> u32 {
    let mut vec1 = Vec::with_capacity(1024);
    let mut vec2 = Vec::with_capacity(1024);

    for (l, r) in input_iter(input) {
        vec1.push(l);
        vec2.push(r);
    }
    vec1.sort_unstable();
    vec2.sort_unstable();
    let mut total = 0;
    for (left, right) in zip(vec1, vec2) {
        total += left.abs_diff(right);
    }
    total
}

pub fn part2(input: &str) -> i32 {
    let mut vec = Vec::with_capacity(1024);
    let mut hash = HashMap::with_capacity(1024);
    for (l, r) in input_iter(input) {
        vec.push(l);
        *hash.entry(r).or_default() += 1;
    }
    let mut score = 0;
    for n in vec {
        score += n * hash.get(&n).copied().unwrap_or(0);
    }
    score
}

fn input_iter(input: &str) -> InputIter {
    InputIter {
        input: input.as_bytes(),
        next: 0,
    }
}

struct InputIter<'a> {
    input: &'a [u8],
    next: usize,
}

impl Iterator for InputIter<'_> {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        let mut i = self.next;
        if i >= self.input.len() {
            return None;
        }
        let mut n1 = 0;
        while self.input[i] != b' ' {
            n1 *= 10;
            n1 += (self.input[i] - b'0') as i32;
            i += 1;
        }
        i += 1;
        while self.input[i] == b' ' {
            i += 1;
        }
        let mut n2 = 0;
        while self.input[i] != b'\n' {
            n2 *= 10;
            n2 += (self.input[i] - b'0') as i32;
            i += 1;
        }
        i += 1;
        self.next = i;
        Some((n1, n2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("data/input1.txt").unwrap();
        assert_eq!(1258579, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("data/input1.txt").unwrap();
        assert_eq!(23981443, part2(&input));
    }
}
