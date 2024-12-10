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

    const DAY: u8 = 25; // FIXME: change day here
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
        let outputs: [u32; 3] = [0, 0, 0];
        for (i, path) in paths.iter().enumerate() {
            let module_dir = Path::new(file!()).parent().unwrap();
            let file_path = module_dir.join(format!("../input/2024/{}", path));
            println!("Reading input file: {}", file_path.display());
            let input = fs::read_to_string(file_path).expect("Failed to read the input file");
            let expected_output = outputs[i];
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
        let outputs: [u32; 3] = [0, 0, 0];
        for (i, path) in paths.iter().enumerate() {
            let module_dir = Path::new(file!()).parent().unwrap();
            let file_path = module_dir.join(format!("../input/2024/{}", path));
            println!("Reading input file: {}", file_path.display());
            let input = fs::read_to_string(file_path).expect("Failed to read the input file");
            let expected_output = outputs[i];
            assert_eq!(part2(&input), expected_output);
        }
    }
}
