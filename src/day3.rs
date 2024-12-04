use memchr::memchr_iter;

#[aoc(day3, part1, naive)]
pub fn part1_naive(input: &str) -> u64 {
    part1(input)
}

#[aoc(day3, part2, naive)]
pub fn part2_naive(input: &str) -> u64 {
    part2(input)
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u64 {
    let mut sum: u64 = 0;
    process_all_mul(input.as_bytes(), &mut sum);
    sum
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u64 {
    let bytes = input.as_bytes();
    let n = bytes.len();
    let mut sum: u64 = 0;
    let mut pos: usize = 0;
    let mut enabled: bool = true;

    for d_pos in memchr_iter(b'd', bytes) {
        if d_pos >= n {
            break;
        }

        if enabled {
            if d_pos + 7 <= n && &bytes[d_pos..d_pos + 7] == b"don't()" {
                // Process all "mul(x,y)" before "don't()"
                process_all_mul(&bytes[pos..d_pos], &mut sum);

                pos = d_pos + 7; // "don't()"
                enabled = false;
                continue;
            }
        } else {
            if d_pos + 4 <= n && &bytes[d_pos..d_pos + 4] == b"do()" {
                pos = d_pos + 4; // "do()"
                enabled = true;
                continue;
            }
        }
    }

    // Process leftovers
    if enabled && pos < n {
        process_all_mul(&bytes[pos..n], &mut sum);
    }

    sum
}

fn process_all_mul(bytes: &[u8], sum: &mut u64) {
    let n = bytes.len();

    for m_pos in memchr_iter(b'm', bytes) {
        if m_pos + 4 > n {
            continue;
        }
        if &bytes[m_pos..m_pos + 4] != b"mul(" {
            continue;
        }

        let mut i = m_pos + 4;

        // x
        let x = match parse_number(&bytes[i..]) {
            Some((num, digits)) => {
                i += digits;
                num
            }
            None => continue,
        };

        // ,
        if i >= n || bytes[i] != b',' {
            continue;
        }
        i += 1; // Skip the comma

        // y
        let y = match parse_number(&bytes[i..]) {
            Some((num, digits)) => {
                i += digits;
                num
            }
            None => continue,
        };

        // )
        if i >= n || bytes[i] != b')' {
            continue;
        }

        *sum += x * y;
    }
}

/// Assumption: all numbers have 1-3 digits
#[inline(always)]
fn parse_number(bytes: &[u8]) -> Option<(u64, usize)> {
    match bytes {
        // 3 digits
        [b0, b1, b2, ..] if b0.is_ascii_digit() && b1.is_ascii_digit() && b2.is_ascii_digit() => {
            Some((
                (*b0 as u64) * 100 + (*b1 as u64) * 10 + (*b2 as u64) - 5328,
                3,
            ))
        }
        // 2 digits
        [b0, b1, ..] if b0.is_ascii_digit() && b1.is_ascii_digit() => {
            Some(((*b0 as u64) * 10 + (*b1 as u64) - 528, 2))
        }
        // 1 digit
        [b0, ..] if b0.is_ascii_digit() => Some(((b0 - b'0') as u64, 1)),
        // No valid number found
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_part1_naive() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part1_naive(input), 161);
    }

    #[test]
    fn test_part2_naive() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part2_naive(input), 48);
    }

    #[test]
    fn test_part1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part1(input), 161);
    }

    #[test]
    fn test_part2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part2(input), 48);
    }

    #[test]
    fn test_compare_part1_with_file() {
        let paths = ["day3.txt", "day3-alt1.txt", "day3-alt2.txt"];
        for path in paths.iter() {
            let module_dir = Path::new(file!()).parent().unwrap();
            let file_path = module_dir.join(format!("../input/2024/{}", path));
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
            // Now compare full results
            let expected_output = part1_naive(&input);
            println!("Part 1 naive output: {}", expected_output);
            assert_eq!(part1(&input), expected_output);
        }
    }

    #[test]
    fn test_compare_part2_with_file() {
        let paths = ["day3.txt", "day3-alt1.txt", "day3-alt2.txt"];
        for path in paths.iter() {
            let module_dir = Path::new(file!()).parent().unwrap();
            let file_path = module_dir.join(format!("../input/2024/{}", path));
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
            // Now compare full results
            let expected_output = part2_naive(&input);
            println!("Part 2 naive output: {}", expected_output);
            assert_eq!(part2(&input), expected_output);
        }
    }
}
