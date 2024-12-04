#[aoc(day25, part1, naive)] // FIXME: change day25 here and elsewhere to current day
pub fn part1_naive(input: &str) -> u64 {
    0
}

#[aoc(day25, part2, naive)]
pub fn part2_naive(input: &str) -> u64 {
    0
}

#[aoc(day25, part1)]
pub fn part1(input: &str) -> u64 {
    part1_naive(input)
}

#[aoc(day25, part2)]
pub fn part2(input: &str) -> u64 {
    part2_naive(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    const DAY: u8 = 0; // FIXME: change day here
    const INPUT: &str = ""; // FIXME: add example input here

    #[test]
    fn test_part1_naive() {
        assert_eq!(part1_naive(INPUT), 0);
    }

    #[test]
    fn test_part2_naive() {
        assert_eq!(part2_naive(INPUT), 0);
    }

    #[test]
    fn test_compare_part1_with_file() {
        let paths = [
            format!("day{DAY}.txt"),
            format!("day{DAY}-alt1.txt"),
            format!("day{DAY}-alt2.txt"),
        ];
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
        let paths = [
            format!("day{DAY}.txt"),
            format!("day{DAY}-alt1.txt"),
            format!("day{DAY}-alt2.txt"),
        ];
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
