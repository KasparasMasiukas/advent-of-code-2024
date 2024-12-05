## Introduction

This repo contains Rust solutions for Advent of Code 2024, optimised for speed.

Competing in the [Advent of CodSpeed](https://codspeed.io/advent/) challenge.

[![CodSpeed Badge](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://codspeed.io/KasparasMasiukas/advent-of-code-2024)

## Benchmarks

<!-- BENCHMARK RESULTS START -->
| Challenge       | Low         | Mean        | High        |
|-----------------|-------------|-------------|-------------|
| day1_part1      | 7.9213 µs   | 7.9575 µs   | 7.9942 µs   |
| day1_part2      | 3.1871 µs   | 3.1975 µs   | 3.2079 µs   |
| day2_part1      | 17.052 µs   | 17.136 µs   | 17.226 µs   |
| day2_part2      | 30.250 µs   | 30.364 µs   | 30.480 µs   |
| day3_part1      | 6.5661 µs   | 6.5898 µs   | 6.6155 µs   |
| day3_part2      | 2.6489 µs   | 2.6590 µs   | 2.6696 µs   |
| day4_part1      | 38.130 µs   | 38.293 µs   | 38.465 µs   |
| day4_part2      | 23.112 µs   | 23.201 µs   | 23.294 µs   |

<!-- BENCHMARK RESULTS END -->

### System Information:
* CPU Model: Intel(R) Core(TM) i7-9700K CPU @ 3.60GHz
* Architecture: x86_64
* Number of CPUs: 8
* Total RAM: 31Gi
* L3 Cache: 12 MiB

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
