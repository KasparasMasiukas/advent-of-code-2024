#[aoc(day13, part1)]
pub fn part1(input: &str) -> i32 {
    unsafe { part1_scalar_impl(input.as_bytes()) }
}

#[aoc(day13, part2)]
pub fn part2(input: &str) -> i64 {
    unsafe { part2_scalar_impl(input.as_bytes()) }
}

const NUM_EQUATIONS: usize = 320;

unsafe fn part1_scalar_impl(input: &[u8]) -> i32 {
    let mut sum: i32 = 0;
    let mut ptr = input.as_ptr();
    let mut equations_processed = 0;

    while equations_processed < NUM_EQUATIONS {
        // Extract digits for X_A
        let x_a_d1 = *ptr.add(12) as i32;
        let x_a_d2 = *ptr.add(13) as i32;
        let x_a = x_a_d1 * 10 + x_a_d2 - 528;

        // Extract digits for Y_A
        let y_a_d1 = *ptr.add(18) as i32;
        let y_a_d2 = *ptr.add(19) as i32;
        let y_a = y_a_d1 * 10 + y_a_d2 - 528;

        ptr = ptr.add(33); // Skip "Button A" line + up to the first digit of X_B

        // Extract digits for X_B
        let x_b_d1 = *ptr as i32;
        let x_b_d2 = *ptr.add(1) as i32;
        let x_b = x_b_d1 * 10 + x_b_d2 - 528;

        // Extract digits for Y_B
        let y_b_d1 = *ptr.add(6) as i32;
        let y_b_d2 = *ptr.add(7) as i32;
        let y_b = y_b_d1 * 10 + y_b_d2 - 528;

        ptr = ptr.add(18); // Skip "Button B" line and "Prize: X="

        // Parse Prize X (at least 3 digits)
        let mut x: i32 = *ptr as i32 * 100 + *ptr.add(1) as i32 * 10 + *ptr.add(2) as i32 - 5328;
        ptr = ptr.add(3);
        loop {
            let byte = *ptr;
            if byte == b',' {
                break;
            }
            x = x * 10 + ((byte - b'0') as i32);
            ptr = ptr.add(1);
        }

        // Skip ", Y=" (4 bytes)
        ptr = ptr.add(4);

        // Parse Prize Y (at least 3 digits)
        let mut y: i32 = *ptr as i32 * 100 + *ptr.add(1) as i32 * 10 + *ptr.add(2) as i32 - 5328;
        ptr = ptr.add(3);
        loop {
            let byte = *ptr;
            if byte == b'\n' {
                break;
            }
            y = y * 10 + ((byte - b'0') as i32);
            ptr = ptr.add(1);
        }

        ptr = ptr.add(2); // \n\n

        sum += compute_i32(x_a, x_b, y_a, y_b, x, y);
        equations_processed += 1;
    }

    sum
}

unsafe fn part2_scalar_impl(input: &[u8]) -> i64 {
    let mut sum: i64 = 0;
    let mut ptr = input.as_ptr();
    let mut equations_processed = 0;

    while equations_processed < NUM_EQUATIONS {
        // Extract digits for X_A
        let x_a_d1 = *ptr.add(12) as i64;
        let x_a_d2 = *ptr.add(13) as i64;
        let x_a = x_a_d1 * 10 + x_a_d2 - 528;

        // Extract digits for Y_A
        let y_a_d1 = *ptr.add(18) as i64;
        let y_a_d2 = *ptr.add(19) as i64;
        let y_a = y_a_d1 * 10 + y_a_d2 - 528;

        ptr = ptr.add(33); // Skip "Button A" line + up to the first digit of X_B

        // Extract digits for X_B
        let x_b_d1 = *ptr as i64;
        let x_b_d2 = *ptr.add(1) as i64;
        let x_b = x_b_d1 * 10 + x_b_d2 - 528;

        // Extract digits for Y_B
        let y_b_d1 = *ptr.add(6) as i64;
        let y_b_d2 = *ptr.add(7) as i64;
        let y_b = y_b_d1 * 10 + y_b_d2 - 528;

        ptr = ptr.add(18); // Skip "Button B" line and "Prize: X="

        // Parse Prize X (at least 3 digits)
        let mut x: i64 = *ptr as i64 * 100 + *ptr.add(1) as i64 * 10 + *ptr.add(2) as i64 - 5328;
        ptr = ptr.add(3);
        loop {
            let byte = *ptr;
            if byte == b',' {
                break;
            }
            x = x * 10 + ((byte - b'0') as i64);
            ptr = ptr.add(1);
        }
        x += 10000000000000;

        // Skip ", Y=" (4 bytes)
        ptr = ptr.add(4);

        // Parse Prize Y (at least 3 digits)
        let mut y: i64 = *ptr as i64 * 100 + *ptr.add(1) as i64 * 10 + *ptr.add(2) as i64 - 5328;
        ptr = ptr.add(3);
        loop {
            let byte = *ptr;
            if byte == b'\n' {
                break;
            }
            y = y * 10 + ((byte - b'0') as i64);
            ptr = ptr.add(1);
        }
        y += 10000000000000;

        ptr = ptr.add(2); // \n\n

        let c = compute_i64(x_a, x_b, y_a, y_b, x, y);
        sum += c;
        equations_processed += 1;
    }

    sum
}

#[inline(always)]
fn compute_i64(x_a: i64, x_b: i64, y_a: i64, y_b: i64, x: i64, y: i64) -> i64 {
    let delta = x_a * y_b - y_a * x_b;
    if delta != 0 {
        let a_num = (3 * x * y_b) - (3 * y * x_b);
        let b_num = (x_a * y) - (y_a * x);
        if a_num % delta == 0 && b_num % delta == 0 {
            return (a_num + b_num) / delta;
        }
    }

    0
}

#[inline(always)]
fn compute_i32(x_a: i32, x_b: i32, y_a: i32, y_b: i32, x: i32, y: i32) -> i32 {
    let delta = x_a * y_b - y_a * x_b;
    if delta != 0 {
        let a_num = (3 * x * y_b) - (3 * y * x_b);
        let b_num = (x_a * y) - (y_a * x);
        if a_num % delta == 0 && b_num % delta == 0 {
            return (a_num + b_num) / delta;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    const DAY: u8 = 13;

    #[test]
    fn test_compute() {
        // Button A: X+94, Y+34
        // Button B: X+22, Y+67
        // Prize: X=8400, Y=5400
        assert_eq!(compute_i64(94, 22, 34, 67, 8400, 5400), 280);
        // Button A: X+26, Y+66
        // Button B: X+67, Y+21
        // Prize: X=12748, Y=12176
        assert_eq!(compute_i64(26, 67, 66, 21, 12748, 12176), 0);
        // Button A: X+17, Y+86
        // Button B: X+84, Y+37
        // Prize: X=7870, Y=6450
        assert_eq!(compute_i64(17, 84, 86, 37, 7870, 6450), 200);
        // Button A: X+69, Y+23
        // Button B: X+27, Y+71
        // Prize: X=18641, Y=10279
        assert_eq!(compute_i64(69, 27, 23, 71, 18641, 10279), 0);
    }

    #[test]
    fn test_compute_u32() {
        // Button A: X+94, Y+34
        // Button B: X+22, Y+67
        // Prize: X=8400, Y=5400
        assert_eq!(compute_i32(94, 22, 34, 67, 8400, 5400), 280);
        // Button A: X+26, Y+66
        // Button B: X+67, Y+21
        // Prize: X=12748, Y=12176
        assert_eq!(compute_i32(26, 67, 66, 21, 12748, 12176), 0);
        // Button A: X+17, Y+86
        // Button B: X+84, Y+37
        // Prize: X=7870, Y=6450
        assert_eq!(compute_i32(17, 84, 86, 37, 7870, 6450), 200);
        // Button A: X+69, Y+23
        // Button B: X+27, Y+71
        // Prize: X=18641, Y=10279
        assert_eq!(compute_i32(69, 27, 23, 71, 18641, 10279), 0);
    }

    #[test]
    fn test_compare_part1_with_file() {
        let paths = [
            format!("day{DAY}.txt"),
            format!("day{DAY}-alt1.txt"),
            format!("day{DAY}-alt2.txt"),
        ];
        let outputs: [i32; 3] = [32067, 31623, 29711];
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
        let outputs: [i64; 3] = [92871736253789, 93209116744825, 94955433618919];
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
