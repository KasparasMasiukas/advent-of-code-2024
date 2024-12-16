use memchr::{memchr, memchr_iter};
use std::mem::MaybeUninit;
use std::ptr;

#[aoc(day15, part1)]
pub fn part1(input: &str) -> usize {
    unsafe { part1_impl(input.as_bytes()) }
}

// Input constraints
const GRID_LINES: usize = 50;
const INSTRUCTION_LINES: usize = 20;
const INSTRUCTIONS_PER_LINE: usize = 1000;

const LINE_LEN: usize = GRID_LINES + 1; // 50 characters + 1 newline
const PADDING: usize = LINE_LEN;
const TOTAL_GRID_SIZE: usize = PADDING * 2 + GRID_LINES * LINE_LEN; // 51 * 2 + 50 * 51 = 2652
const DIR_OFFSETS: [isize; 256] = {
    let mut lut = [0isize; 256];
    lut[b'<' as usize] = -1;
    lut[b'>' as usize] = 1;
    lut[b'^' as usize] = -(LINE_LEN as isize);
    lut[b'v' as usize] = LINE_LEN as isize;
    lut
};

// For part 2:
const LINE_LEN2: usize = LINE_LEN * 2; // 51 * 2 = 102
const PADDING2: usize = LINE_LEN2;
const TOTAL_GRID_SIZE2: usize = PADDING2 * 2 + GRID_LINES * LINE_LEN2; // 102 * 2 + 50 * 102 = 5304

const DIR_OFFSETS2: [isize; 256] = {
    let mut lut = [0isize; 256];
    lut[b'<' as usize] = -1;
    lut[b'>' as usize] = 1;
    lut[b'^' as usize] = -(LINE_LEN2 as isize);
    lut[b'v' as usize] = LINE_LEN2 as isize;
    lut
};

unsafe fn part1_impl(input: &[u8]) -> usize {
    let mut grid_uninit: MaybeUninit<[u8; TOTAL_GRID_SIZE]> = MaybeUninit::uninit();
    let grid_ptr = grid_uninit.as_mut_ptr() as *mut u8;

    let front_padding_ptr = grid_ptr;
    ptr::write_bytes(front_padding_ptr, b'#', PADDING);

    let grid_start = grid_ptr.add(PADDING);
    ptr::copy_nonoverlapping(input.as_ptr(), grid_start, GRID_LINES * LINE_LEN);

    let back_padding_ptr = grid_ptr.add(PADDING + GRID_LINES * LINE_LEN);
    ptr::write_bytes(back_padding_ptr, b'#', PADDING);

    // Convert newlines to b'#'
    for i in (PADDING + LINE_LEN - 1..PADDING + GRID_LINES * LINE_LEN).step_by(LINE_LEN) {
        *grid_ptr.add(i) = b'#';
    }

    let mut robot_pos: isize = (memchr(b'@', &input).unwrap() + PADDING) as isize;

    let grid = grid_uninit.assume_init_mut();
    let grid_mut_ptr = grid.as_mut_ptr();
    let mut instructions_ptr = input.as_ptr().add(GRID_LINES * LINE_LEN + 1);

    for _ in 0..INSTRUCTION_LINES {
        for _ in 0..INSTRUCTIONS_PER_LINE {
            let instr = *instructions_ptr;
            let offset = *DIR_OFFSETS.get_unchecked(instr as usize);
            let target_pos = robot_pos + offset;

            let mut current_pos = target_pos;
            while *grid_mut_ptr.add(current_pos as usize) == b'O' {
                current_pos += offset;
            }

            if *grid_mut_ptr.add(current_pos as usize) == b'.' {
                // Perform the shift from the first empty space back to the robot's original position
                let mut pos = current_pos;
                while pos != robot_pos {
                    let prev_pos = pos - offset;
                    *grid_mut_ptr.add(pos as usize) = *grid_mut_ptr.add(prev_pos as usize);
                    pos = prev_pos;
                }
                // Set the original robot position to empty
                *grid_mut_ptr.add(robot_pos as usize) = b'.';
                robot_pos = target_pos;
            }
            // Otherwise if it's a wall, simply discard the instruction

            instructions_ptr = instructions_ptr.add(1);
        }

        instructions_ptr = instructions_ptr.add(1);
    }

    let mut gps_sum: usize = 0;
    for pos in memchr_iter(b'O', &grid[PADDING..PADDING + GRID_LINES * LINE_LEN]) {
        let row = pos / LINE_LEN;
        let col = pos % LINE_LEN;
        gps_sum += row * 100 + col;
    }

    gps_sum
}

#[aoc(day15, part2)]
pub fn part2(input: &str) -> usize {
    unsafe { part2_impl(input.as_bytes()) }
}

#[allow(dead_code)]
fn print_grid(grid: &[u8; TOTAL_GRID_SIZE]) {
    for line_start in (0..TOTAL_GRID_SIZE).step_by(LINE_LEN) {
        let line_end = line_start + LINE_LEN;
        let line = unsafe { std::str::from_utf8_unchecked(&grid[line_start..line_end]) };
        println!("{}", line);
    }
}

#[allow(dead_code)]
fn print_stretched_grid(grid: &[u8; TOTAL_GRID_SIZE2]) {
    for line in grid.chunks_exact(LINE_LEN2) {
        let line = unsafe { std::str::from_utf8_unchecked(line) };
        println!("{}", line);
    }
}

#[allow(static_mut_refs)]
unsafe fn part2_impl(input: &[u8]) -> usize {
    let mut grid_input = &input[..GRID_LINES * LINE_LEN];

    let mut grid_uninit: MaybeUninit<[u8; TOTAL_GRID_SIZE2]> = MaybeUninit::uninit();
    let mut grid_ptr = grid_uninit.as_mut_ptr() as *mut u8;
    let start_ptr = grid_ptr;

    // Top padding
    ptr::write_bytes(grid_ptr, b'#', PADDING2);

    // Scale the map horizontally
    grid_ptr = grid_ptr.add(PADDING2);
    let mut robot_pos: isize = 0;
    while grid_input.len() > 0 {
        let line_slice = &grid_input[..LINE_LEN];
        grid_input = &grid_input[LINE_LEN..];
        for &ch in line_slice {
            match ch {
                b'#' | b'.' => {
                    *grid_ptr = ch;
                    *grid_ptr.add(1) = ch;
                    grid_ptr = grid_ptr.add(2);
                }
                b'O' => {
                    *grid_ptr = b'[';
                    *grid_ptr.add(1) = b']';
                    grid_ptr = grid_ptr.add(2);
                }
                b'@' => {
                    *grid_ptr = b'@';
                    robot_pos = grid_ptr.offset_from(start_ptr);
                    *grid_ptr.add(1) = b'.';
                    grid_ptr = grid_ptr.add(2);
                }
                _ => {
                    // Could be newlines
                    *grid_ptr = b'#';
                    *grid_ptr.add(1) = b'#';
                    grid_ptr = grid_ptr.add(2);
                }
            }
        }
    }

    // Bottom padding
    ptr::write_bytes(
        grid_ptr.add(PADDING2 + GRID_LINES * LINE_LEN2),
        b'#',
        PADDING2,
    );

    let grid = grid_uninit.assume_init_mut();
    let grid_mut_ptr = grid.as_mut_ptr();
    let mut instructions_ptr = input.as_ptr().add(GRID_LINES * LINE_LEN + 1);
    for _ in 0..INSTRUCTION_LINES {
        for _ in 0..INSTRUCTIONS_PER_LINE {
            let instr = *instructions_ptr;
            // println!("Move: {}", instr as char);
            let offset = *DIR_OFFSETS2.get_unchecked(instr as usize);
            match instr {
                b'<' | b'>' => {
                    robot_pos = move_horizontal(grid_mut_ptr, robot_pos, offset);
                }
                _ => {
                    robot_pos = move_vertical(grid_mut_ptr, robot_pos, offset);
                }
            }
            instructions_ptr = instructions_ptr.add(1);
        }
        instructions_ptr = instructions_ptr.add(1);
    }

    let mut gps_sum: usize = 0;
    let start_index = PADDING2; // start of actual map
    let end_index = PADDING2 + GRID_LINES * LINE_LEN2;
    for pos in memchr_iter(b'[', &grid[start_index..end_index]) {
        let row = pos / LINE_LEN2;
        let col = pos % LINE_LEN2;
        gps_sum += row * 100 + col;
    }

    // print_stretched_grid(grid);

    gps_sum
}

#[inline(always)]
fn is_box(ch: u8) -> bool {
    (b'['..=b']').contains(&ch)
}

unsafe fn move_horizontal(grid_ptr: *mut u8, mut robot_pos: isize, offset: isize) -> isize {
    let mut target_pos = robot_pos + offset;
    while is_box(*grid_ptr.add(target_pos as usize)) {
        target_pos += offset;
    }
    // Iff we ended in empty '.' cell, we can move
    if *grid_ptr.add(target_pos as usize) == b'.' {
        let mut pos = target_pos;
        while pos != robot_pos {
            let prev = pos - offset;
            *grid_ptr.add(pos as usize) = *grid_ptr.add(prev as usize);
            pos = prev;
        }
        *grid_ptr.add(robot_pos as usize) = b'.';
        robot_pos += offset;
    }
    robot_pos
}

static mut VISITED: [u32; TOTAL_GRID_SIZE2] = [0; TOTAL_GRID_SIZE2];
static mut TRUE: u32 = 1;
static mut STACK: [usize; 512] = [0; 512];

#[allow(static_mut_refs)]
unsafe fn move_vertical(grid_ptr: *mut u8, robot_pos: isize, offset: isize) -> isize {
    let front_pos = robot_pos + offset;
    let front_ch = *grid_ptr.add(front_pos as usize);
    if front_ch == b'.' {
        // Just move robot
        *grid_ptr.add(robot_pos as usize) = b'.';
        *grid_ptr.add(front_pos as usize) = b'@';
        return front_pos;
    }
    if !is_box(front_ch) {
        // Blocked by wall
        return robot_pos;
    }

    // Set up search
    let (box_left, box_right) = if front_ch == b'[' {
        (front_pos as usize, (front_pos + 1) as usize)
    } else {
        ((front_pos - 1) as usize, front_pos as usize)
    };
    TRUE = TRUE.wrapping_add(1);
    if TRUE == 0 {
        ptr::write_bytes(VISITED.as_mut_ptr(), 0, TOTAL_GRID_SIZE2);
        TRUE = 1;
    }
    *VISITED.get_unchecked_mut(box_left) = TRUE;
    let mut stack_size = 2;
    *STACK.get_unchecked_mut(0) = box_left;
    *STACK.get_unchecked_mut(1) = box_right;

    let mut idx = 0;
    while idx < stack_size {
        let p = *STACK.get_unchecked(idx) as isize;
        idx += 1;
        let next_p = (p + offset) as usize;
        let ch = *grid_ptr.add(next_p);
        match ch {
            b'#' => {
                return robot_pos;
            }
            b'[' | b']' => {
                // Found a box half, add both halves of this box
                let (box_left, box_right) = if ch == b'[' {
                    (next_p, next_p + 1)
                } else {
                    (next_p - 1, next_p)
                };

                let visited = VISITED.get_unchecked_mut(box_left);
                if *visited != TRUE {
                    *visited = TRUE;
                    *STACK.get_unchecked_mut(stack_size) = box_left;
                    *STACK.get_unchecked_mut(stack_size + 1) = box_right;
                    stack_size += 2;
                }
            }
            _ => {} // Empty spaces are good - search will end after all boxes meet an empty space
        }
    }

    // Pull up the boxes
    for i in (0..stack_size).rev() {
        let p = *STACK.get_unchecked(i);
        let np = (p as isize + offset) as usize;
        *grid_ptr.add(np) = *grid_ptr.add(p);
        *grid_ptr.add(p) = b'.';
    }

    // Move robot
    *grid_ptr.add(robot_pos as usize) = b'.';
    *grid_ptr.add((robot_pos + offset) as usize) = b'@'; // For visualization
    robot_pos + offset
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    const DAY: u8 = 15;

    #[test]
    fn test_compare_part1_with_file() {
        let paths = [
            format!("day{DAY}.txt"),
            format!("day{DAY}-alt1.txt"),
            format!("day{DAY}-alt2.txt"),
        ];
        let outputs = [1509074, 1515788, 1448589];
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
        let outputs = [1521453, 1516544, 1472235];
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
