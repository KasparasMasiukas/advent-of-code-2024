use std::collections::HashMap;

// Input
const CHUNK_SIZE: usize = 14;
const TOTAL_LINES: usize = 1000;
const COUNTS_SIZE: usize = 90_000;
static mut COUNTS: [u8; COUNTS_SIZE] = [0; COUNTS_SIZE];
static mut FIRST_NUMBERS: [u32; TOTAL_LINES] = [0; TOTAL_LINES];
static mut SECOND_NUMBERS: [u32; TOTAL_LINES] = [0; TOTAL_LINES];
static mut FIRST_NUMBERS_PART2: [u32; TOTAL_LINES] = [0; TOTAL_LINES];
// Generation
static mut LAST_UPDATED: [u32; COUNTS_SIZE] = [0; COUNTS_SIZE];
static mut GENERATION: u32 = 1;
// Radix
const LOWER_BITS: usize = 9;
const UPPER_BITS: usize = 8;
const FIRST_MASK: usize = (1 << LOWER_BITS) - 1; // 0x1FF
const SECOND_MASK: usize = (1 << UPPER_BITS) - 1; // 0xFF
const LOWER_SIZE: usize = 1 << LOWER_BITS; // 512
const UPPER_SIZE: usize = 1 << UPPER_BITS; // 256

#[aoc(day1, part1, naive)]
pub fn part1_naive(input: &str) -> u64 {
    let (mut list1, mut list2) = get_lists(input);
    list1.sort_unstable();
    list2.sort_unstable();
    list1
        .iter()
        .zip(list2.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum()
}

#[aoc(day1, part2, naive)]
pub fn part2_naive(input: &str) -> u64 {
    let (list1, list2) = get_lists(input);
    let occurrences = list2.iter().fold(HashMap::new(), |mut acc, &num| {
        *acc.entry(num).or_insert(0) += 1;
        acc
    });
    list1
        .iter()
        .map(|num| num * occurrences.get(num).unwrap_or(&0))
        .sum()
}

#[inline(always)]
fn radix_sort_two_pass(data: &mut [u32]) {
    let mut counts_lower: [usize; LOWER_SIZE] = [0; LOWER_SIZE];
    let mut counts_upper: [usize; UPPER_SIZE] = [0; UPPER_SIZE];
    let mut buffer: [u32; TOTAL_LINES] = [0; TOTAL_LINES];

    // 1) Lower 9 bits
    for &num in data.iter() {
        let digit = (num as usize) & FIRST_MASK;
        counts_lower[digit] += 1;
    }

    let mut cumulative = 0;
    for count in counts_lower.iter_mut() {
        let temp = *count;
        *count = cumulative;
        cumulative += temp;
    }

    for &num in data.iter() {
        let digit = (num as usize) & FIRST_MASK;
        let pos = counts_lower[digit];
        buffer[pos] = num;
        counts_lower[digit] += 1;
    }

    // 2) Upper 8 bits
    for &num in buffer.iter() {
        let digit = ((num as usize) >> LOWER_BITS) & SECOND_MASK;
        counts_upper[digit] += 1;
    }

    cumulative = 0;
    for count in counts_upper.iter_mut() {
        let temp = *count;
        *count = cumulative;
        cumulative += temp;
    }

    for &num in buffer.iter() {
        let digit = ((num as usize) >> LOWER_BITS) & SECOND_MASK;
        let pos = counts_upper[digit];
        data[pos] = num;
        counts_upper[digit] += 1;
    }
}

#[allow(static_mut_refs)]
#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    let bytes = input.as_bytes();

    let mut sum: u32 = 0;
    let mut offset = 0;

    unsafe {
        for n in 0..TOTAL_LINES {
            let num1 = parse_5_digit_number(bytes.get_unchecked(offset..offset + 5));
            let num2 = parse_5_digit_number(bytes.get_unchecked(offset + 8..offset + 13));

            *FIRST_NUMBERS.get_unchecked_mut(n) = num1;
            *SECOND_NUMBERS.get_unchecked_mut(n) = num2;

            offset += CHUNK_SIZE;
        }

        radix_sort_two_pass(&mut FIRST_NUMBERS);
        radix_sort_two_pass(&mut SECOND_NUMBERS);

        for i in 0..TOTAL_LINES {
            let a = *FIRST_NUMBERS.get_unchecked(i);
            let b = *SECOND_NUMBERS.get_unchecked(i);
            sum += a.abs_diff(b);
        }
    }

    sum
}

#[allow(static_mut_refs)]
#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    let bytes = input.as_bytes();

    let mut sum: u32 = 0;
    let mut offset = 0;

    unsafe {
        GENERATION = GENERATION.wrapping_add(1);

        for n in 0..TOTAL_LINES {
            let num1 = parse_5_digit_number(bytes.get_unchecked(offset..offset + 5));
            let num2 = parse_5_digit_number(bytes.get_unchecked(offset + 8..offset + 13));
            let index = (num2 - 10_000) as usize;

            let last_updated_ref = LAST_UPDATED.get_unchecked_mut(index);
            let counts_ref = COUNTS.get_unchecked_mut(index);

            if *last_updated_ref != GENERATION {
                *last_updated_ref = GENERATION;
                *counts_ref = 0;
            }

            *counts_ref += 1;
            *FIRST_NUMBERS_PART2.get_unchecked_mut(n) = num1;
            offset += CHUNK_SIZE;
        }

        for n in 0..TOTAL_LINES {
            let num = *FIRST_NUMBERS_PART2.get_unchecked(n);
            let index = (num - 10_000) as usize;

            if *LAST_UPDATED.get_unchecked(index) == GENERATION {
                sum += (*COUNTS.get_unchecked(index) as u32) * num;
            }
        }
    }

    sum
}

#[inline(always)]
fn parse_5_digit_number(slice: &[u8]) -> u32 {
    unsafe {
        (*slice.get_unchecked(0) as u32) * 10000
            + (*slice.get_unchecked(1) as u32) * 1000
            + (*slice.get_unchecked(2) as u32) * 100
            + (*slice.get_unchecked(3) as u32) * 10
            + (*slice.get_unchecked(4) as u32)
            - 533328 // (b'0' = 48 -> 48 * (10000 + 1000 + 100 + 10 + 1))
    }
}

fn get_lists(input: &str) -> (Vec<u64>, Vec<u64>) {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_ascii_whitespace();
            (
                parts.next().unwrap().parse::<u64>().unwrap(),
                parts.next().unwrap().parse::<u64>().unwrap(),
            )
        })
        .unzip()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    const TEST_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn part1_example() {
        assert_eq!(part1_naive(TEST_INPUT), 11);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2_naive(TEST_INPUT), 31);
    }

    #[test]
    fn test_radix_sort_random() {
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let mut data: Vec<u32> = (0..1000).map(|_| rng.gen_range(10_000..=99_999)).collect();
            let mut expected = data.clone();

            radix_sort_two_pass(&mut data);
            expected.sort_unstable();

            assert_eq!(data, expected);
        }
    }

    #[test]
    fn test_radix_sort_pre_sorted() {
        let mut data: Vec<u32> = (10_000..11_000).collect();
        let mut expected = data.clone();

        radix_sort_two_pass(&mut data);
        expected.sort_unstable();

        assert_eq!(data, expected);
    }

    #[test]
    fn test_radix_sort_reverse_sorted() {
        let mut data: Vec<u32> = (10_000..11_000).rev().collect();
        let mut expected = data.clone();

        radix_sort_two_pass(&mut data);
        expected.sort_unstable();

        assert_eq!(data, expected);
    }

    #[test]
    fn test_radix_sort_duplicates() {
        let mut data = [55_555; 1000];
        let mut expected = data.clone();

        radix_sort_two_pass(&mut data);
        expected.sort_unstable();

        assert_eq!(data, expected);
    }
}
