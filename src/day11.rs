use std::mem::transmute;

#[aoc(day11, part1)]
pub fn part1(input: &str) -> u64 {
    unsafe { solve::<Lut1Provider, { MAX_BLINKS - 25 }>(input.as_bytes()) }
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> u64 {
    unsafe { solve::<Lut2Provider, 0>(input.as_bytes()) }
}

const DIGIT_THRESHOLD: u8 = b'0' - 1;

unsafe fn solve<P: LutProvider, const START_FROM: usize>(input: &[u8]) -> u64 {
    let mut sum = 0;
    let mut n: usize = 0;

    let mut current = input.as_ptr();
    let end = current.add(input.len());
    let lut_ptr = P::LUT.as_ptr();

    while current < end {
        if *current > DIGIT_THRESHOLD {
            n = n * 10 + (*current - b'0') as usize;
        } else {
            sum += if n < BIN_LUT_SIZE {
                *lut_ptr.add(n)
            } else {
                // Fallback to recursion
                count_stones(START_FROM, n as u64, &MINI_LUT.0)
            };
            n = 0;
        }
        current = current.add(1);
    }

    sum
}

#[inline(always)]
const fn num_digits(n: u64) -> usize {
    match n {
        0..10 => 1,
        10..100 => 2,
        100..1_000 => 3,
        1_000..10_000 => 4,
        10_000..100_000 => 5,
        100_000..1_000_000 => 6,
        1_000_000..10_000_000 => 7,
        10_000_000..100_000_000 => 8,
        100_000_000..1_000_000_000 => 9,
        1_000_000_000..10_000_000_000 => 10,
        10_000_000_000..100_000_000_000 => 11,
        100_000_000_000..1_000_000_000_000 => 12,
        1_000_000_000_000..10_000_000_000_000 => 13,
        10_000_000_000_000..100_000_000_000_000 => 14,
        100_000_000_000_000..1_000_000_000_000_000 => 15,
        1_000_000_000_000_000..10_000_000_000_000_000 => 16,
        10_000_000_000_000_000..100_000_000_000_000_000 => 17,
        _ => 18, // ..u64::MAX
    }
}

/// Get a divisor for splitting a number in half.
#[inline(always)]
const fn half_divisor(n: u64) -> u64 {
    match n {
        0..1_000 => 10,                                                // Half of 2 digits
        1_000..100_000 => 100,                                         // Half of 4 digits
        100_000..10_000_000 => 1_000,                                  // Half of 6 digits
        10_000_000..1_000_000_000 => 10_000,                           // Half of 8 digits
        1_000_000_000..100_000_000_000 => 100_000,                     // Half of 10 digits
        100_000_000_000..10_000_000_000_000 => 1_000_000,              // Half of 12 digits
        10_000_000_000_000..1_000_000_000_000_000 => 10_000_000,       // Half of 14 digits
        1_000_000_000_000_000..100_000_000_000_000_000 => 100_000_000, // Half of 16 digits
        _ => 1_000_000_000,                                            // Half of 18 digits
    }
}

#[inline(always)]
const fn split_number(n: u64) -> (u64, u64) {
    let divisor = half_divisor(n);
    let left = n / divisor;
    let right = n % divisor;
    (left, right)
}

const BIN_LUT_SIZE: usize = 100_000; // Increase to 10_000_000 for slower build but faster runtime
const MINI_LUT_SIZE: usize = 1000;
const MINI_LUT_BOUND: u64 = MINI_LUT_SIZE as u64;
const MAX_BLINKS: usize = 75;

/// `LUT[blink][stone_num]` gives the number of stones resulting from `stone_num`
/// from `blink` to `MAX_BLINKS`.
/// Mini version doesn't contain that many stone numbers, but covers all blinks.
/// This is used as a fallback if any input numbers are above 7 digits.
#[allow(long_running_const_eval)]
const MINI_LUT: AlignedMiniLUT = AlignedMiniLUT(compute_mini_lut());

#[repr(align(64))]
struct AlignedMiniLUT([[u64; MINI_LUT_SIZE]; MAX_BLINKS]);

#[allow(long_running_const_eval)]
const BIG_LUT1: AlignedBigLUT =
    unsafe { transmute(*include_bytes!(concat!(env!("OUT_DIR"), "/day11lut1.bin"))) };
#[allow(long_running_const_eval)]
const BIG_LUT2: AlignedBigLUT =
    unsafe { transmute(*include_bytes!(concat!(env!("OUT_DIR"), "/day11lut2.bin"))) };

#[repr(align(64))]
struct AlignedBigLUT([u64; BIN_LUT_SIZE]);

trait LutProvider {
    const LUT: &'static [u64; BIN_LUT_SIZE];
}

struct Lut1Provider;
struct Lut2Provider;

impl LutProvider for Lut1Provider {
    const LUT: &'static [u64; BIN_LUT_SIZE] = &BIG_LUT1.0;
}

impl LutProvider for Lut2Provider {
    const LUT: &'static [u64; BIN_LUT_SIZE] = &BIG_LUT2.0;
}

/// Resolve total number of stones, assuming the lut from the next blink is already built.
const fn count_stones(
    blink: usize,
    stone_num: u64,
    lut: &[[u64; MINI_LUT_SIZE]; MAX_BLINKS],
) -> u64 {
    if stone_num < MINI_LUT_BOUND {
        lut[blink][stone_num as usize]
    } else if blink == MAX_BLINKS - 1 {
        // Base case: Blink 74
        2 - num_digits(stone_num) as u64 % 2 // 1 stone if odd digits, 2 stones if even digits (split)
    } else {
        let digits = num_digits(stone_num);
        if digits % 2 == 0 {
            // Rule 2: Even number of digits, split into two stones
            let (left, right) = split_number(stone_num);
            count_stones(blink + 1, left, lut) + count_stones(blink + 1, right, lut)
        } else {
            // Rule 3: Odd number of digits, replace with stone_num * 2024
            count_stones(blink + 1, stone_num * 2024, lut)
        }
    }
}

const fn compute_mini_lut() -> [[u64; MINI_LUT_SIZE]; MAX_BLINKS] {
    let mut lut = [[0u64; MINI_LUT_SIZE]; MAX_BLINKS];

    // Iterate from blink 74 down to 0
    let mut blink = MAX_BLINKS - 1;
    loop {
        let current_blink = blink;

        if current_blink == MAX_BLINKS - 1 {
            // Base Case: Blink 74
            let mut stone = 0;
            while stone < MINI_LUT_SIZE {
                let digits = num_digits(stone as u64);
                if digits % 2 == 0 {
                    // Even number of digits will get split
                    lut[current_blink][stone] = 2;
                } else {
                    // Odd number of digits will simply get increased
                    lut[current_blink][stone] = 1;
                }
                stone += 1;
            }
        } else {
            // Recursive Case: Blinks 73 down to 0
            let mut stone = 0;
            while stone < MINI_LUT_SIZE {
                let stone_num = stone as u64;

                if stone_num == 0 {
                    // Rule 1: Replace 0 with 1
                    lut[current_blink][stone] = lut[current_blink + 1][1];
                } else {
                    let digits = num_digits(stone_num);
                    if digits % 2 == 0 {
                        // Rule 2: Even number of digits, split into two stones
                        let (left, right) = split_number(stone_num);
                        let mut count = 0;
                        count += lut[current_blink + 1][left as usize];
                        count += lut[current_blink + 1][right as usize];
                        lut[current_blink][stone] = count;
                    } else {
                        // Rule 3: Odd number of digits, replace with stone_num * 2024
                        let new_num = stone_num * 2024;
                        if new_num < MINI_LUT_SIZE as u64 {
                            // just in case we play around with LUT_SIZE
                            lut[current_blink][stone] = lut[current_blink + 1][new_num as usize];
                        } else {
                            lut[current_blink][stone] =
                                count_stones(current_blink + 1, new_num, &lut);
                        }
                    }
                }

                stone += 1;
            }
        }

        if blink == 0 {
            break;
        }
        blink -= 1;
    }

    lut
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    const DAY: u8 = 11;

    #[test]
    fn test_compare_part1_with_file() {
        let paths = [
            format!("day{DAY}.txt"),
            format!("day{DAY}-alt1.txt"),
            format!("day{DAY}-alt2.txt"),
        ];
        let outputs: [u64; 3] = [199946, 204022, 193269];
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
        let outputs: [u64; 3] = [237994815702032, 241651071960597, 228449040027793];
        for (i, path) in paths.iter().enumerate() {
            let module_dir = Path::new(file!()).parent().unwrap();
            let file_path = module_dir.join(format!("../input/2024/{}", path));
            println!("Reading input file: {}", file_path.display());
            let input = fs::read_to_string(file_path).expect("Failed to read the input file");
            let expected_output = outputs[i];
            assert_eq!(part2(&input), expected_output);
        }
    }

    #[test]
    fn test_compare_lut() {
        // Confirm MINI_LUT[50] == LUT1
        for i in 0..MINI_LUT_SIZE {
            assert_eq!(
                MINI_LUT.0[MAX_BLINKS - 25][i],
                BIG_LUT1.0[i],
                "Mismatch at LUT[50][{}]: {} != {}",
                i,
                MINI_LUT.0[MAX_BLINKS - 25][i],
                BIG_LUT1.0[i]
            );
        }

        // Confirm MINI_LUT[0] == LUT2
        for i in 0..MINI_LUT_SIZE {
            assert_eq!(
                MINI_LUT.0[0][i], BIG_LUT2.0[i],
                "Mismatch at LUT[0][{}]: {} != {}",
                i, MINI_LUT.0[0][i], BIG_LUT2.0[i]
            );
        }
    }
}
