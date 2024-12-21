use memchr::memchr_iter;
use std::ptr;

#[aoc(day19, part1)]
pub fn part1(input: &str) -> u64 {
    unsafe { part1_impl(input.as_bytes()) as u64 }
}

#[aoc(day19, part2)]
pub fn part2(input: &str) -> usize {
    unsafe { part2_impl(input.as_bytes()) }
}

const NODE_ID: [usize; 120] = {
    let mut lut = [0; 120];
    lut[b'w' as usize] = 1;
    lut[b'u' as usize] = 2;
    lut[b'b' as usize] = 3;
    lut[b'r' as usize] = 4;
    lut[b'g' as usize] = 5;
    lut
};

const TOTAL_PATTERNS: usize = 447;
const COUNTS_SIZE: usize = 61;
const TRIE_SIZE: usize = 5000;
const NODE_SIZE: usize = 6;
// Every node takes 6 slots: i=0: 1 if terminal, 0 otherwise; i=(1..=5) - next nodes for wubrg.
static mut TRIE: [usize; TRIE_SIZE] = [0; TRIE_SIZE];
const IS_TERMINAL: usize = 1;

const MAX_STACK_SIZE: usize = 128;
static mut STACK: [usize; MAX_STACK_SIZE] = [0; MAX_STACK_SIZE];

#[inline(always)]
#[allow(static_mut_refs)]
unsafe fn parse_trie(ptr: *mut *const u8) {
    ptr::write_bytes(TRIE.as_mut_ptr(), 0, 6);
    let mut next_empty_space = NODE_SIZE;
    for _ in 0..TOTAL_PATTERNS - 1 {
        parse_pattern::<b','>(ptr, &mut next_empty_space);
    }
    // Add final (it will end with a newline instead of comma)
    parse_pattern::<b'\n'>(ptr, &mut next_empty_space);
}

#[inline(always)]
#[allow(static_mut_refs)]
unsafe fn parse_pattern<const END_CHAR: u8>(ptr: *mut *const u8, next_empty_space: &mut usize) {
    let mut p = *ptr;
    let mut offset = 0;
    // wubrg > b',' | b'\n'
    while *p > END_CHAR {
        let i = *NODE_ID.get_unchecked(*p as usize);
        let node = TRIE.get_unchecked_mut(offset + i);
        if *node == 0 {
            // Expand
            *node = *next_empty_space;
            ptr::write_bytes(TRIE.as_mut_ptr().add(*next_empty_space), 0, 6);
            *next_empty_space += NODE_SIZE;
        }

        offset = *node;
        p = p.add(1);
    }
    *TRIE.get_unchecked_mut(offset) = IS_TERMINAL;
    *ptr = p.add(2); // b", " | b"\n\n"
}

#[allow(static_mut_refs)]
unsafe fn part1_impl(input: &[u8]) -> usize {
    let mut ptr = input.as_ptr();
    parse_trie(&mut ptr);

    let designs_start = ptr.offset_from(input.as_ptr()) as usize;
    let designs = &input[designs_start..];
    let mut possible_count = 0;
    let mut stack_size;

    let mut start_pos = 0;
    for end_pos in memchr_iter(b'\n', designs) {
        let design = &designs[start_pos..end_pos];
        let len = design.len();
        start_pos = end_pos + 1; // for next iteration

        STACK[0] = 0; // Start from root
        stack_size = 1;

        let mut visited = 0u64; // Bitmask for visited nodes
        let mut possible = 0;

        while stack_size > 0 {
            stack_size -= 1;
            let current_pos = *STACK.get_unchecked_mut(stack_size);

            if current_pos == len {
                possible = 1;
                break;
            }

            let mut offset = 0;

            let mut next = current_pos;
            while next < len {
                let idx = *NODE_ID.get_unchecked(*design.get_unchecked(next) as usize);
                offset = *TRIE.get_unchecked(offset + idx);

                if offset == 0 {
                    break;
                }

                next += 1; // Expand
                let next_mask = 1 << next;
                if *TRIE.get_unchecked(offset) > 0 && (visited & next_mask) == 0 {
                    visited |= next_mask;
                    *STACK.get_unchecked_mut(stack_size) = next;
                    stack_size += 1;
                }
            }
        }

        possible_count += possible;
    }

    possible_count
}

#[allow(static_mut_refs)]
unsafe fn part2_impl(input: &[u8]) -> usize {
    let mut ptr = input.as_ptr();
    parse_trie(&mut ptr);

    let designs_start = ptr.offset_from(input.as_ptr()) as usize;
    let designs = &input[designs_start..];

    let mut counts = [0; COUNTS_SIZE];
    let counts_reset_ptr = counts.as_mut_ptr().add(1);
    let mut total = 0;

    let mut start_pos = 0;
    for end_pos in memchr_iter(b'\n', designs) {
        let design = &designs[start_pos..end_pos];
        let len = design.len();
        start_pos = end_pos + 1; // for next iteration

        ptr::write_bytes(counts_reset_ptr, 0, len); // (1..=size+1) = 0
        counts[0] = 1; // Root

        for start in 0..len {
            let start_count = *counts.get_unchecked(start);
            if start_count > 0 {
                let mut offset = 0;

                let mut next = start;
                while next < len {
                    let idx = *NODE_ID.get_unchecked(*design.get_unchecked(next) as usize);
                    offset = *TRIE.get_unchecked(offset + idx);

                    if offset == 0 {
                        break;
                    }

                    next += 1; // Expand
                    *counts.get_unchecked_mut(next) += *TRIE.get_unchecked(offset) * start_count;
                }
            }
        }

        total += *counts.get_unchecked(len);
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    const DAY: u8 = 19;
    const INPUT: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    #[ignore]
    fn test_part1() {
        assert_eq!(part1(INPUT), 6);
    }

    #[test]
    #[ignore]
    fn test_part2() {
        assert_eq!(part2(INPUT), 16);
    }

    #[test]
    fn test_compare_part1_with_file() {
        let paths = [
            format!("day{DAY}.txt"),
            format!("day{DAY}-alt1.txt"),
            format!("day{DAY}-alt2.txt"),
        ];
        let outputs = [315, 308, 285];
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

    #[test]
    fn test_compare_part2_with_file() {
        let paths = [
            format!("day{DAY}.txt"),
            format!("day{DAY}-alt1.txt"),
            format!("day{DAY}-alt2.txt"),
        ];
        let outputs = [625108891232249, 662726441391898, 636483903099279];
        for _ in 0..10 {
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
}
