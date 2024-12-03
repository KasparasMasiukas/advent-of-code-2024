## Introduction

This repo contains Rust solutions for Advent of Code 2024, optimised for speed.
Competing in the [Advent of CodSpeed](https://codspeed.io/advent/) challenge.

[![CodSpeed Badge](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://codspeed.io/KasparasMasiukas/advent-of-code-2024)

## Benchmarks

<!-- BENCHMARK RESULTS START -->
| Challenge       | Low         | Mean        | High        |
|-----------------|-------------|-------------|-------------|
| day1_part1      | 5.7455 µs   | 5.7672 µs   | 5.7960 µs   |
| day1_part2      | 2.0359 µs   | 2.0458 µs   | 2.0620 µs   |
| day2_part1      | 11.821 µs   | 11.992 µs   | 12.209 µs   |
| day2_part2      | 18.297 µs   | 18.361 µs   | 18.429 µs   |

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
