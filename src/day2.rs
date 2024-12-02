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

/// Assumptions: all numbers between 1 and 99, up to 10 numbers per line
#[aoc(day2, part1)]
pub fn part1(input: &str) -> usize {
    let mut bytes = input.as_bytes();
    let mut safe_count = 0;

    while bytes.len() > 0 {
        let mut is_valid = true;
        let mut is_increasing = None; // None: first number, Some(true): increasing, Some(false): decreasing
        let mut prev = None;
        let mut anything_found = false;

        // Iterate through the line
        while bytes.len() > 0 {
            let first_digit = bytes[0];
            bytes = &bytes[1..];

            if first_digit == b'\n' {
                break;
            }

            let number = if bytes.len() > 0 {
                let second_digit = bytes[0];
                if second_digit != b' ' && second_digit != b'\n' {
                    bytes = &bytes[1..];
                    (first_digit as i32) * 10 + (second_digit as i32) - 528
                } else {
                    (first_digit - b'0') as i32
                }
            } else {
                (first_digit - b'0') as i32
            };

            if bytes.len() > 0 && bytes[0] == b' ' {
                bytes = &bytes[1..];
            }

            if let Some(prev_num) = prev {
                let diff = number - prev_num;

                if is_increasing.is_none() {
                    // Determine the direction based on the first difference
                    if diff >= 1 && diff <= 3 {
                        is_increasing = Some(true);
                    } else if diff <= -1 && diff >= -3 {
                        is_increasing = Some(false);
                    } else {
                        is_valid = false;
                        break;
                    }
                } else {
                    if let Some(incr) = is_increasing {
                        if incr {
                            if diff < 1 || diff > 3 {
                                is_valid = false;
                                break;
                            }
                        } else {
                            if diff > -1 || diff < -3 {
                                is_valid = false;
                                break;
                            }
                        }
                    }
                }
            }

            prev = Some(number);
            anything_found = true;
        }

        if is_valid && anything_found && is_increasing.is_some() {
            safe_count += 1;
        } else {
            // Seek to the end of the line
            while bytes.len() > 0 && bytes[0] != b'\n' {
                bytes = &bytes[1..];
            }
            if bytes.len() > 0 && bytes[0] == b'\n' {
                bytes = &bytes[1..];
            }
        }
    }

    safe_count
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> usize {
    let mut safe_count = 0;
    let mut input_bytes = input.as_bytes();

    let mut numbers = [0i32; 10];

    while input_bytes.len() > 0 {
        // New line
        let mut length = 0;

        while input_bytes.len() > 0 {
            let first_digit = input_bytes[0];
            input_bytes = &input_bytes[1..];

            if first_digit == b'\n' {
                break;
            }

            let number = if input_bytes.len() > 0 {
                let second_digit = input_bytes[0];
                if second_digit != b' ' && second_digit != b'\n' {
                    input_bytes = &input_bytes[1..];
                    (first_digit as i32) * 10 + (second_digit as i32) - 528
                } else {
                    (first_digit - b'0') as i32
                }
            } else {
                (first_digit - b'0') as i32
            };

            numbers[length] = number;
            length += 1;

            if input_bytes.len() > 0 && input_bytes[0] == b' ' {
                input_bytes = &input_bytes[1..];
            }
        }

        if length == 0 {
            continue;
        }
        if is_safe_with_dampener(&numbers[..length]) {
            safe_count += 1;
        }
    }

    safe_count
}

#[inline(always)]
fn is_safe_with_dampener(levels: &[i32]) -> bool {
    can_be_made_monotonic::<1, 3>(levels) || can_be_made_monotonic::<-3, -1>(levels)
}

#[inline(always)]
fn can_be_made_monotonic<const MIN_DIFF: i32, const MAX_DIFF: i32>(levels: &[i32]) -> bool {
    let mut already_violated = false;
    let mut i = 1;
    let mut diff = levels[1] - levels[0];

    loop {
        if diff < MIN_DIFF || diff > MAX_DIFF {
            if already_violated {
                return false;
            }
            already_violated = true;

            // Decide whether to remove levels[i] or levels[i - 1]
            if i + 1 < levels.len() {
                let new_diff = levels[i + 1] - levels[i - 1];
                if new_diff >= MIN_DIFF && new_diff <= MAX_DIFF {
                    diff = new_diff; // Set up diff now with skipped levels[i]
                    i += 1;
                    continue;
                }
            } else {
                // We can always remove the final element
                return true;
            }

            if i >= 2 {
                let new_diff = levels[i] - levels[i - 2];
                if new_diff < MIN_DIFF || new_diff > MAX_DIFF {
                    // This was the last chance to make it right.
                    // Removing levels[i] or levels[i - 1] won't help to fix the violation.
                    return false;
                } // Else we can remove levels[i - 1] which won't affect the next iteration
            } // Else we can always remove the first element without any further effect
        }

        i += 1;
        if i == levels.len() {
            return true;
        }
        diff = levels[i] - levels[i - 1];
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    const INPUT: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_part1_naive() {
        assert_eq!(part1_naive(INPUT), 2);
    }

    #[test]
    fn test_part2_naive() {
        assert_eq!(part2_naive(INPUT), 4);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 4);
    }

    #[test]
    fn test_all_safe() {
        let input = "\
1 2 3 4 5
5 4 3 2 1
1 3 5 7 9
9 7 5 3 1";
        assert_eq!(part2(input), 4);
    }

    #[test]
    fn test_all_safe_with_dampener() {
        let input = "\
1 4 4
5 2 8
3 1 2
3 2 1 3";
        assert_eq!(part2(input), 4);
    }

    #[test]
    fn test_unsafe() {
        let input = "1 4 5 6 3 4";
        assert_eq!(part2(input), 0);
    }

    #[test]
    fn test_dampened() {
        let input = "14 15 17 18 17 21";
        assert_eq!(part2(input), 1);
    }

    #[test]
    fn test_dampen_first() {
        let input = "24 23 24 25 26 29 32";
        assert_eq!(part2(input), 1);
    }

    #[test]
    fn test_fake_direction() {
        let input = "5 6 4 3 2 1";
        assert_eq!(part2(input), 1);
    }

    #[test]
    fn test_monotonic_failing_match() {
        assert_eq!(part1("72 73 73 74 75"), 0);
    }

    #[test]
    fn test_compare_part1_with_file() {
        let module_dir = Path::new(file!()).parent().unwrap();
        let file_path = module_dir.join("../input/2024/day2.txt");
        println!("Reading input file: {}", file_path.display());
        let input = fs::read_to_string(file_path).expect("Failed to read the input file");

        for (index, line) in input.lines().enumerate() {
            let naive_result = part1_naive(line);
            let optimized_result = part1(line);
            if naive_result != optimized_result {
                println!("Discrepancy found at line {}: {}", index + 1, line);
                println!(
                    "Naive result: {}, Optimized result: {}",
                    naive_result, optimized_result
                );
                assert_eq!(
                    naive_result, optimized_result,
                    "Results differ for input: {}",
                    line
                );
            }
        }
    }

    #[test]
    fn test_compare_part2_with_file() {
        let module_dir = Path::new(file!()).parent().unwrap();
        let file_path = module_dir.join("../input/2024/day2.txt");
        println!("Reading input file: {}", file_path.display());
        let input = fs::read_to_string(file_path).expect("Failed to read the input file");

        for (index, line) in input.lines().enumerate() {
            let naive_result = part2_naive(line);
            let optimized_result = part2(line);
            if naive_result != optimized_result {
                println!("Discrepancy found at line {}: {}", index + 1, line);
                println!(
                    "Naive result: {}, Optimized result: {}",
                    naive_result, optimized_result
                );
                assert_eq!(
                    naive_result, optimized_result,
                    "Results differ for input: {}",
                    line
                );
            }
        }
    }
}
