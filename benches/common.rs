use criterion::{black_box, Criterion};
use reqwest::blocking::Client;
use std::env;
use std::fmt::Display;

pub fn fetch_input(day: u32) -> String {
    let session_cookie = env::var("AOC_SESSION").expect("AOC_SESSION environment variable not set");
    let client = Client::new();
    let url = format!("https://adventofcode.com/2024/day/{}/input", day);

    client
        .get(&url)
        .header("Cookie", format!("session={}", session_cookie))
        .send()
        .expect("Failed to send request")
        .text()
        .expect("Failed to read response text")
}

pub fn run_benchmarks<D1, D2>(c: &mut Criterion, day: u32, part1: D1, part2: D2)
where
    D1: Fn(&str) -> Box<dyn Display> + 'static,
    D2: Fn(&str) -> Box<dyn Display> + 'static,
{
    let input = fetch_input(day);

    c.bench_function(&format!("day{}_part1", day), |b| {
        b.iter(|| part1(black_box(&input)))
    });

    c.bench_function(&format!("day{}_part2", day), |b| {
        b.iter(|| part2(black_box(&input)))
    });
}
