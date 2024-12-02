use aoc2024_codspeed::day2 as day;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const INPUT: &str = include_str!("../data/input2.txt");

fn part1(c: &mut Criterion) {
    c.bench_function("part1", |b| b.iter(|| day::part1(black_box(INPUT))));
}

fn part2(c: &mut Criterion) {
    c.bench_function("part2", |b| b.iter(|| day::part2(black_box(INPUT))));
}

criterion_group!(benches, part1, part2);
criterion_main!(benches);
