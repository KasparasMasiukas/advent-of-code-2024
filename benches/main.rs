use criterion::{criterion_group, criterion_main, Criterion};
mod common;

macro_rules! generate_day_benchmarks {
    ($($day:literal),*) => {
        paste::paste! {
            $(
                mod [<day $day>] {
                    use super::*;
                    use advent_of_code_2024::[<day $day>]::{part1, part2};

                    pub fn run(c: &mut Criterion) {
                        common::run_benchmarks(
                            c,
                            $day,
                            |input| Box::new(part1(input)),
                            |input| Box::new(part2(input)),
                        );
                    }
                }
            )*

            pub fn register_benchmarks(c: &mut Criterion) {
                $(
                    [<day $day>]::run(c);
                )*
            }
        }
    };
}

generate_day_benchmarks!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16); // Add new days here

criterion_group!(benches, register_benchmarks);
criterion_main!(benches);
