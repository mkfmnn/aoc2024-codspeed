use std::{collections::HashMap, iter::zip};

pub fn part1(input: &str) -> u32 {
    let (mut vec1, mut vec2) = load_input(input);
    vec1.sort();
    vec2.sort();
    let mut total = 0;
    for (left, right) in zip(vec1, vec2) {
        total += left.abs_diff(right);
    }
    total
}

pub fn part2(input: &str) -> i32 {
    let (vec1, vec2): (Vec<i32>, Vec<i32>) = load_input(input);
    let mut counts = HashMap::new();
    for n in vec2 {
        counts.entry(n).and_modify(|c| *c += 1).or_insert(1);
    }

    let mut score = 0;
    for n in vec1 {
        score += n * counts.get(&n).copied().unwrap_or(0);
    }
    score
}

fn load_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut vec1 = Vec::with_capacity(1024);
    let mut vec2 = Vec::with_capacity(1024);

    for line in input.lines() {
        let mut fields = line.split_ascii_whitespace();
        vec1.push(fields.next().unwrap().parse().unwrap());
        vec2.push(fields.next().unwrap().parse().unwrap());
        assert_eq!(fields.next(), None);
    }
    (vec1, vec2)
}
