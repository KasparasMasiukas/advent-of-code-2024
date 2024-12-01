use std::collections::HashMap;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u64 {
    let (mut list1, mut list2) = get_lists(input);
    list1.sort_unstable();
    list2.sort_unstable();
    list1
        .iter()
        .zip(list2.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u64 {
    let (list1, list2) = get_lists(input);
    let occurrences = list2.iter().fold(HashMap::new(), |mut acc, &num| {
        *acc.entry(num).or_insert(0) += 1;
        acc
    });
    list1
        .iter()
        .map(|num| num * occurrences.get(num).unwrap_or(&0))
        .sum()
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

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn part1_example() {
        assert_eq!(part1(TEST_INPUT), 11);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(TEST_INPUT), 31);
    }
}
