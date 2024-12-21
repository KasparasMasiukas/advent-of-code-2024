use std::ptr;
use std::simd::num::SimdUint;
use std::simd::{simd_swizzle, Simd};

include!("day21_luts.rs");

#[aoc(day21, part1)]
pub fn part1(input: &str) -> u32 {
    unsafe {
        let (first, second, third, fourth, fifth) = parse_scalar(input.as_bytes());
        *P1.get_unchecked(first)
            + *P1.get_unchecked(second)
            + *P1.get_unchecked(third)
            + *P1.get_unchecked(fourth)
            + *P1.get_unchecked(fifth)
    }
}

#[aoc(day21, part2)]
pub fn part2(input: &str) -> u64 {
    unsafe {
        let (first, second, third, fourth, fifth) = parse_scalar(input.as_bytes());
        *P2.get_unchecked(first)
            + *P2.get_unchecked(second)
            + *P2.get_unchecked(third)
            + *P2.get_unchecked(fourth)
            + *P2.get_unchecked(fifth)
    }
}

#[inline(always)]
#[allow(dead_code)] // 1.6us on CodSpeed
unsafe fn parse_simd(input: &[u8]) -> Simd<u16, 8> {
    let ascii = ptr::read(input.as_ptr() as *const Simd<u8, 32>);

    const SHUFFLE_MASK: [usize; 16] = [0, 1, 2, 5, 6, 7, 10, 11, 12, 15, 16, 17, 20, 21, 22, 0];
    let digits: Simd<u8, 16> = simd_swizzle!(ascii, SHUFFLE_MASK);

    let zero = Simd::splat(b'0');
    let numbers: Simd<u16, 16> = (digits - zero).cast();

    let weights = Simd::from_array([
        100, 10, 1, 100, 10, 1, 100, 10, 1, 100, 10, 1, 100, 10, 1, 0,
    ]);

    let weighted = numbers * weights;

    let first = simd_swizzle!(weighted, [0, 3, 6, 9, 12, 0, 0, 0]);
    let second = simd_swizzle!(weighted, [1, 4, 7, 10, 13, 0, 0, 0]);
    let third = simd_swizzle!(weighted, [2, 5, 8, 11, 14, 0, 0, 0]);

    let five_nums = first + second + third;

    five_nums
}

#[inline(always)]
#[allow(dead_code)] // 1.3us on CodSpeed
unsafe fn parse_scalar(input: &[u8]) -> (usize, usize, usize, usize, usize) {
    let first = *input.get_unchecked(0) as usize * 100
        + *input.get_unchecked(1) as usize * 10
        + *input.get_unchecked(2) as usize
        - 5328;
    let second = *input.get_unchecked(5) as usize * 100
        + *input.get_unchecked(6) as usize * 10
        + *input.get_unchecked(7) as usize
        - 5328;
    let third = *input.get_unchecked(10) as usize * 100
        + *input.get_unchecked(11) as usize * 10
        + *input.get_unchecked(12) as usize
        - 5328;
    let fourth = *input.get_unchecked(15) as usize * 100
        + *input.get_unchecked(16) as usize * 10
        + *input.get_unchecked(17) as usize
        - 5328;
    let fifth = *input.get_unchecked(20) as usize * 100
        + *input.get_unchecked(21) as usize * 10
        + *input.get_unchecked(22) as usize
        - 5328;
    (first, second, third, fourth, fifth)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    const DAY: u8 = 21;

    #[test]
    fn test_compare_part1_with_file() {
        let paths = [
            format!("day{DAY}.txt"),
            format!("day{DAY}-alt1.txt"),
            format!("day{DAY}-alt2.txt"),
        ];
        let outputs = [176452, 107934, 219254];
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
        let outputs = [218309335714068, 130470079151124, 264518225304496];
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
