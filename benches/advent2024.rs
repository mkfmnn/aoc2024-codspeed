use aoc2024_codspeed::day2 as day;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const INPUT: &str = include_str!("../data/input2.txt");

fn parse_lines(c: &mut Criterion) {
    c.bench_function("parse lines", |b| {
        b.iter(|| day::parse_lines(black_box(INPUT)))
    });
}

fn parse_all(c: &mut Criterion) {
    c.bench_function("parse all", |b| b.iter(|| day::parse_all(black_box(INPUT))));
}

fn part1(c: &mut Criterion) {
    c.bench_function("day2 part1", |b| b.iter(|| day::part1(black_box(INPUT))));
}

fn part2(c: &mut Criterion) {
    c.bench_function("day2 part2", |b| b.iter(|| day::part2(black_box(INPUT))));
}

criterion_group!(benches, parse_lines, parse_all, part1, part2);
criterion_main!(benches);
