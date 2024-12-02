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
    zip(vec1, vec2)
        .map(|(left, right)| left.abs_diff(right))
        .sum()
}

pub fn part2(input: &str) -> i32 {
    let mut vec = Vec::with_capacity(1024);
    let mut hash: HashMap<i32, i32> = HashMap::with_capacity(1024);
    for (l, r) in input_iter(input) {
        vec.push(l);
        *hash.entry(r).or_default() += 1;
    }
    vec.into_iter()
        .map(|n| n * hash.get(&n).copied().unwrap_or_default())
        .sum()
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
        i += 3;
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
