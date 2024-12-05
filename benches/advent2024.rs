use std::hint::black_box;

use aoc2024_codspeed::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn day1(c: &mut Criterion) {
    let input = include_str!("../data/input1.txt");
    let mut group = c.benchmark_group("day1");
    group.bench_function("day1 part1", |b| b.iter(|| day1::part1(black_box(&input))));
    group.bench_function("day1 part2", |b| b.iter(|| day1::part2(black_box(&input))));
    group.finish();
}

fn day2(c: &mut Criterion) {
    let input = include_str!("../data/input2.txt");
    let mut group = c.benchmark_group("day2");
    group.bench_function("day2 part1", |b| b.iter(|| day2::part1(black_box(&input))));
    group.bench_function("day2 part2", |b| b.iter(|| day2::part2(black_box(&input))));
    group.finish();
}

fn day3(c: &mut Criterion) {
    let input = include_str!("../data/input3.txt");
    let mut group = c.benchmark_group("day3");
    group.bench_function("day3 part1", |b| b.iter(|| day3::part1(black_box(&input))));
    group.bench_function("day3 part2", |b| b.iter(|| day3::part2(black_box(&input))));
    group.finish();
}

fn day4(c: &mut Criterion) {
    let input = include_str!("../data/input4.txt");
    let mut group = c.benchmark_group("day4");
    group.bench_function("day4 test1", |b| b.iter(|| day4::test1(black_box(&input))));
    group.bench_function("day4 test2", |b| b.iter(|| day4::test2(black_box(&input))));
    group.bench_function("day4 part1", |b| b.iter(|| day4::part1(black_box(&input))));
    group.bench_function("day4 part2", |b| b.iter(|| day4::part2(black_box(&input))));
    group.finish();
}

criterion_group!(benches, day1, day2, day3, day4);
criterion_main!(benches);
