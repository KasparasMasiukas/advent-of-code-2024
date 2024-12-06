use memchr::memchr_iter;

#[aoc(day5, part1, naive)]
pub fn part1_naive(input: &str) -> u32 {
    part1(input)
}

#[aoc(day5, part2, naive)]
pub fn part2_naive(input: &str) -> usize {
    part2(input)
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> u32 {
    unsafe { impl1(input) }
}

static mut GREATER: [u8; 10000] = [0; 10000];
static mut GREATER_CNT: [u128; 100] = [0; 100];
static mut TRUE: u8 = 0;

unsafe fn impl1(input: &str) -> u32 {
    TRUE = TRUE.wrapping_add(1);
    let bytes = parse_orderings(input.as_bytes());
    let mut sum: u32 = 0;
    let mut prev_npos = 0;
    for npos in memchr_iter(b'\n', bytes) {
        sum += get_mid_if_valid(&bytes[prev_npos..npos + 1]);
        prev_npos = npos + 1;
    }
    sum
}

unsafe fn get_mid_if_valid(bytes: &[u8]) -> u32 {
    let mut line = &bytes[..];
    while line.len() >= 6 {
        if GREATER[take_two(line)] == TRUE {
            // Line invalid - skip
            return 0;
        }
        line = &line[3..];
    }
    let mid_start = bytes.len() / 2 - 1;
    take_one(&bytes[mid_start..])
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> usize {
    unsafe { impl2(input) }
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn impl2(input: &str) -> usize {
    TRUE = TRUE.wrapping_add(1);
    let mut bytes = parse_orderings_with_cnt(input.as_bytes());
    let mut sum: usize = 0;

    while bytes.len() >= 6 {
        let (mid_result, updated_bytes) = get_mid_ordered(bytes);
        sum += mid_result;
        bytes = updated_bytes;
    }

    sum
}

#[target_feature(enable = "avx2,bmi1,bmi2,cmpxchg16b,lzcnt,movbe,popcnt")]
unsafe fn get_mid_ordered(mut line: &[u8]) -> (usize, &[u8]) {
    let mut nums: [usize; 24] = [0; 24];
    let mut count = 0;

    let mut prev = take_one_usize(line);
    nums[0] = prev;
    count += 1;
    line = &line[3..];
    let mut ordered = true;

    let mut seen: u128 = 1u128 << prev;

    loop {
        let next = take_one_usize(line);
        nums[count] = next;
        count += 1;
        seen |= 1u128 << next;
        ordered &= GREATER[prev * 100 + next] != TRUE;
        prev = next;

        if *line.get_unchecked(2) == b'\n' {
            line = &line[3..];
            break;
        }
        line = &line[3..];
    }

    if ordered {
        return (0, line);
    }

    let mid = count / 2;

    // SIMD was slightly faster for u128 bitmaps
    // Below performs counting of 1-bits to determine which number has exactly half of the elements
    // ordered above it. This is only possible because the input has full ordering.
    for num in &nums {
        if (GREATER_CNT[*num] & seen).count_ones() as usize == mid {
            return (*num, line);
        }
    }

    // Should never reach here
    (0, line)
}

/// Parses orderings until no more orderings are left, returns remaining bytes.
unsafe fn parse_orderings(mut input: &[u8]) -> &[u8] {
    while input[0] != b'\n' {
        let pattern = take_two_reversed(&input);
        GREATER[pattern] = TRUE;
        input = &input[6..];
    }

    // Skip the final \n
    &input[1..]
}

#[allow(static_mut_refs)]
unsafe fn parse_orderings_with_cnt(mut input: &[u8]) -> &[u8] {
    GREATER_CNT.fill(0);
    while input[0] != b'\n' {
        let (a, b) = take_two_separate(&input);
        GREATER[b * 100 + a] = TRUE;
        GREATER_CNT[b] |= 1u128 << a;
        input = &input[6..];
    }

    // Skip the final \n
    &input[1..]
}

#[inline(always)]
unsafe fn take_one(b: &[u8]) -> u32 {
    *b.get_unchecked(0) as u32 * 10 + *b.get_unchecked(1) as u32 - 528
}

#[inline(always)]
unsafe fn take_one_usize(b: &[u8]) -> usize {
    *b.get_unchecked(0) as usize * 10 + *b.get_unchecked(1) as usize - 528
}

#[inline(always)]
unsafe fn take_two(b: &[u8]) -> usize {
    *b.get_unchecked(0) as usize * 1000
        + *b.get_unchecked(1) as usize * 100
        + *b.get_unchecked(3) as usize * 10
        + *b.get_unchecked(4) as usize
        - 53328
}

#[inline(always)]
unsafe fn take_two_separate(b: &[u8]) -> (usize, usize) {
    (
        *b.get_unchecked(0) as usize * 10 + *b.get_unchecked(1) as usize - 528,
        *b.get_unchecked(3) as usize * 10 + *b.get_unchecked(4) as usize - 528,
    )
}

#[inline(always)]
unsafe fn take_two_reversed(b: &[u8]) -> usize {
    *b.get_unchecked(3) as usize * 1000
        + *b.get_unchecked(4) as usize * 100
        + *b.get_unchecked(0) as usize * 10
        + *b.get_unchecked(1) as usize
        - 53328
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    const DAY: u8 = 5;
    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

    #[test]
    fn test_part1_naive() {
        assert_eq!(part1_naive(INPUT), 143);
    }

    #[test]
    fn test_part2_naive() {
        assert_eq!(part2_naive(INPUT), 123);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 143);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 123);
    }

    #[test]
    fn test_compare_part1_with_file() {
        let paths = [
            format!("day{DAY}.txt"),
            format!("day{DAY}-alt1.txt"),
            format!("day{DAY}-alt2.txt"),
        ];
        let outputs: [u32; 3] = [5452, 4996, 4185];
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
        let outputs: [u32; 3] = [4598, 6311, 4480];
        for (i, path) in paths.iter().enumerate() {
            let module_dir = Path::new(file!()).parent().unwrap();
            let file_path = module_dir.join(format!("../input/2024/{}", path));
            println!("Reading input file: {}", file_path.display());
            let input = fs::read_to_string(file_path).expect("Failed to read the input file");
            let expected_output = outputs[i] as usize;
            assert_eq!(part2(&input), expected_output);
        }
    }
}
