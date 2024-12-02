use advent_of_code_2024::day1::{part1, part2};
use criterion::{criterion_group, criterion_main, Criterion};
mod common;

const DAY: u32 = 1;

fn run_benchmarks(c: &mut Criterion) {
    common::run_benchmarks(c, DAY, part1, part2);
}

criterion_group!(benches, run_benchmarks);
criterion_main!(benches);
