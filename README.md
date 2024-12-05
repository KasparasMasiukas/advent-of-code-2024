## Introduction

This repo contains Rust solutions for Advent of Code 2024, optimised for speed.

Competing in the [Advent of CodSpeed](https://codspeed.io/advent/) challenge.

[![CodSpeed Badge](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://codspeed.io/KasparasMasiukas/advent-of-code-2024)

## Benchmarks

<!-- BENCHMARK RESULTS START -->
| Challenge       | Low         | Mean        | High        |
|-----------------|-------------|-------------|-------------|
| day1_part1      | 5.6769 µs   | 5.7093 µs   | 5.7463 µs   |
| day1_part2      | 2.1910 µs   | 2.1993 µs   | 2.2081 µs   |
| day2_part1      | 11.737 µs   | 11.830 µs   | 11.922 µs   |
| day2_part2      | 18.590 µs   | 18.767 µs   | 18.944 µs   |
| day3_part1      | 6.1346 µs   | 6.1845 µs   | 6.2418 µs   |
| day3_part2      | 2.3862 µs   | 2.3924 µs   | 2.3996 µs   |

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
