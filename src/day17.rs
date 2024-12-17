use std::hint::unreachable_unchecked;
use std::ops::Mul;
use std::simd::num::{SimdInt, SimdUint};
use std::simd::{simd_swizzle, Simd};
use std::str;

#[aoc(day17, part1)]
pub fn part1(input: &str) -> &'static str {
    unsafe { part1_impl(input.as_bytes()) }
}

#[aoc(day17, part2)]
pub fn part2(input: &str) -> u64 {
    unsafe { part2_impl(input.as_bytes()) }
}

const BUFFER_SIZE: usize = 64;
static mut BUFFER: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
const MASK: [usize; 16] = [0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30];
const PROGRAM_OFFSET: usize = 59;
const REG_A_OFFSET: usize = 12;
const ASCII_ADJUST: u32 = 48 * (10000000 + 1000000 + 100000 + 10000 + 1000 + 100 + 10 + 1);

#[allow(static_mut_refs)]
unsafe fn part1_impl(input: &[u8]) -> &'static str {
    let reg_a = parse_reg_a(&input[REG_A_OFFSET..]);
    let (x, y) = parse_xy_from_program(&input[PROGRAM_OFFSET..]);
    let out_size = fast_compute(reg_a, x, y);
    str::from_utf8_unchecked(&BUFFER[..out_size])
}

#[inline(always)]
fn parse_reg_a(bytes: &[u8]) -> u32 {
    Simd::from_slice(&bytes[..8])
        .cast::<i32>()
        .mul(Simd::from_array([
            10000000, 1000000, 100000, 10000, 1000, 100, 10, 1,
        ]))
        .reduce_sum() as u32
        - ASCII_ADJUST
}

// e.g. b"2,4,1,1,7,5,1,5,4,3,5,5,0,3,3,0"
#[inline(always)]
unsafe fn parse_xy_from_program(program: &[u8]) -> (u32, u32) {
    let x = (*program.get_unchecked(6) - b'0') as u32;
    let y = quick_find_y_from_program(&program[12..]);
    (x, y)
}

#[inline(always)]
unsafe fn quick_find_y_from_program(program: &[u8]) -> u32 {
    for i in (0..program.len()).step_by(4) {
        if *program.get_unchecked(i) == b'1' {
            return (*program.get_unchecked(i + 2) - b'0') as u32;
        }
    }
    unreachable_unchecked()
}

#[inline(always)]
unsafe fn parse_xy_u64(instructions: &[u8; 16]) -> (u64, u64) {
    let x = instructions[3] as u64;
    let y = quick_find_y(&instructions[6..]);
    (x, y)
}

#[inline(always)]
unsafe fn quick_find_y(instructions: &[u8]) -> u64 {
    for i in (0..instructions.len()).step_by(2) {
        if *instructions.get_unchecked(i) == 1 {
            return *instructions.get_unchecked(i + 1) as u64;
        }
    }
    unreachable_unchecked()
}

#[inline(always)]
#[allow(static_mut_refs)]
unsafe fn fast_compute(mut reg_a: u32, x: u32, y: u32) -> usize {
    let mut out_size = 0;
    while reg_a != 0 {
        let out = fast_hash_u32(reg_a, x, y);
        *BUFFER.get_unchecked_mut(out_size) = out as u8 + b'0';
        *BUFFER.get_unchecked_mut(out_size + 1) = b',';
        out_size += 2;
        reg_a >>= 3;
    }
    out_size - 1 // Get rid of the trailing comma
}

#[inline(always)]
unsafe fn fast_hash_u32(reg_a: u32, x: u32, y: u32) -> u32 {
    let b = (reg_a & 0b0111) ^ x;
    let c = reg_a >> b;
    (b ^ c ^ y) & 0b0111
}

#[inline(always)]
unsafe fn fast_hash_u64(reg_a: u64, x: u64, y: u64) -> u64 {
    let b = (reg_a & 0b0111) ^ x;
    let c = reg_a >> b;
    (b ^ c ^ y) & 0b0111
}

#[inline(always)]
unsafe fn part2_impl(input: &[u8]) -> u64 {
    let instructions = parse_instructions(&input[PROGRAM_OFFSET..]);
    let (x, y) = parse_xy_u64(&instructions);
    Recursor {
        instructions: &instructions,
        x,
        y,
    }
    .recurse(0, instructions.len() - 1)
    .unwrap_or(0)
}

#[inline(always)]
pub unsafe fn parse_instructions(input: &[u8]) -> [u8; 16] {
    let simd_input: Simd<u8, 32> = Simd::from_slice(input);
    let digits_simd = simd_input - Simd::splat(b'0');
    let parsed_digits: Simd<u8, 16> = simd_swizzle!(digits_simd, MASK);
    parsed_digits.to_array()
}

struct Recursor<'a> {
    instructions: &'a [u8],
    x: u64,
    y: u64,
}

impl<'a> Recursor<'a> {
    unsafe fn recurse(&self, current_a: u64, current_index: usize) -> Option<u64> {
        let expected_output = *self.instructions.get_unchecked(current_index) as u64;
        let reg_a_partial = current_a << 3;

        if current_index == 0 {
            for i in 0..8 {
                let reg_a = reg_a_partial | i;
                let out = fast_hash_u64(reg_a, self.x, self.y);
                if out == expected_output {
                    return Some(reg_a);
                }
            }
        } else {
            for i in 0..8 {
                let reg_a = reg_a_partial | i;
                let out = fast_hash_u64(reg_a, self.x, self.y);
                if out == expected_output {
                    if let Some(result) = self.recurse(reg_a, current_index - 1) {
                        return Some(result);
                    }
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    const DAY: u8 = 17;
    const INPUT: &str = "Register A: 38610541
Register B: 0
Register C: 0

Program: 2,4,1,1,7,5,1,5,4,3,5,5,0,3,3,0
";

    #[test]
    fn test_parse_reg_a() {
        let bytes = &INPUT.as_bytes()[REG_A_OFFSET..];
        assert_eq!(parse_reg_a(bytes), 38610541);
        let bytes = b"99999999";
        assert_eq!(parse_reg_a(bytes), 99999999);
    }

    #[test]
    fn test_program_offset() {
        let program_str = &INPUT.as_bytes()[PROGRAM_OFFSET..];
        assert_eq!(program_str, b"2,4,1,1,7,5,1,5,4,3,5,5,0,3,3,0\n");
    }

    #[test]
    fn test_parse_instructions() {
        let program_str = &INPUT.as_bytes()[PROGRAM_OFFSET..];
        let expected = [2, 4, 1, 1, 7, 5, 1, 5, 4, 3, 5, 5, 0, 3, 3, 0];
        let output = unsafe { parse_instructions(program_str) };
        assert_eq!(output, expected);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), "7,5,4,3,4,5,3,4,6");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 164278899142333);
    }

    #[test]
    fn test_compare_part1_with_file() {
        let paths = [
            format!("day{DAY}.txt"),
            format!("day{DAY}-alt1.txt"),
            format!("day{DAY}-alt2.txt"),
        ];
        let outputs = [
            "7,5,4,3,4,5,3,4,6",
            "7,4,2,0,5,0,5,3,7",
            "3,5,0,1,5,1,5,1,0",
        ];
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
        let outputs = [164278899142333, 202991746427434, 107413700225434];
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
