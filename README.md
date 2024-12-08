## Introduction

This repo contains Rust solutions for Advent of Code 2024, optimised for speed.

Competing in the [Advent of CodSpeed](https://codspeed.io/advent/) challenge.

[![CodSpeed Badge](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://codspeed.io/KasparasMasiukas/advent-of-code-2024)

## Benchmarks

<!-- BENCHMARK RESULTS START -->
| Challenge       | Low         | Mean        | High        |
|-----------------|-------------|-------------|-------------|
| day1_part1      | 8.1360 µs   | 8.1496 µs   | 8.1651 µs   |
| day1_part2      | 3.5023 µs   | 3.5096 µs   | 3.5167 µs   |
| day2_part1      | 18.038 µs   | 18.057 µs   | 18.080 µs   |
| day2_part2      | 32.172 µs   | 32.573 µs   | 33.001 µs   |
| day3_part1      | 6.9174 µs   | 6.9309 µs   | 6.9454 µs   |
| day3_part2      | 3.0024 µs   | 3.0520 µs   | 3.1078 µs   |
| day4_part1      | 39.775 µs   | 39.850 µs   | 39.951 µs   |
| day4_part2      | 24.042 µs   | 24.075 µs   | 24.107 µs   |
| day5_part1      | 4.3985 µs   | 4.4087 µs   | 4.4182 µs   |
| day5_part2      | 12.783 µs   | 12.866 µs   | 12.955 µs   |
| day6_part1      | 267.16 µs   | 267.54 µs   | 267.88 µs   |
| day6_part2      | 6.8980 ms   | 6.9079 ms   | 6.9184 ms   |
| day7_part1      | 172.78 µs   | 173.03 µs   | 173.27 µs   |
| day7_part2      | 209.82 µs   | 210.27 µs   | 210.69 µs   |

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
