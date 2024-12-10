use std::cmp::min;
use std::simd::{u8x64, Simd};

#[aoc(day9, part1)]
pub fn part1(input: &str) -> usize {
    unsafe { part1_impl(input) }
}

/// Cumulative positional adjustment term for each file size, e.g. 2 => 0+1, 3 => 0+1+2, 4 => 0+1+2+3, etc.
const SIZE_TO_POS_ADD: [usize; 10] = [0, 0, 1, 3, 6, 10, 15, 21, 28, 36];
/// ASCII digits converted to scalar digits
static mut DIGITS: [u8; 20_000] = [0; 20_000];

unsafe fn ascii_to_digits_in_place(bytes: &[u8]) {
    let zero = Simd::splat(b'0');
    let chunks = bytes.chunks_exact(64);
    let remainder = chunks.remainder();
    let mut digit_index = 0;

    for chunk in chunks {
        let ascii_bytes = u8x64::from_slice(chunk);
        let digits = ascii_bytes - zero;
        DIGITS[digit_index..digit_index + 64].copy_from_slice(&digits.to_array());
        digit_index += 64;
    }

    for byte in remainder {
        DIGITS[digit_index] = *byte - b'0';
        digit_index += 1;
    }
}

#[allow(static_mut_refs)]
unsafe fn part1_impl(input: &str) -> usize {
    ascii_to_digits_in_place(&input.as_bytes()[..input.len() - 1]);
    let digits = &DIGITS[..input.len() - 1];

    let mut left = 0;
    let mut give = 0;

    let mut right = digits.len() - 1;
    let mut take = *digits.get_unchecked(right) as usize;

    let mut pos = 0;
    let mut checksum = 0;

    while left < right {
        let size = min(give, take);
        if size > 0 {
            // Match found - transfer file pos
            let file_id = right >> 1;
            let pos_mult = pos * size + *SIZE_TO_POS_ADD.get_unchecked(size);
            checksum += file_id * pos_mult;
            pos += size;

            give -= size;
            take -= size;
        }

        if take == 0 {
            // File has been moved fully, take next in line
            right -= 2;
            take = *digits.get_unchecked(right) as usize;
        }

        if give == 0 {
            // Seek to the next free space, update checksum of jumped over file
            let size = *digits.get_unchecked(left) as usize;

            let file_id = left >> 1;
            let pos_mult = pos * size + *SIZE_TO_POS_ADD.get_unchecked(size);
            checksum += file_id * pos_mult;
            pos += size;

            give = *digits.get_unchecked(left + 1) as usize;
            left += 2;
        }
    }

    // Leftover
    if take > 0 {
        let id = right >> 1;
        let extra = pos * take + *SIZE_TO_POS_ADD.get_unchecked(take);
        checksum += id * extra;
    }

    checksum
}

/// Free space positions by size (1-9).
/// Digits seem to be uniformly distributed, so 2000 per size should be more than enough.
static mut FREE_HEAPS: [[usize; 2000]; 10] = [[0; 2000]; 10];
/// Track the number of free spaces for each size.
static mut FREE_SIZES: [usize; 10] = [0; 10];
/// Track the max free size available.
static mut MAX_FREE_SIZE_AVAILABLE: usize = 10;

#[inline(always)]
#[allow(static_mut_refs)]
unsafe fn heap_peek(heap_i: usize) -> Option<usize> {
    let size = *FREE_SIZES.get_unchecked(heap_i);
    if size == 0 {
        None
    } else {
        Some(*FREE_HEAPS.get_unchecked(heap_i).get_unchecked(0))
    }
}

#[inline(always)]
#[allow(static_mut_refs)]
unsafe fn heap_push(heap_i: usize, value: usize) {
    let size = *FREE_SIZES.get_unchecked(heap_i);
    *FREE_SIZES.get_unchecked_mut(heap_i) = size + 1;
    *FREE_HEAPS.get_unchecked_mut(heap_i).get_unchecked_mut(size) = value;

    // Bubble up
    let mut idx = size;
    while idx > 0 {
        let parent = (idx - 1) >> 1;
        let current_val = *FREE_HEAPS.get_unchecked(heap_i).get_unchecked(idx);
        let parent_val = *FREE_HEAPS.get_unchecked(heap_i).get_unchecked(parent);

        if current_val < parent_val {
            // Swap
            *FREE_HEAPS.get_unchecked_mut(heap_i).get_unchecked_mut(idx) = parent_val;
            *FREE_HEAPS
                .get_unchecked_mut(heap_i)
                .get_unchecked_mut(parent) = current_val;
            idx = parent;
        } else {
            break;
        }
    }
}

#[inline(always)]
#[allow(static_mut_refs)]
unsafe fn heap_pop(heap_i: usize) -> Option<usize> {
    let size = *FREE_SIZES.get_unchecked(heap_i);
    if size == 0 {
        return None;
    }
    let last_idx = size - 1;
    let root_val = *FREE_HEAPS.get_unchecked(heap_i).get_unchecked(0);

    // Move last to root
    *FREE_SIZES.get_unchecked_mut(heap_i) = last_idx;
    if last_idx > 0 {
        let last_val = *FREE_HEAPS.get_unchecked(heap_i).get_unchecked(last_idx);
        *FREE_HEAPS.get_unchecked_mut(heap_i).get_unchecked_mut(0) = last_val;

        // Bubble down
        let mut idx = 0;
        loop {
            let left = (idx << 1) + 1;
            let right = left + 1;
            if left >= last_idx {
                break;
            }
            let left_val = *FREE_HEAPS.get_unchecked(heap_i).get_unchecked(left);
            let mut min_idx = left;
            let mut min_val = left_val;

            if right < last_idx {
                let right_val = *FREE_HEAPS.get_unchecked(heap_i).get_unchecked(right);
                if right_val < min_val {
                    min_idx = right;
                    min_val = right_val;
                }
            }

            let current_val = *FREE_HEAPS.get_unchecked(heap_i).get_unchecked(idx);
            if min_val < current_val {
                // Swap
                *FREE_HEAPS.get_unchecked_mut(heap_i).get_unchecked_mut(idx) = min_val;
                *FREE_HEAPS
                    .get_unchecked_mut(heap_i)
                    .get_unchecked_mut(min_idx) = current_val;
                idx = min_idx;
            } else {
                break;
            }
        }
    }

    Some(root_val)
}

/// Find the next best fit for a file of size `size` starting at `pos`.
/// Returns the position and the size of the free spot
/// If no free space is found, returns the current position and 0 as the free size
#[inline(always)]
unsafe fn heap_next_fit(pos: usize, size: usize) -> (usize, usize) {
    heap_cleanup(pos);
    let mut fit_pos = pos;
    let mut free_size = 0;

    let mut i = size;
    // Check all spaces where we can fit
    while i < MAX_FREE_SIZE_AVAILABLE {
        if let Some(free_pos) = heap_peek(i) {
            if free_pos < fit_pos {
                fit_pos = free_pos;
                free_size = i;
            }
        }
        i += 1;
    }
    (fit_pos, free_size)
}

/// Cleanup the heaps that don't have any more free space in front of the current position.
#[inline(always)]
unsafe fn heap_cleanup(after_pos: usize) {
    while MAX_FREE_SIZE_AVAILABLE > 0 {
        if let Some(min_free_pos) = heap_peek(MAX_FREE_SIZE_AVAILABLE - 1) {
            if min_free_pos > after_pos {
                MAX_FREE_SIZE_AVAILABLE = MAX_FREE_SIZE_AVAILABLE - 1;
                continue;
            }
        }
        break;
    }
}

pub fn part2(input: &str) -> usize {
    unsafe { part2_impl(input) }
}

#[allow(static_mut_refs)]
pub unsafe fn part2_impl(input: &str) -> usize {
    ascii_to_digits_in_place(&input.as_bytes()[..input.len() - 1]);
    let digits = &DIGITS[..input.len() - 1];
    let mut pos = 0;
    let mut checksum = 0;

    // Setup heaps of free positions for each size
    for i in 0..10 {
        *FREE_SIZES.get_unchecked_mut(i) = 0;
    }
    MAX_FREE_SIZE_AVAILABLE = 10;
    let mut idx = 0;
    while idx + 1 < digits.len() {
        // Add file size before the free space
        pos += *digits.get_unchecked(idx) as usize;
        let size_free = *digits.get_unchecked(idx + 1) as usize;
        if size_free > 0 {
            // Push the current pos position into the corresponding free heap
            heap_push(size_free, pos);
        }
        pos += size_free;
        idx += 2;
    }
    // Add last file
    pos += *digits.get_unchecked(digits.len() - 1) as usize;

    // Now, traverse backwards to find the best free pos for each file
    let mut idx = digits.len() - 1;
    while idx > 0 {
        let size = *digits.get_unchecked(idx) as usize;
        pos -= size;

        // Find the best fit for the file. Resulting file_pos is either moved or original
        let (file_pos, free_size) = heap_next_fit(pos, size);
        let file_id = idx >> 1;
        let pos_mult = file_pos * size + *SIZE_TO_POS_ADD.get_unchecked(size);
        checksum += file_id * pos_mult;

        // Readjust the free space if anything was moved
        if free_size > 0 {
            heap_pop(free_size);
            if size < free_size {
                // Push the leftover space
                heap_push(free_size - size, file_pos + size);
            }
        }
        // Adjust the position for empty space we're jumping over
        pos -= *digits.get_unchecked(idx - 1) as usize;
        idx -= 2;
    }
    // Last file is idx == 0 and can be ignored for checksum

    checksum
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    const DAY: u8 = 9; // FIXME: change day here
    const INPUT: &str = "2333133121414131402\n"; // FIXME: add example input here

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 1928);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 2858);
    }

    #[test]
    fn test_compare_part1_with_file() {
        let paths = [
            format!("day{DAY}.txt"),
            format!("day{DAY}-alt1.txt"),
            format!("day{DAY}-alt2.txt"),
        ];
        let outputs: [usize; 3] = [6283404590840, 6607511583593, 6241633730082];
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
        let outputs: [usize; 3] = [6304576012713, 6636608781232, 6265268809555];
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
