#[aoc(day25, part1)]
pub fn part1(input: &str) -> u32 {
    let mut bytes = input.as_bytes();
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    while bytes.len() >= 35 {
        let is_lock = bytes[0] == b'#';
        let bits = bytes[6..35]
            .iter()
            .fold(0, |acc, &b| (acc << 1) | (b & 1) as u32);

        if is_lock {
            locks.push(bits);
        } else {
            keys.push(bits);
        }

        bytes = if bytes.len() >= 43 { &bytes[43..] } else { &[] };
    }

    locks
        .iter()
        .map(|l| keys.iter().filter(|k| (l & *k) == 0).count() as u32)
        .sum()
}

#[aoc(day25, part2)]
pub fn part2(_input: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    const DAY: u8 = 25;
    const INPUT: &str = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 3);
    }

    #[test]
    fn test_compare_part1_with_file() {
        let paths = [format!("day{DAY}.txt")];
        let outputs = [2933];
        for _ in 0..10 {
            for (i, path) in paths.iter().enumerate() {
                let module_dir = Path::new(file!()).parent().unwrap();
                let file_path = module_dir.join(format!("../input/2024/{}", path));
                println!("Reading input file: {}", file_path.display());
                let input = fs::read_to_string(file_path).expect("Failed to read the input file");
                let expected_output = outputs[i];
                assert_eq!(part1(&input), expected_output);
            }
        }
    }
}
