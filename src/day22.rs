use std::hint::unreachable_unchecked;
use std::mem::transmute;
use std::ptr;

#[aoc(day22, part1)]
pub fn part1(input: &str) -> usize {
    unsafe { part1_impl(input) }
}

#[aoc(day22, part2)]
pub fn part2(input: &str) -> u16 {
    unsafe { part2_impl(input) }
}

const MODULO: u32 = 1 << 24;
const NUM_CHANGES: usize = 2000;
const SEQ_LEN: usize = 4;
const BASE: usize = 19;
const BASE_POW_2: usize = BASE * BASE;
const BASE_POW_3: usize = BASE * BASE * BASE;
const TOTAL_SEQUENCES: usize = BASE * BASE * BASE * BASE;

static P1: [u32; MODULO as usize] = unsafe { transmute(*include_bytes!("luts/day22.bin")) };

#[inline(always)]
const fn next_secret(mut secret: usize) -> usize {
    const MOD_MASK: usize = (1 << 24) - 1;
    secret ^= (secret << 6) & MOD_MASK;
    secret ^= (secret >> 5) & MOD_MASK;
    secret ^= (secret << 11) & MOD_MASK;
    secret
}

#[inline(always)]
fn sequence_to_idx(a: usize, b: usize, c: usize, d: usize) -> usize {
    a * BASE_POW_3 + b * BASE_POW_2 + c * BASE + d
}

#[inline(always)]
unsafe fn part1_impl(input: &str) -> usize {
    let mut total_sum = 0;

    let mut ptr = input.as_ptr();
    let end = ptr.add(input.len());

    while ptr < end {
        let mut num = *ptr as usize * 100000
            + *ptr.add(1) as usize * 10000
            + *ptr.add(2) as usize * 1000
            + *ptr.add(3) as usize * 100
            + *ptr.add(4) as usize * 10
            + *ptr.add(5) as usize
            - 5333328;
        ptr = ptr.add(6);
        while *ptr != b'\n' {
            num = num * 10 + (*ptr - b'0') as usize;
            ptr = ptr.add(1);
        }
        ptr = ptr.add(1); // \n

        total_sum += *P1.get_unchecked(num) as usize;
    }

    total_sum
}

static mut SEEN: [u16; TOTAL_SEQUENCES] = [0; TOTAL_SEQUENCES];
static mut TRUE: u16 = 0;

#[inline(always)]
#[allow(static_mut_refs)]
unsafe fn part2_impl(input: &str) -> u16 {
    let mut sum_sequences = [0u16; TOTAL_SEQUENCES];
    if TRUE > u16::MAX - 2000 {
        ptr::write_bytes(SEEN.as_mut_ptr(), 0, SEEN.len());
        TRUE = 0;
    }

    let mut ptr = input.as_ptr();
    let end = ptr.add(input.len());

    while ptr < end {
        let mut num = *ptr as usize * 100000
            + *ptr.add(1) as usize * 10000
            + *ptr.add(2) as usize * 1000
            + *ptr.add(3) as usize * 100
            + *ptr.add(4) as usize * 10
            + *ptr.add(5) as usize
            - 5333328;
        ptr = ptr.add(6);
        while *ptr != b'\n' {
            num = num * 10 + (*ptr - b'0') as usize;
            ptr = ptr.add(1);
        }
        ptr = ptr.add(1); // \n

        TRUE += 1;

        let mut secret = num;
        let mut prev_price = secret % 10;

        // Sliding window
        let mut a: usize = 0;
        let mut b: usize = 0;
        let mut c: usize = 0;
        const OFFSET: usize = 9; // To handle negative changes

        for i in 0..(SEQ_LEN - 1) {
            secret = next_secret(secret);
            let price = secret % 10;

            let change = OFFSET + price - prev_price;
            prev_price = price;

            match i {
                0 => a = change,
                1 => b = change,
                2 => c = change,
                _ => unreachable_unchecked(),
            }
        }

        for _ in (SEQ_LEN - 1)..NUM_CHANGES {
            secret = next_secret(secret);
            let price = secret % 10;

            let change = OFFSET + price - prev_price;
            prev_price = price;
            let d = change;

            let seq_idx = sequence_to_idx(a, b, c, d);

            let seen = SEEN.get_unchecked_mut(seq_idx);
            if *seen < TRUE {
                *seen = TRUE;
                *sum_sequences.get_unchecked_mut(seq_idx) += price as u16;
            }

            a = b;
            b = c;
            c = d;
        }
    }

    *sum_sequences.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    const DAY: u8 = 22;

    #[test]
    fn test_compare_part1_with_file() {
        let paths = [
            format!("day{DAY}.txt"),
            format!("day{DAY}-alt1.txt"),
            format!("day{DAY}-alt2.txt"),
        ];
        let outputs = [13234715490, 14622549304, 14726157693];
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
        let outputs = [1490, 1735, 1614];
        for _ in 0..3 {
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
