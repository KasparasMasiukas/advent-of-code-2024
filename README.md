## Introduction

This repo contains Rust solutions for Advent of Code 2024, optimised for speed.

Competing in the [Advent of CodSpeed](https://codspeed.io/advent/) challenge.

[![CodSpeed Badge](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://codspeed.io/KasparasMasiukas/advent-of-code-2024)

## Benchmarks

<!-- BENCHMARK RESULTS START -->
| Challenge       | Low         | Mean        | High        |
|-----------------|-------------|-------------|-------------|
| day1_part1      | 8.3343 µs   | 8.3472 µs   | 8.3599 µs   |
| day1_part2      | 3.4721 µs   | 3.4743 µs   | 3.4767 µs   |
| day2_part1      | 18.943 µs   | 18.952 µs   | 18.961 µs   |
| day2_part2      | 32.860 µs   | 33.113 µs   | 33.484 µs   |
| day3_part1      | 7.2946 µs   | 7.3088 µs   | 7.3232 µs   |
| day3_part2      | 2.8053 µs   | 2.8060 µs   | 2.8069 µs   |
| day4_part1      | 39.902 µs   | 39.915 µs   | 39.930 µs   |
| day4_part2      | 25.640 µs   | 25.672 µs   | 25.706 µs   |
| day5_part1      | 4.9639 µs   | 4.9647 µs   | 4.9657 µs   |
| day5_part2      | 10.386 µs   | 10.394 µs   | 10.404 µs   |

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
