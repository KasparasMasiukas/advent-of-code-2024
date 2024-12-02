#[aoc(day2, part1, naive)]
pub fn part1_naive(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<u8>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|level| {
            let increasing = level.windows(2).all(|w| w[0] < w[1]);
            let decreasing = level.windows(2).all(|w| w[0] > w[1]);
            let diff_in_range = level.windows(2).all(|w| {
                let diff = w[0].abs_diff(w[1]);
                diff >= 1 && diff <= 3
            });
            (increasing || decreasing) && diff_in_range
        })
        .count() as usize
}

#[aoc(day2, part2, naive)]
pub fn part2_naive(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<u8>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|main_level| {
            for i in 0..main_level.len() {
                let mut level = main_level.clone();
                level.remove(i);
                let increasing = level.windows(2).all(|w| w[0] < w[1]);
                let decreasing = level.windows(2).all(|w| w[0] > w[1]);
                let diff_in_range = level.windows(2).all(|w| {
                    let diff = w[0].abs_diff(w[1]);
                    diff >= 1 && diff <= 3
                });
                if (increasing || decreasing) && diff_in_range {
                    return true;
                }
            }
            false
        })
        .count() as usize
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> usize {
    part1_naive(input)
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> usize {
    part2_naive(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_part1() {
        assert_eq!(part1_naive(INPUT), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2_naive(INPUT), 4);
    }
}
