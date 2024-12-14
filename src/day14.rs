use std::cmp::Ordering;

#[aoc(day14, part1)]
pub fn part1(input: &str) -> u32 {
    unsafe { part1_impl(input.as_bytes()) }
}

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;
const X_MID: i32 = 50;
const Y_MID: i32 = 51;
const MODULO_PRODUCT: i32 = WIDTH * HEIGHT; // 10403
const X_FACTOR: i32 = HEIGHT * modular_inverse(HEIGHT, WIDTH);
const Y_FACTOR: i32 = WIDTH * modular_inverse(WIDTH, HEIGHT);
const VARIANCE_THRESHOLD: f32 = 540.0;

const NUM_ROBOTS: usize = 500;
const P1_TIMESTEPS: i32 = 100;

unsafe fn part1_impl(input: &[u8]) -> u32 {
    let mut ptr = input.as_ptr().add(2);
    let mut q1 = 0u32;
    let mut q2 = 0u32;
    let mut q3 = 0u32;
    let mut q4 = 0u32;
    for _ in 0..NUM_ROBOTS {
        let (px, py, vx, vy) = parse_line_naive(&mut ptr);

        let x_final = (px + P1_TIMESTEPS * vx).rem_euclid(WIDTH);
        let y_final = (py + P1_TIMESTEPS * vy).rem_euclid(HEIGHT);

        match (x_final.cmp(&X_MID), y_final.cmp(&Y_MID)) {
            (Ordering::Less, Ordering::Less) => q1 += 1,
            (Ordering::Less, Ordering::Greater) => q2 += 1,
            (Ordering::Greater, Ordering::Less) => q3 += 1,
            (Ordering::Greater, Ordering::Greater) => q4 += 1,
            _ => {}
        }
    }
    q1 * q2 * q3 * q4
}

#[aoc(day14, part2)]
pub fn part2(input: &str) -> i32 {
    unsafe { part2_impl(input.as_bytes()) }
}

#[inline(always)]
fn compute_variance(sum: i32, sum_sq: i32) -> f32 {
    let mean = sum as f32 / SAMPLE_SIZE;
    (sum_sq as f32 / SAMPLE_SIZE) - (mean * mean)
}

macro_rules! compute_t {
    ($func_name:ident, $data:ident, $dim:expr) => {
        #[inline(always)]
        #[allow(static_mut_refs)]
        unsafe fn $func_name() -> (i32, f32) {
            let mut t = 0;
            loop {
                t += 1;
                let mut sum: i32 = 0;
                let mut sum_sq: i32 = 0;

                for i in 0..SAMPLE_ROBOTS {
                    let (p, v) = unsafe { *$data.get_unchecked(i) };
                    let pos = (p + t * v).rem_euclid($dim);
                    sum += pos;
                    sum_sq += pos * pos;
                }

                let variance = compute_variance(sum, sum_sq);
                if variance < VARIANCE_THRESHOLD {
                    // println!("t: {}, variance: {}", t, variance);
                    return (t, variance);
                }
            }
        }
    };
}

macro_rules! compute_next_var {
    ($func_name:ident, $data:ident, $dim:expr) => {
        #[inline(always)]
        #[allow(static_mut_refs)]
        unsafe fn $func_name(input_variance: f32) -> (i32, f32) {
            let mut min_variance_t: i32 = i32::MAX;
            let mut min_variance: f32 = f32::MAX;

            for t in 0..=HEIGHT {
                let mut sum: i32 = 0;
                let mut sum_sq: i32 = 0;

                for i in 0..SAMPLE_ROBOTS {
                    let (p, v) = *$data.get_unchecked(i);
                    let pos = (p + t * v).rem_euclid($dim);
                    sum += pos;
                    sum_sq += pos * pos;
                }

                let variance = compute_variance(sum, sum_sq);
                if variance != input_variance && variance < min_variance {
                    min_variance = variance;
                    min_variance_t = t;
                }
            }

            println!(
                "{} - t: {}, variance: {}",
                stringify!($func_name),
                min_variance_t,
                min_variance
            );
            (min_variance_t, min_variance)
        }
    };
}

compute_t!(compute_t_x, X, WIDTH);
compute_t!(compute_t_y, Y, HEIGHT);

compute_next_var!(compute_next_var_x, X, WIDTH);
compute_next_var!(compute_next_var_y, Y, HEIGHT);

// Coords and velocities
const SAMPLE_DIVISOR: usize = 5;
const SAMPLE_ROBOTS: usize = NUM_ROBOTS / SAMPLE_DIVISOR;
const SAMPLE_SIZE: f32 = SAMPLE_ROBOTS as f32;
static mut X: [(i32, i32); SAMPLE_ROBOTS] = [(0, 0); SAMPLE_ROBOTS];
static mut Y: [(i32, i32); SAMPLE_ROBOTS] = [(0, 0); SAMPLE_ROBOTS];

#[allow(static_mut_refs)]
unsafe fn part2_impl(input: &[u8]) -> i32 {
    let mut ptr = input.as_ptr().add(2);
    // Take a reasonable sample of robots
    for i in 0..SAMPLE_ROBOTS {
        let (px, py, vx, vy) = parse_line_naive(&mut ptr);
        *X.get_unchecked_mut(i) = (px, vx);
        *Y.get_unchecked_mut(i) = (py, vy);
    }
    let (t_x, _var_x) = compute_t_x();
    let (t_y, _var_y) = compute_t_y();

    crt(t_x, t_y) as i32
}

#[allow(static_mut_refs)]
unsafe fn part2_analysis(input: &[u8], offset: usize) -> i32 {
    let mut ptr = input.as_ptr().add(2);
    for _ in 0..offset {
        parse_line_naive(&mut ptr);
    }
    // Take a reasonable sample of robots
    for i in 0..SAMPLE_ROBOTS {
        let (px, py, vx, vy) = parse_line_naive(&mut ptr);
        *X.get_unchecked_mut(i) = (px, vx);
        *Y.get_unchecked_mut(i) = (py, vy);
    }
    let (t_x, var_x) = compute_t_x();
    let (t_y, var_y) = compute_t_y();
    _ = compute_next_var_x(var_x);
    _ = compute_next_var_y(var_y);

    crt(t_x, t_y) as i32
}

#[allow(dead_code)]
unsafe fn visualize(input: &[u8], t: i32) {
    let mut ptr = input.as_ptr().add(2);
    let mut grid = vec![vec![' '; WIDTH as usize]; HEIGHT as usize];
    for _ in 0..NUM_ROBOTS {
        let (px, py, vx, vy) = parse_line_naive(&mut ptr);
        let x_final = (px + t * vx).rem_euclid(WIDTH);
        let y_final = (py + t * vy).rem_euclid(HEIGHT);
        grid[y_final as usize][x_final as usize] = '#';
    }
    for row in grid.iter() {
        println!("{}", row.iter().collect::<String>());
    }
}

const fn modular_inverse(a: i32, b: i32) -> i32 {
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (1, 0);

    while r != 0 {
        let quotient = old_r / r;
        (old_r, r) = (r, old_r - quotient * r);
        (old_s, s) = (s, old_s - quotient * s);
    }

    old_s.rem_euclid(b)
}

#[inline(always)]
fn crt(x: i32, y: i32) -> i32 {
    let t1 = x * X_FACTOR;
    let t2 = y * Y_FACTOR;
    (t1 + t2).rem_euclid(MODULO_PRODUCT)
}

/// Parses a single line starting just after "p=".
/// Returns a tuple (px, py, vx, vy).
/// Modifies the pointer to point to the next line just after "p=".
#[inline(always)]
unsafe fn parse_line_naive(ptr: &mut *const u8) -> (i32, i32, i32, i32) {
    let mut current = *ptr;

    // --- Parse px ---
    let mut px = (*current - b'0') as i32;
    current = current.add(1);

    let byte1_px = *current;
    if *current != b',' {
        px = px * 10 + (byte1_px - b'0') as i32;
        current = current.add(1);

        let byte2_px = *current;
        if byte2_px != b',' {
            px = px * 10 + (byte2_px - b'0') as i32;
            current = current.add(1);
        }
    }
    current = current.add(1); // ,

    // --- Parse py ---
    let mut py = (*current - b'0') as i32;
    current = current.add(1);

    let byte1_py = *current;
    if byte1_py != b' ' {
        py = py * 10 + (byte1_py - b'0') as i32;
        current = current.add(1);

        let byte2_py = *current;
        if byte2_py != b' ' {
            py = py * 10 + (byte2_py - b'0') as i32;
            current = current.add(1);
        }
    }
    current = current.add(3); // ' v='

    // --- Parse vx ---
    let mut sign_vx = 1i32;
    if *current == b'-' {
        sign_vx = -1;
        current = current.add(1);
    }

    let mut vx = (*current - b'0') as i32;
    current = current.add(1);

    let byte_vx2 = *current;
    if byte_vx2 != b',' {
        vx = vx * 10 + (byte_vx2 - b'0') as i32;
        current = current.add(1);
    }

    vx *= sign_vx;

    current = current.add(1); // ,

    // --- Parse vy ---
    let mut sign_vy = 1i32;
    if *current == b'-' {
        sign_vy = -1;
        current = current.add(1);
    }

    let mut vy = (*current - b'0') as i32;
    current = current.add(1);

    let byte_vy2 = *current;
    if byte_vy2 != b'\n' {
        vy = vy * 10 + (byte_vy2 - b'0') as i32;
        current = current.add(1);
    }

    vy *= sign_vy;

    *ptr = current.add(3); // \np=

    (px, py, vx, vy)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    const DAY: u8 = 14;
    const INPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

    const INPUT_MIN_MAX: &str = "p=0,0 v=1,1
p=100,102 v=-99,-99
p=100,102 v=-99,99
p=99,99 v=1,1
p=100,102 v=-1,-1
";

    #[test]
    fn test_parse_line() {
        let mut ptr = unsafe { INPUT.as_bytes().as_ptr().add(2) }; // Skip the first "p="
        let num_lines = INPUT.lines().count();
        let expected = [
            (0, 4, 3, -3),
            (6, 3, -1, -3),
            (10, 3, -1, 2),
            (2, 0, 2, -1),
            (0, 0, 1, 3),
            (3, 0, -2, -2),
            (7, 6, -1, -3),
            (3, 0, -1, -2),
            (9, 3, 2, 3),
            (7, 3, -1, 2),
            (2, 4, 2, -3),
            (9, 5, -3, -3),
        ];
        for i in 0..num_lines {
            let (px, py, vx, vy) = unsafe { parse_line_naive(&mut ptr) };
            assert_eq!(expected[i], (px, py, vx, vy));
        }

        let mut ptr = unsafe { INPUT_MIN_MAX.as_bytes().as_ptr().add(2) }; // Skip the first "p="
        let num_lines = INPUT_MIN_MAX.lines().count();
        let expected = [
            (0, 0, 1, 1),
            (100, 102, -99, -99),
            (100, 102, -99, 99),
            (99, 99, 1, 1),
            (100, 102, -1, -1),
        ];
        for i in 0..num_lines {
            let (px, py, vx, vy) = unsafe { parse_line_naive(&mut ptr) };
            assert_eq!(expected[i], (px, py, vx, vy));
        }
    }

    #[test]
    fn test_compare_part1_with_file() {
        let paths = [
            format!("day{DAY}.txt"),
            format!("day{DAY}-alt1.txt"),
            format!("day{DAY}-alt2.txt"),
        ];
        let outputs: [u32; 3] = [226236192, 228421332, 215987200];
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
            format!("day{DAY}-alt4.txt"),
        ];
        let outputs: [i32; 4] = [8168, 7790, 8050, 6587];
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
    fn test_visualize() {
        let paths = [
            format!("day{DAY}.txt"),
            format!("day{DAY}-alt1.txt"),
            format!("day{DAY}-alt2.txt"),
            format!("day{DAY}-alt4.txt"),
        ];
        for path in paths.iter() {
            let module_dir = Path::new(file!()).parent().unwrap();
            let file_path = module_dir.join(format!("../input/2024/{}", path));
            println!("Reading input file: {}", file_path.display());
            let input = fs::read_to_string(file_path).expect("Failed to read the input file");
            let output = unsafe { part2_impl(&input.as_bytes()) };
            unsafe { visualize(&input.as_bytes(), output) };
        }
    }

    #[test]
    fn test_analysis() {
        let paths = [
            format!("day{DAY}.txt"),
            format!("day{DAY}-alt1.txt"),
            format!("day{DAY}-alt2.txt"),
            format!("day{DAY}-alt4.txt"),
        ];
        let outputs: [i32; 4] = [8168, 7790, 8050, 6587];
        for (i, path) in paths.iter().enumerate() {
            let module_dir = Path::new(file!()).parent().unwrap();
            let file_path = module_dir.join(format!("../input/2024/{}", path));
            println!("Reading input file: {}", file_path.display());
            let input = fs::read_to_string(file_path).expect("Failed to read the input file");
            for k in 0..SAMPLE_DIVISOR {
                let offset = k * SAMPLE_ROBOTS;
                let output = unsafe { part2_analysis(&input.as_bytes(), offset) };
                assert_eq!(output, outputs[i]);
            }
        }
    }
}
