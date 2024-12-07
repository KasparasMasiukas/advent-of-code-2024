## Introduction

This repo contains Rust solutions for Advent of Code 2024, optimised for speed.

Competing in the [Advent of CodSpeed](https://codspeed.io/advent/) challenge.

[![CodSpeed Badge](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://codspeed.io/KasparasMasiukas/advent-of-code-2024)

## Benchmarks

<!-- BENCHMARK RESULTS START -->
| Challenge       | Low         | Mean        | High        |
|-----------------|-------------|-------------|-------------|
| day1_part1      | 8.0797 µs   | 8.0876 µs   | 8.0951 µs   |
| day1_part2      | 3.3199 µs   | 3.3231 µs   | 3.3262 µs   |
| day2_part1      | 17.541 µs   | 17.553 µs   | 17.564 µs   |
| day2_part2      | 31.698 µs   | 31.729 µs   | 31.763 µs   |
| day3_part1      | 7.2853 µs   | 7.2997 µs   | 7.3147 µs   |
| day3_part2      | 2.7706 µs   | 2.7744 µs   | 2.7789 µs   |
| day4_part1      | 39.582 µs   | 39.700 µs   | 39.810 µs   |
| day4_part2      | 23.930 µs   | 23.986 µs   | 24.040 µs   |
| day5_part1      | 4.3842 µs   | 4.3948 µs   | 4.4053 µs   |
| day5_part2      | 9.4366 µs   | 9.4596 µs   | 9.4820 µs   |
| day6_part1      | 268.03 µs   | 268.34 µs   | 268.63 µs   |
| day6_part2      | 6.9168 ms   | 6.9236 ms   | 6.9313 ms   |

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
