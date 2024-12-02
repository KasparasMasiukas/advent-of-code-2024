use advent_of_code_2024::day2::{part1, part2};
use criterion::{criterion_group, criterion_main, Criterion};
mod common;

const DAY: u32 = 2;

fn part1_boxed(input: &str) -> Box<dyn std::fmt::Display> {
    Box::new(part1(input))
}

fn part2_boxed(input: &str) -> Box<dyn std::fmt::Display> {
    Box::new(part2(input))
}

fn run_benchmarks(c: &mut Criterion) {
    common::run_benchmarks(c, DAY, part1_boxed, part2_boxed);
}

criterion_group!(benches, run_benchmarks);
criterion_main!(benches);
