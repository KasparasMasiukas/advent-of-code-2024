use crate::utils::read_input;
use std::collections::HashMap;

pub fn run() {
    let input = read_input(file!(), false);
    solve_part1(&input);
    solve_part2(&input);
}

fn solve_part1(input: &str) {
    let (mut list1, mut list2) = get_lists(&input);
    list1.sort();
    list2.sort();
    let total: u64 = list1
        .iter()
        .zip(list2.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum();
    println!("Part 1 total: {}", total);
}

fn solve_part2(input: &str) {
    let (list1, list2) = get_lists(&input);
    let occurrences = list2.iter().fold(HashMap::new(), |mut acc, &num| {
        *acc.entry(num).or_insert(0) += 1;
        acc
    });
    let total: u64 = list1
        .iter()
        .map(|num| num * occurrences.get(num).unwrap_or(&0))
        .sum();
    println!("Part 2 total: {}", total);
}

fn get_lists(input: &str) -> (Vec<u64>, Vec<u64>) {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_ascii_whitespace();
            (
                parts.next().unwrap().parse::<u64>().unwrap(),
                parts.next().unwrap().parse::<u64>().unwrap(),
            )
        })
        .unzip()
}
