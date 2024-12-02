## Introduction

This repo contains Rust solutions for Advent of Code 2024, optimised for speed.
Competing in the [Advent of CodSpeed](https://codspeed.io/advent/) challenge.

[![CodSpeed Badge](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://codspeed.io/KasparasMasiukas/advent-of-code-2024)

## Benchmarks

<!-- BENCHMARK RESULTS START -->

| Challenge  | Low       | Mean      | High      |
|------------|-----------|-----------|-----------|
| day1_part1 | 5.8723 µs | 5.9174 µs | 5.9791 µs |
| day1_part2 | 2.0669 µs | 2.0851 µs | 2.1151 µs |
| day2_part1 | 90.890 µs | 91.487 µs | 92.226 µs |
| day2_part2 | 211.98 µs | 214.56 µs | 217.92 µs |

<!-- BENCHMARK RESULTS END -->

## Setup

```shell
cargo install cargo-aoc
```

Log into your Advent of Code account, then take the value of the `session` cookie from browser dev
tools, then set up credentials for `cargo-aoc` to be able to download puzzle inputs:

```shell
cargo aoc credentials {session}
```

Download input for today:

```shell
cargo aoc input
```

Download input for a specific day:

```shell
cargo aoc input -d {day} -y {year}
```

Run solution for today:

```shell
cargo aoc
```

Run solution for specific day:

```shell
cargo aoc -d {day}
```

Update the README.md with benchmark results:

```shell
cargo run --bin update_readme
```
