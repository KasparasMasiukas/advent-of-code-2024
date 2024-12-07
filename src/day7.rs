// Operator logic (inverse):
// - Addition: forward was X + Y = Z, backward: if Z >= Y, Z - Y = X
// - Multiplication: forward was X * Y = Z, backward: if Z % Y == 0, Z / Y = X
// - Concatenation: forward X | Y = (X * (10^len(Y))) + Y
//   backward: if Z ends with digits of Y, Z / (10^len(Y)) = X
//   To check ending: Z % (10^len(Y)) == Y

use memchr::memchr;
use std::hint::unreachable_unchecked;

/// Holds target and index of the last number we are currently trying to match.
/// This is to avoid recursive calls.
static mut STACK: [(usize, u64); 100] = [(0, 0); 100];
static mut STACK_PTR: usize = 0;

unsafe fn implementation<F>(input: &str, can_form_target_fn: F) -> u64
where
    F: Fn(&[u64], u64) -> bool,
{
    let mut total_sum = 0u64;

    let mut bytes = input.as_bytes();
    let mut numbers = [0u64; 16];
    let mut count: usize;

    while !bytes.is_empty() {
        // Parse target number before ':'
        let (target, offset) = parse_target(&bytes);
        bytes = &bytes[offset..];

        // Parse the sequence of numbers until newline
        count = 0;
        while bytes[0] != b'\n' {
            match parse_number(bytes) {
                Some((val, offset)) => {
                    numbers[count] = val;
                    count += 1;
                    bytes = &bytes[offset + 1..];
                }
                None => break,
            }
        }
        bytes = &bytes[1..]; // \n

        if can_form_target_fn(&numbers[..count], target) {
            total_sum += target;
        }
    }

    total_sum
}

#[aoc(day7, part1, naive)]
pub fn part1_naive(input: &str) -> u64 {
    part1(input)
}

#[aoc(day7, part1)]
pub fn part1(input: &str) -> u64 {
    unsafe { part1_impl(input) }
}

unsafe fn part1_impl(input: &str) -> u64 {
    implementation(input, |numbers, target| unsafe {
        can_form_target(numbers, target)
    })
}

#[aoc(day7, part2, naive)]
pub fn part2_naive(input: &str) -> u64 {
    part2(input)
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> u64 {
    unsafe { part2_impl(input) }
}

unsafe fn part2_impl(input: &str) -> u64 {
    implementation(input, |numbers, target| unsafe {
        can_form_target_with_concat(numbers, target)
    })
}

#[inline(always)]
#[allow(static_mut_refs)]
unsafe fn can_form_target(numbers: &[u64], target: u64) -> bool {
    STACK_PTR = 0;
    *STACK.get_unchecked_mut(STACK_PTR) = (numbers.len() - 1, target);
    STACK_PTR += 1;

    while STACK_PTR > 0 {
        STACK_PTR -= 1;
        let (current_idx, current_target) = *STACK.get_unchecked(STACK_PTR);
        let val = *numbers.get_unchecked(current_idx);

        if current_idx == 0 {
            if val == current_target {
                return true;
            }
            continue;
        }

        // Inverse Addition: target = x + val => x = target - val
        if current_target >= val {
            // Push the new state onto the stack
            *STACK.get_unchecked_mut(STACK_PTR) = (current_idx - 1, current_target - val);
            STACK_PTR += 1;
        }

        // Inverse Multiplication: target = x * val => x = target / val if divisible
        if current_target % val == 0 {
            let div = current_target / val;
            *STACK.get_unchecked_mut(STACK_PTR) = (current_idx - 1, div);
            STACK_PTR += 1;
        }
    }

    false
}

#[inline]
#[allow(static_mut_refs)]
unsafe fn can_form_target_with_concat(numbers: &[u64], target: u64) -> bool {
    STACK_PTR = 0;
    *STACK.get_unchecked_mut(STACK_PTR) = (numbers.len() - 1, target);
    STACK_PTR += 1;

    while STACK_PTR > 0 {
        STACK_PTR -= 1;
        let (current_idx, current_target) = *STACK.get_unchecked(STACK_PTR);
        let val = *numbers.get_unchecked(current_idx);

        if current_idx == 0 {
            if val == current_target {
                return true;
            }
            continue;
        }

        // Inverse Addition: target = x + val => x = target - val
        if current_target >= val {
            // Push the new state onto the stack
            *STACK.get_unchecked_mut(STACK_PTR) = (current_idx - 1, current_target - val);
            STACK_PTR += 1;
        }

        // Inverse Multiplication: target = x * val => x = target / val if divisible
        if current_target % val == 0 {
            let div = current_target / val;
            *STACK.get_unchecked_mut(STACK_PTR) = (current_idx - 1, div);
            STACK_PTR += 1;
        }

        // Inverse Concatenation: target = x * p + val => x = target / p if target % p == val
        let p = power_of_10(val);
        if current_target % p == val {
            let div = current_target / p;
            *STACK.get_unchecked_mut(STACK_PTR) = (current_idx - 1, div);
            STACK_PTR += 1;
        }
    }

    false
}

// A helper function to determine the smallest power of 10 greater than `val`,
// given that val is guaranteed to be 1 to 3 digits.
#[inline]
fn power_of_10(val: u64) -> u64 {
    if val < 10 {
        10
    } else if val < 100 {
        100
    } else {
        1000
    }
}

/// Parse the target (before :) and return offset after the target.
#[inline(always)]
unsafe fn parse_target(bytes: &[u8]) -> (u64, usize) {
    let num_digits = memchr(b':', bytes).expect("':' not found in input");

    debug_assert!(
        num_digits >= 1 && num_digits <= 20,
        "Invalid number of digits: {}",
        num_digits
    );

    let target: u64 = match num_digits {
        1 => (*bytes.get_unchecked(0) as u64) - 48, // b'0'
        2 => ((*bytes.get_unchecked(0) as u64) * 10 + (*bytes.get_unchecked(1) as u64)) - 48 * 11,
        3 => {
            (((*bytes.get_unchecked(0) as u32) * 100
                + (*bytes.get_unchecked(1) as u32) * 10
                + (*bytes.get_unchecked(2) as u32))
                - 48 * 111) as u64
        }
        4 => {
            (((*bytes.get_unchecked(0) as u32) * 1000
                + (*bytes.get_unchecked(1) as u32) * 100
                + (*bytes.get_unchecked(2) as u32) * 10
                + (*bytes.get_unchecked(3) as u32))
                - 48 * 1111) as u64
        }
        5 => {
            (((*bytes.get_unchecked(0) as u32) * 10_000
                + (*bytes.get_unchecked(1) as u32) * 1_000
                + (*bytes.get_unchecked(2) as u32) * 100
                + (*bytes.get_unchecked(3) as u32) * 10
                + (*bytes.get_unchecked(4) as u32))
                - 48 * 11_111) as u64
        }
        6 => {
            (((*bytes.get_unchecked(0) as u32) * 100_000
                + (*bytes.get_unchecked(1) as u32) * 10_000
                + (*bytes.get_unchecked(2) as u32) * 1_000
                + (*bytes.get_unchecked(3) as u32) * 100
                + (*bytes.get_unchecked(4) as u32) * 10
                + (*bytes.get_unchecked(5) as u32))
                - 48 * 111_111) as u64
        }
        7 => {
            (((*bytes.get_unchecked(0) as u32) * 1_000_000
                + (*bytes.get_unchecked(1) as u32) * 100_000
                + (*bytes.get_unchecked(2) as u32) * 10_000
                + (*bytes.get_unchecked(3) as u32) * 1_000
                + (*bytes.get_unchecked(4) as u32) * 100
                + (*bytes.get_unchecked(5) as u32) * 10
                + (*bytes.get_unchecked(6) as u32))
                - 48 * 1_111_111) as u64
        }
        8 => {
            (((*bytes.get_unchecked(0) as u32) * 10_000_000
                + (*bytes.get_unchecked(1) as u32) * 1_000_000
                + (*bytes.get_unchecked(2) as u32) * 100_000
                + (*bytes.get_unchecked(3) as u32) * 10_000
                + (*bytes.get_unchecked(4) as u32) * 1_000
                + (*bytes.get_unchecked(5) as u32) * 100
                + (*bytes.get_unchecked(6) as u32) * 10
                + (*bytes.get_unchecked(7) as u32))
                - 48 * 11_111_111) as u64
        }
        9 => {
            ((*bytes.get_unchecked(0) as u64) * 100_000_000
                + (*bytes.get_unchecked(1) as u64) * 10_000_000
                + (*bytes.get_unchecked(2) as u64) * 1_000_000
                + (*bytes.get_unchecked(3) as u64) * 100_000
                + (*bytes.get_unchecked(4) as u64) * 10_000
                + (*bytes.get_unchecked(5) as u64) * 1_000
                + (*bytes.get_unchecked(6) as u64) * 100
                + (*bytes.get_unchecked(7) as u64) * 10
                + (*bytes.get_unchecked(8) as u64))
                - 48 * 111_111_111
        }
        10 => {
            ((*bytes.get_unchecked(0) as u64) * 1_000_000_000
                + (*bytes.get_unchecked(1) as u64) * 100_000_000
                + (*bytes.get_unchecked(2) as u64) * 10_000_000
                + (*bytes.get_unchecked(3) as u64) * 1_000_000
                + (*bytes.get_unchecked(4) as u64) * 100_000
                + (*bytes.get_unchecked(5) as u64) * 10_000
                + (*bytes.get_unchecked(6) as u64) * 1_000
                + (*bytes.get_unchecked(7) as u64) * 100
                + (*bytes.get_unchecked(8) as u64) * 10
                + (*bytes.get_unchecked(9) as u64))
                - 48 * 1_111_111_111
        }
        11 => {
            ((*bytes.get_unchecked(0) as u64) * 10_000_000_000
                + (*bytes.get_unchecked(1) as u64) * 1_000_000_000
                + (*bytes.get_unchecked(2) as u64) * 100_000_000
                + (*bytes.get_unchecked(3) as u64) * 10_000_000
                + (*bytes.get_unchecked(4) as u64) * 1_000_000
                + (*bytes.get_unchecked(5) as u64) * 100_000
                + (*bytes.get_unchecked(6) as u64) * 10_000
                + (*bytes.get_unchecked(7) as u64) * 1_000
                + (*bytes.get_unchecked(8) as u64) * 100
                + (*bytes.get_unchecked(9) as u64) * 10
                + (*bytes.get_unchecked(10) as u64))
                - 48 * 11_111_111_111
        }
        12 => {
            ((*bytes.get_unchecked(0) as u64) * 100_000_000_000
                + (*bytes.get_unchecked(1) as u64) * 10_000_000_000
                + (*bytes.get_unchecked(2) as u64) * 1_000_000_000
                + (*bytes.get_unchecked(3) as u64) * 100_000_000
                + (*bytes.get_unchecked(4) as u64) * 10_000_000
                + (*bytes.get_unchecked(5) as u64) * 1_000_000
                + (*bytes.get_unchecked(6) as u64) * 100_000
                + (*bytes.get_unchecked(7) as u64) * 10_000
                + (*bytes.get_unchecked(8) as u64) * 1_000
                + (*bytes.get_unchecked(9) as u64) * 100
                + (*bytes.get_unchecked(10) as u64) * 10
                + (*bytes.get_unchecked(11) as u64))
                - 48 * 111_111_111_111
        }
        13 => {
            ((*bytes.get_unchecked(0) as u64) * 1_000_000_000_000
                + (*bytes.get_unchecked(1) as u64) * 100_000_000_000
                + (*bytes.get_unchecked(2) as u64) * 10_000_000_000
                + (*bytes.get_unchecked(3) as u64) * 1_000_000_000
                + (*bytes.get_unchecked(4) as u64) * 100_000_000
                + (*bytes.get_unchecked(5) as u64) * 10_000_000
                + (*bytes.get_unchecked(6) as u64) * 1_000_000
                + (*bytes.get_unchecked(7) as u64) * 100_000
                + (*bytes.get_unchecked(8) as u64) * 10_000
                + (*bytes.get_unchecked(9) as u64) * 1_000
                + (*bytes.get_unchecked(10) as u64) * 100
                + (*bytes.get_unchecked(11) as u64) * 10
                + (*bytes.get_unchecked(12) as u64))
                - 48 * 1_111_111_111_111
        }
        14 => {
            ((*bytes.get_unchecked(0) as u64) * 10_000_000_000_000
                + (*bytes.get_unchecked(1) as u64) * 1_000_000_000_000
                + (*bytes.get_unchecked(2) as u64) * 100_000_000_000
                + (*bytes.get_unchecked(3) as u64) * 10_000_000_000
                + (*bytes.get_unchecked(4) as u64) * 1_000_000_000
                + (*bytes.get_unchecked(5) as u64) * 100_000_000
                + (*bytes.get_unchecked(6) as u64) * 10_000_000
                + (*bytes.get_unchecked(7) as u64) * 1_000_000
                + (*bytes.get_unchecked(8) as u64) * 100_000
                + (*bytes.get_unchecked(9) as u64) * 10_000
                + (*bytes.get_unchecked(10) as u64) * 1_000
                + (*bytes.get_unchecked(11) as u64) * 100
                + (*bytes.get_unchecked(12) as u64) * 10
                + (*bytes.get_unchecked(13) as u64))
                - 48 * 11_111_111_111_111
        }
        15 => {
            ((*bytes.get_unchecked(0) as u64) * 100_000_000_000_000
                + (*bytes.get_unchecked(1) as u64) * 10_000_000_000_000
                + (*bytes.get_unchecked(2) as u64) * 1_000_000_000_000
                + (*bytes.get_unchecked(3) as u64) * 100_000_000_000
                + (*bytes.get_unchecked(4) as u64) * 10_000_000_000
                + (*bytes.get_unchecked(5) as u64) * 1_000_000_000
                + (*bytes.get_unchecked(6) as u64) * 100_000_000
                + (*bytes.get_unchecked(7) as u64) * 10_000_000
                + (*bytes.get_unchecked(8) as u64) * 1_000_000
                + (*bytes.get_unchecked(9) as u64) * 100_000
                + (*bytes.get_unchecked(10) as u64) * 10_000
                + (*bytes.get_unchecked(11) as u64) * 1_000
                + (*bytes.get_unchecked(12) as u64) * 100
                + (*bytes.get_unchecked(13) as u64) * 10
                + (*bytes.get_unchecked(14) as u64))
                - 48 * 111_111_111_111_111
        }
        16 => {
            ((*bytes.get_unchecked(0) as u64) * 1_000_000_000_000_000
                + (*bytes.get_unchecked(1) as u64) * 100_000_000_000_000
                + (*bytes.get_unchecked(2) as u64) * 10_000_000_000_000
                + (*bytes.get_unchecked(3) as u64) * 1_000_000_000_000
                + (*bytes.get_unchecked(4) as u64) * 100_000_000_000
                + (*bytes.get_unchecked(5) as u64) * 10_000_000_000
                + (*bytes.get_unchecked(6) as u64) * 1_000_000_000
                + (*bytes.get_unchecked(7) as u64) * 100_000_000
                + (*bytes.get_unchecked(8) as u64) * 10_000_000
                + (*bytes.get_unchecked(9) as u64) * 1_000_000
                + (*bytes.get_unchecked(10) as u64) * 100_000
                + (*bytes.get_unchecked(11) as u64) * 10_000
                + (*bytes.get_unchecked(12) as u64) * 1_000
                + (*bytes.get_unchecked(13) as u64) * 100
                + (*bytes.get_unchecked(14) as u64) * 10
                + (*bytes.get_unchecked(15) as u64))
                - 48 * 1_111_111_111_111_111
        }
        17 => {
            ((*bytes.get_unchecked(0) as u64) * 10_000_000_000_000_000
                + (*bytes.get_unchecked(1) as u64) * 1_000_000_000_000_000
                + (*bytes.get_unchecked(2) as u64) * 100_000_000_000_000
                + (*bytes.get_unchecked(3) as u64) * 10_000_000_000_000
                + (*bytes.get_unchecked(4) as u64) * 1_000_000_000_000
                + (*bytes.get_unchecked(5) as u64) * 100_000_000_000
                + (*bytes.get_unchecked(6) as u64) * 10_000_000_000
                + (*bytes.get_unchecked(7) as u64) * 1_000_000_000
                + (*bytes.get_unchecked(8) as u64) * 100_000_000
                + (*bytes.get_unchecked(9) as u64) * 10_000_000
                + (*bytes.get_unchecked(10) as u64) * 1_000_000
                + (*bytes.get_unchecked(11) as u64) * 100_000
                + (*bytes.get_unchecked(12) as u64) * 10_000
                + (*bytes.get_unchecked(13) as u64) * 1_000
                + (*bytes.get_unchecked(14) as u64) * 100
                + (*bytes.get_unchecked(15) as u64) * 10
                + (*bytes.get_unchecked(16) as u64))
                - 48 * 11_111_111_111_111_111
        }
        18 => {
            ((*bytes.get_unchecked(0) as u64) * 100_000_000_000_000_000
                + (*bytes.get_unchecked(1) as u64) * 10_000_000_000_000_000
                + (*bytes.get_unchecked(2) as u64) * 1_000_000_000_000_000
                + (*bytes.get_unchecked(3) as u64) * 100_000_000_000_000
                + (*bytes.get_unchecked(4) as u64) * 10_000_000_000_000
                + (*bytes.get_unchecked(5) as u64) * 1_000_000_000_000
                + (*bytes.get_unchecked(6) as u64) * 100_000_000_000
                + (*bytes.get_unchecked(7) as u64) * 10_000_000_000
                + (*bytes.get_unchecked(8) as u64) * 1_000_000_000
                + (*bytes.get_unchecked(9) as u64) * 100_000_000
                + (*bytes.get_unchecked(10) as u64) * 10_000_000
                + (*bytes.get_unchecked(11) as u64) * 1_000_000
                + (*bytes.get_unchecked(12) as u64) * 100_000
                + (*bytes.get_unchecked(13) as u64) * 10_000
                + (*bytes.get_unchecked(14) as u64) * 1_000
                + (*bytes.get_unchecked(15) as u64) * 100
                + (*bytes.get_unchecked(16) as u64) * 10
                + (*bytes.get_unchecked(17) as u64))
                - 48 * 111_111_111_111_111_111
        }
        19 => {
            ((*bytes.get_unchecked(0) as u64 - 48) * 1_000_000_000_000_000_000
                + (*bytes.get_unchecked(1) as u64) * 100_000_000_000_000_000
                + (*bytes.get_unchecked(2) as u64) * 10_000_000_000_000_000
                + (*bytes.get_unchecked(3) as u64) * 1_000_000_000_000_000
                + (*bytes.get_unchecked(4) as u64) * 100_000_000_000_000
                + (*bytes.get_unchecked(5) as u64) * 10_000_000_000_000
                + (*bytes.get_unchecked(6) as u64) * 1_000_000_000_000
                + (*bytes.get_unchecked(7) as u64) * 100_000_000_000
                + (*bytes.get_unchecked(8) as u64) * 10_000_000_000
                + (*bytes.get_unchecked(9) as u64) * 1_000_000_000
                + (*bytes.get_unchecked(10) as u64) * 100_000_000
                + (*bytes.get_unchecked(11) as u64) * 10_000_000
                + (*bytes.get_unchecked(12) as u64) * 1_000_000
                + (*bytes.get_unchecked(13) as u64) * 100_000
                + (*bytes.get_unchecked(14) as u64) * 10_000
                + (*bytes.get_unchecked(15) as u64) * 1_000
                + (*bytes.get_unchecked(16) as u64) * 100
                + (*bytes.get_unchecked(17) as u64) * 10
                + (*bytes.get_unchecked(18) as u64))
                - 48 * 111_111_111_111_111_111
        }
        20 => {
            // careful with overflow
            (*bytes.get_unchecked(0) as u64 - 48) * 10_000_000_000_000_000_000
                + (*bytes.get_unchecked(1) as u64 - 48) * 1_000_000_000_000_000_000
                + (*bytes.get_unchecked(2) as u64 - 48) * 100_000_000_000_000_000
                + (*bytes.get_unchecked(3) as u64 - 48) * 10_000_000_000_000_000
                + (*bytes.get_unchecked(4) as u64 - 48) * 1_000_000_000_000_000
                + (*bytes.get_unchecked(5) as u64 - 48) * 100_000_000_000_000
                + (*bytes.get_unchecked(6) as u64 - 48) * 10_000_000_000_000
                + (*bytes.get_unchecked(7) as u64 - 48) * 1_000_000_000_000
                + (*bytes.get_unchecked(8) as u64 - 48) * 100_000_000_000
                + (*bytes.get_unchecked(9) as u64 - 48) * 10_000_000_000
                + (*bytes.get_unchecked(10) as u64 - 48) * 1_000_000_000
                + (*bytes.get_unchecked(11) as u64 - 48) * 100_000_000
                + (*bytes.get_unchecked(12) as u64 - 48) * 10_000_000
                + (*bytes.get_unchecked(13) as u64 - 48) * 1_000_000
                + (*bytes.get_unchecked(14) as u64 - 48) * 100_000
                + (*bytes.get_unchecked(15) as u64 - 48) * 10_000
                + (*bytes.get_unchecked(16) as u64 - 48) * 1_000
                + (*bytes.get_unchecked(17) as u64 - 48) * 100
                + (*bytes.get_unchecked(18) as u64 - 48) * 10
                + (*bytes.get_unchecked(19) as u64 - 48)
        }
        _ => unreachable_unchecked(),
    };

    // Return the position after ':'
    (target, num_digits + 1)
}

/// Parses a number of equation, returns the number and the count of digits.
/// Assumes the first character is either a space or a newline.
#[inline(always)]
unsafe fn parse_number(bytes: &[u8]) -> Option<(u64, usize)> {
    match *bytes.get_unchecked(0) {
        b'\n' => None, // If the first character is a newline, return None
        b' ' => {
            // Assume the number starts at index 1
            let c2 = *bytes.get_unchecked(2);
            if c2 == b' ' || c2 == b'\n' {
                // **1 Digit Case**
                return Some(((*bytes.get_unchecked(1) as u64) - 48, 1));
            }
            let c3 = *bytes.get_unchecked(3);
            if c3 == b' ' || c3 == b'\n' {
                // **2 Digits Case**
                return Some((
                    (*bytes.get_unchecked(1) as u64) * 10 + (*bytes.get_unchecked(2) as u64)
                        - 48 * 11,
                    2,
                ));
            }
            // **3 Digits Case**
            Some((
                (*bytes.get_unchecked(1) as u64) * 100
                    + (*bytes.get_unchecked(2) as u64) * 10
                    + (*bytes.get_unchecked(3) as u64)
                    - 48 * 111,
                3,
            ))
        }
        _ => None, // If the first character is neither space nor newline, return None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    const DAY: u8 = 7;
    const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

    #[test]
    fn test_part1_naive() {
        assert_eq!(part1_naive(INPUT), 3749);
    }

    #[test]
    fn test_part2_naive() {
        assert_eq!(part2_naive(INPUT), 11387);
    }

    #[test]
    fn test_parse_target() {
        let test_cases = vec![
            ("9: ", 9, 2),
            ("98: ", 98, 3),
            ("987: ", 987, 4),
            ("9876: ", 9876, 5),
            ("98765: ", 98765, 6),
            ("987654: ", 987654, 7),
            ("9876543: ", 9876543, 8),
            ("98765432: ", 98765432, 9),
            ("987654321: ", 987654321, 10),
            ("9876543210: ", 9876543210, 11),
            ("98765432109: ", 98765432109, 12),
            ("987654321098: ", 987654321098, 13),
            ("9876543210987: ", 9876543210987, 14),
            ("98765432109876: ", 98765432109876, 15),
            ("987654321098765: ", 987654321098765, 16),
            ("9876543210987654: ", 9876543210987654, 17),
            ("98765432109876543: ", 98765432109876543, 18),
            ("987654321098765432: ", 987654321098765432, 19),
            ("9876543210987654321: ", 9876543210987654321, 20),
            ("18446744073709551615: ", 18446744073709551615, 21),
        ];

        for (input, expected_target, expected_offset) in test_cases {
            let bytes = input.as_bytes();

            unsafe {
                let (target, offset) = parse_target(bytes);

                assert_eq!(target, expected_target, "Failed on input: {}", input);
                assert_eq!(offset, expected_offset, "Failed offset on input: {}", input);
            }
        }
    }

    #[test]
    fn test_parse_number() {
        let test_cases = vec![
            // 1 Digit
            (b" 0   ", Some((0, 1))),
            (b" 1   ", Some((1, 1))),
            (b" 9   ", Some((9, 1))),
            (b" 5\n  ", Some((5, 1))),
            // 2 Digits
            (b" 10  ", Some((10, 2))),
            (b" 99  ", Some((99, 2))),
            (b" 98\n ", Some((98, 2))),
            (b" 57  ", Some((57, 2))),
            // 3 Digits
            (b" 100 ", Some((100, 3))),
            (b" 999 ", Some((999, 3))),
            (b" 987\n", Some((987, 3))),
            (b" 123 ", Some((123, 3))),
        ];

        for (input, expected) in test_cases {
            unsafe {
                let result = parse_number(input);
                assert_eq!(result, expected, "Failed on input: {:?}", input);
            }
        }

        let edge_cases = vec![
            (b"\n1 2", None),
            (b"\n 1 ", None),
            (b"\n98 ", None),
            (b"\n987", None),
        ];

        for (input, expected) in edge_cases {
            unsafe {
                let result = parse_number(input);
                assert_eq!(result, expected, "Failed on edge input: {:?}", input);
            }
        }
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 3749);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 11387);
    }

    #[test]
    fn test_compare_part1_with_file() {
        let paths = [
            format!("day{DAY}.txt"),
            format!("day{DAY}-alt1.txt"),
            format!("day{DAY}-alt2.txt"),
        ];
        let outputs: [u64; 3] = [1298300076754, 7579994664753, 1708857123053];
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
        let outputs: [u64; 3] = [248427118972289, 438027111276610, 189207836795655];
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
