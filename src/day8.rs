use std::ptr;
use std::ptr::write_bytes;
use std::simd::cmp::SimdPartialOrd;
use std::simd::{u8x64, Simd};

#[aoc(day8, part1)]
pub fn part1(input: &str) -> usize {
    unsafe { part1_impl(input) }
}

/// 75 is enough to cover the range of ASCII 0-9A-Za-z if we subtract b'0' from each character.
static mut ANTENNA_COUNTS: [u8; 75] = [0; 75]; // Count of antennas per frequency adjusted to 0-74
/// Positions of up to 8 antennas per frequency (x, y).
/// Only up to 4 were observed in the input, but having extra space still fits in the cache.
static mut ANTENNAS: [[(u8, u8); 8]; 75] = [[(0, 0); 8]; 75];
/// Bitmask for antinodes at (y, x) - bit = 1 if antinode is present.
static mut ANTINODES: [u64; 50] = [0; 50];

#[allow(static_mut_refs)]
unsafe fn part1_impl(input: &str) -> usize {
    let mut line_ptr = input.as_bytes().as_ptr();

    let antenna_counts_ptr = ANTENNA_COUNTS.as_mut_ptr();
    write_bytes(antenna_counts_ptr, 0, 75);
    let antinodes_ptr = ANTINODES.as_mut_ptr();
    write_bytes(antinodes_ptr, 0, 50);
    let antennas_ptr = ANTENNAS.as_mut_ptr();

    for y in 0isize..50isize {
        let mut mask = get_line_mask(line_ptr);

        while mask != 0 {
            let x = mask.trailing_zeros() as isize;
            mask &= mask - 1;

            let c = *line_ptr.add(x as usize);
            let c_index = (c - b'0') as usize;
            let count_ptr = antenna_counts_ptr.add(c_index);
            let count = *count_ptr as usize;
            let freq_arr_ptr = antennas_ptr.add(c_index); // *mut [(u8, u8); 4]

            // Process against previously found antennas of same type
            for k in 0..count {
                let old_pos = (*freq_arr_ptr).get_unchecked(k); // (x, y)
                let old_x = old_pos.0 as isize;
                let old_y = old_pos.1 as isize;

                let dx = x - old_x;
                let dy = y - old_y;

                // From old to current: B + (B - A) = 2B - A
                {
                    let a_x = x + dx;
                    let a_y = y + dy;
                    if a_x >= 0 && a_x < 50 && a_y >= 0 && a_y < 50 {
                        *antinodes_ptr.add(a_y as usize) |= 1 << a_x;
                    }
                }

                // From current to old: A + (A - B) = 2A - B
                {
                    let a_x = old_x - dx;
                    let a_y = old_y - dy;
                    if a_x >= 0 && a_x < 50 && a_y >= 0 && a_y < 50 {
                        *antinodes_ptr.add(a_y as usize) |= 1 << a_x;
                    }
                }
            }

            // Store new antenna (x,y)
            *(*freq_arr_ptr).get_unchecked_mut(count) = (x as u8, y as u8);
            *count_ptr = (count + 1) as u8;
        }

        line_ptr = line_ptr.add(51); // Advance to next line for next iteration
    }

    // Count the bits
    let mut result = 0usize;
    for i in 0..50 {
        let row_bits = *antinodes_ptr.add(i);
        result += row_bits.count_ones() as usize;
    }

    result
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> usize {
    unsafe { part2_impl(input) }
}

const LINE_LEN: usize = 50;
const VALID_MASK: u64 = (1u64 << LINE_LEN) - 1;

/// Return bits that have antennas
#[inline(always)]
unsafe fn get_line_mask(line_ptr: *const u8) -> u64 {
    ptr::read_unaligned(line_ptr as *const u8x64)
        .simd_gt(Simd::splat(b'.'))
        .to_bitmask()
        & VALID_MASK
}

#[allow(static_mut_refs)]
unsafe fn part2_impl(input: &str) -> usize {
    let mut line_ptr = input.as_bytes().as_ptr();

    let antenna_counts_ptr = ANTENNA_COUNTS.as_mut_ptr();
    write_bytes(antenna_counts_ptr, 0, 75);
    let antinodes_ptr = ANTINODES.as_mut_ptr();
    write_bytes(antinodes_ptr, 0, 50);
    let antennas_ptr = ANTENNAS.as_mut_ptr();

    for y in 0isize..50isize {
        let mut mask = get_line_mask(line_ptr);

        while mask != 0 {
            let x = mask.trailing_zeros() as isize;
            mask &= mask - 1;

            let c = *line_ptr.add(x as usize);
            let c_index = (c - b'0') as usize;
            let count_ptr = antenna_counts_ptr.add(c_index);
            let count = *count_ptr as usize;
            let freq_arr_ptr = antennas_ptr.add(c_index); // *mut [(u8, u8); 4]

            // Process pairs (old antennas)
            for k in 0..count {
                let old_pos = (*freq_arr_ptr).get_unchecked(k); // (x, y)
                let old_x = old_pos.0 as isize;
                let old_y = old_pos.1 as isize;

                let dx = x - old_x;
                let dy = y - old_y;

                // Backward
                let mut s_x = old_x + dx;
                let mut s_y = old_y + dy;
                while s_x >= 0 && s_x < 50 && s_y >= 0 && s_y < 50 {
                    *antinodes_ptr.add(s_y as usize) |= 1 << s_x;
                    s_x += dx;
                    s_y += dy;
                }

                // Forward
                let mut s_x = x - dx;
                let mut s_y = y - dy;
                while s_x >= 0 && s_x < 50 && s_y >= 0 && s_y < 50 {
                    *antinodes_ptr.add(s_y as usize) |= 1 << s_x;
                    s_x -= dx;
                    s_y -= dy;
                }
            }

            // Store new antenna (x, y)
            *(*freq_arr_ptr).get_unchecked_mut(count) = (x as u8, y as u8);
            *count_ptr = (count + 1) as u8;
        }

        line_ptr = line_ptr.add(51); // Advance to next line for next iteration
    }

    // Count the bits
    let mut result = 0usize;
    for i in 0..50 {
        let row_bits = *antinodes_ptr.add(i);
        result += row_bits.count_ones() as usize;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    const DAY: u8 = 8;

    #[test]
    fn test_compare_part1_with_file() {
        let paths = [
            format!("day{DAY}.txt"),
            format!("day{DAY}-alt1.txt"),
            format!("day{DAY}-alt2.txt"),
        ];
        let outputs: [usize; 3] = [379, 423, 269];
        for _ in 0..10 {
            // Should work no matter how many times we re-run it
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
        let outputs: [usize; 3] = [1339, 1287, 949];
        for _ in 0..10 {
            // Should work no matter how many times we re-run it
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
