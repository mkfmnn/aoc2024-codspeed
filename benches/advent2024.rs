use aoc2024_codspeed::day1;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn day1p1(c: &mut Criterion) {
    let input = include_str!("../data/input1.txt");
    c.bench_function("day1 p1", |b| b.iter(|| day1::part1(black_box(&input))));
}

fn day1p2(c: &mut Criterion) {
    let input = include_str!("../data/input1.txt");
    c.bench_function("day1 p2", |b| b.iter(|| day1::part2(black_box(&input))));
}

criterion_group!(benches, day1p1, day1p2);
criterion_main!(benches);
