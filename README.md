## Introduction

This repo contains Rust solutions for Advent of Code 2024, optimised for speed.

Competing in the [Advent of CodSpeed](https://codspeed.io/advent/) challenge.

[![CodSpeed Badge](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://codspeed.io/KasparasMasiukas/advent-of-code-2024)

## Benchmarks

<!-- BENCHMARK RESULTS START -->

| Challenge   | Low       | Mean      | High      |
|-------------|-----------|-----------|-----------|
| day1_part1  | 6.9629 µs | 6.9708 µs | 6.9782 µs |
| day1_part2  | 3.2405 µs | 3.2439 µs | 3.2477 µs |
| day2_part1  | 18.697 µs | 18.717 µs | 18.737 µs |
| day2_part2  | 31.390 µs | 31.422 µs | 31.453 µs |
| day3_part1  | 5.6886 µs | 5.6959 µs | 5.7029 µs |
| day3_part2  | 2.3863 µs | 2.3894 µs | 2.3926 µs |
| day4_part1  | 33.523 µs | 33.573 µs | 33.630 µs |
| day4_part2  | 22.666 µs | 22.712 µs | 22.756 µs |
| day5_part1  | 4.0602 µs | 4.0651 µs | 4.0699 µs |
| day5_part2  | 9.1015 µs | 9.1180 µs | 9.1349 µs |
| day6_part1  | 262.40 µs | 262.79 µs | 263.18 µs |
| day6_part2  | 6.8224 ms | 6.8310 ms | 6.8430 ms |
| day7_part1  | 157.92 µs | 158.08 µs | 158.22 µs |
| day7_part2  | 205.14 µs | 205.35 µs | 205.58 µs |
| day8_part1  | 913.22 ns | 914.99 ns | 916.52 ns |
| day8_part2  | 2.6761 µs | 2.6802 µs | 2.6845 µs |
| day9_part1  | 10.932 µs | 11.514 µs | 12.149 µs |
| day9_part2  | 138.35 µs | 139.46 µs | 140.76 µs |
| day10_part1 | 7.9557 µs | 7.9753 µs | 7.9951 µs |
| day10_part2 | 4.3962 µs | 4.4078 µs | 4.4182 µs |
| day11_part1 | 35.865 ns | 35.905 ns | 35.946 ns |
| day11_part2 | 35.379 ns | 35.416 ns | 35.452 ns |
| day12_part1 | 186.63 µs | 186.70 µs | 186.77 µs |
| day12_part2 | 199.17 µs | 199.30 µs | 199.43 µs |
| day13_part1 | 2.6444 µs | 2.6563 µs | 2.6677 µs |
| day13_part2 | 5.3431 µs | 5.3683 µs | 5.3960 µs |
| day14_part1 | 5.1940 µs | 5.1143 µs | 5.2069 µs |
| day14_part2 | 3.2825 µs | 3.2886 µs | 3.2948 µs |

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
CARGO_ENCODED_RUSTFLAGS="-Ctarget-cpu=native" cargo run --bin update_readme
```
