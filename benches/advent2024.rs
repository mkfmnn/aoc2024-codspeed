use std::hint::black_box;

use aoc2024_codspeed::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn day18(c: &mut Criterion) {
    let input = include_str!("../data/input18.txt");
    let mut group = c.benchmark_group("day18");
    group.bench_function("day18 part1", |b| {
        b.iter(|| day18::part1(black_box(&input)))
    });
    group.bench_function("day18 part2", |b| {
        b.iter(|| day18::part2(black_box(&input)))
    });
    group.finish();
}

criterion_group!(benches, day18);
criterion_main!(benches);
