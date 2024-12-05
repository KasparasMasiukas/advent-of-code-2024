use memchr::memchr_iter;

#[aoc(day4, part1, naive)]
pub fn part1_naive(input: &str) -> usize {
    part1(input)
}

#[aoc(day4, part2, naive)]
pub fn part2_naive(input: &str) -> usize {
    part2(input)
}

// Constants based on expected input size
const GRID_SIZE: usize = 140;
const INPUT_ROWS: usize = GRID_SIZE;
const INPUT_COLS: usize = GRID_SIZE + 1; // \n
const MIN_INPUT_SIZE: usize = INPUT_ROWS * INPUT_COLS - 1; // except the final \n is optional
const D: usize = INPUT_COLS; // \n
const L: isize = -1;
const R: usize = 1;
const DL: usize = (D as isize + L) as usize;
const DR: usize = D + R;
const DOWN: (usize, usize, usize) = (D, 2 * D, 3 * D);
const DOWN_LEFT: (usize, usize, usize) = (DL, 2 * DL, 3 * DL);
const DOWN_RIGHT: (usize, usize, usize) = (DR, 2 * DR, 3 * DR);

// Use minus (-) offsets to stick to usize
const MINUS_UP: (usize, usize, usize) = (D, 2 * D, 3 * D);
const MINUS_UP_LEFT: (usize, usize, usize) = (DR, 2 * DR, 3 * DR);
const MINUS_UP_RIGHT: (usize, usize, usize) = (DL, 2 * DL, 3 * DL);

const MAS: u32 = u32::from_le_bytes(*b"MAS\0");

#[aoc(day4, part1)]
pub fn part1(input: &str) -> usize {
    let bytes = input.as_bytes();

    debug_assert!(
        bytes.len() >= MIN_INPUT_SIZE,
        "Input does not have {} bytes. It has {}",
        MIN_INPUT_SIZE,
        bytes.len()
    );

    let mut count = 0;

    for xi in memchr_iter(b'X', bytes) {
        // Consider extra byte for newlines
        let row = xi / INPUT_COLS;
        let col = xi % INPUT_COLS;
        let left_ok = col >= 3;
        let right_ok = col < GRID_SIZE - 3;
        let up_ok = row >= 3;
        let down_ok = row < GRID_SIZE - 3;

        // LEFT
        count += unsafe { (left_ok && bytes.get_unchecked(xi - 3..xi) == b"SAM") as usize };
        // RIGHT
        count += unsafe { (right_ok && bytes.get_unchecked(xi + 1..xi + 4) == b"MAS") as usize };

        // UP
        if up_ok {
            count += unsafe {
                ((*bytes.get_unchecked(xi - MINUS_UP.0) as u32
                    | ((*bytes.get_unchecked(xi - MINUS_UP.1) as u32) << 8)
                    | ((*bytes.get_unchecked(xi - MINUS_UP.2) as u32) << 16))
                    == MAS) as usize
            };

            // UP-LEFT
            if left_ok {
                count += unsafe {
                    ((*bytes.get_unchecked(xi - MINUS_UP_LEFT.0) as u32
                        | ((*bytes.get_unchecked(xi - MINUS_UP_LEFT.1) as u32) << 8)
                        | ((*bytes.get_unchecked(xi - MINUS_UP_LEFT.2) as u32) << 16))
                        == MAS) as usize
                };
            }

            // UP-RIGHT
            if right_ok {
                count += unsafe {
                    ((*bytes.get_unchecked(xi - MINUS_UP_RIGHT.0) as u32
                        | ((*bytes.get_unchecked(xi - MINUS_UP_RIGHT.1) as u32) << 8)
                        | ((*bytes.get_unchecked(xi - MINUS_UP_RIGHT.2) as u32) << 16))
                        == MAS) as usize
                };
            }
        }

        // DOWN
        if down_ok {
            count += unsafe {
                ((*bytes.get_unchecked(xi + DOWN.0) as u32
                    | ((*bytes.get_unchecked(xi + DOWN.1) as u32) << 8)
                    | ((*bytes.get_unchecked(xi + DOWN.2) as u32) << 16))
                    == MAS) as usize
            };

            // DOWN-LEFT
            if left_ok {
                count += unsafe {
                    ((*bytes.get_unchecked(xi + DOWN_LEFT.0) as u32
                        | ((*bytes.get_unchecked(xi + DOWN_LEFT.1) as u32) << 8)
                        | ((*bytes.get_unchecked(xi + DOWN_LEFT.2) as u32) << 16))
                        == MAS) as usize
                };
            }

            // DOWN-RIGHT
            if right_ok {
                count += unsafe {
                    ((*bytes.get_unchecked(xi + DOWN_RIGHT.0) as u32
                        | ((*bytes.get_unchecked(xi + DOWN_RIGHT.1) as u32) << 8)
                        | ((*bytes.get_unchecked(xi + DOWN_RIGHT.2) as u32) << 16))
                        == MAS) as usize
                };
            }
        }
    }

    count
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> usize {
    let bytes = input.as_bytes();
    let mut count = 0;
    const M_S: u8 = b'M' ^ b'S';

    // Eliminate the need for ever checking up/down boundaries
    for i in memchr_iter(b'A', &bytes[INPUT_COLS..MIN_INPUT_SIZE - INPUT_COLS - 1]) {
        let col = i % INPUT_COLS;
        if col == 0 || col == GRID_SIZE - 1 {
            continue;
        }
        let i = i + INPUT_COLS; // convert back to real index
        count += unsafe {
            (((bytes.get_unchecked(i - DR) ^ bytes.get_unchecked(i + DR))
                & (bytes.get_unchecked(i - DL) ^ bytes.get_unchecked(i + DL))
                & M_S)
                == M_S) as usize
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    const DAY: u8 = 4;
    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    #[ignore]
    fn test_part1_naive() {
        assert_eq!(part1_naive(INPUT), 18);
    }

    #[test]
    #[ignore]
    fn test_part2_naive() {
        assert_eq!(part2_naive(INPUT), 9);
    }

    /*
    // These tests require changing const GRID_SIZE to 4:
    #[test]
    fn test_left() {
        let input = "....\nSAMX\n....\n....";
        assert_eq!(part1(input), 1);
    }

    #[test]
    fn test_right() {
        let input = "....\n....\nXMAS\n....";
        assert_eq!(part1(input), 1);
    }

    #[test]
    fn test_up() {
        let input = "S...\nA...\nM...\nX...";
        assert_eq!(part1(input), 1);
    }

    #[test]
    fn test_down() {
        let input = "X...\nM...\nA...\nS...";
        assert_eq!(part1(input), 1);
    }

    #[test]
    fn test_up_right() {
        let input = "S...\n.A..\n..M.\n...X";
        assert_eq!(part1(input), 1);
    }

    #[test]
    fn test_up_left() {
        let input = "...S\n..A.\n.M..\nX...";
        assert_eq!(part1(input), 1);
    }

    #[test]
    fn test_down_right() {
        let input = "X...\n.M..\n..A.\n...S";
        assert_eq!(part1(input), 1);
    }

    #[test]
    fn test_down_left() {
        let input = "...X\n..M.\n.A..\nS...";
        assert_eq!(part1(input), 1);
    }
     */

    #[test]
    fn test_compare_part1_with_file() {
        let paths = [
            format!("day{DAY}.txt"),
            format!("day{DAY}-alt1.txt"),
            format!("day{DAY}-alt2.txt"),
        ];
        let outputs: [usize; 3] = [2454, 2500, 2583];
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
        let outputs: [usize; 3] = [1858, 1933, 1978];
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
