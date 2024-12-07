use std::collections::HashSet;

/// Directions represented as (delta_row, delta_col)
const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

#[aoc(day6, part1, naive)]
pub fn part1_naive(input: &str) -> usize {
    part1_impl(input).0
}

/// Implementation of part 1 that returns the number of unique visited spaces
/// and a vector of all visited positions (excluding the starting position).
fn part1_impl(input: &str) -> (usize, Vec<(usize, usize)>, usize, usize) {
    // Parse the input into a grid of characters
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let num_rows = grid.len();
    if num_rows == 0 {
        return (0, Vec::new(), 0, 0);
    }
    let num_cols = grid[0].len();

    // Find the starting position marked by '^'
    let mut start_pos = None;
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, &ch) in row.iter().enumerate() {
            if ch == '^' {
                start_pos = Some((row_idx as i32, col_idx as i32));
                break;
            }
        }
        if start_pos.is_some() {
            break;
        }
    }

    // If no starting position found, return 0 and empty Vec
    let (mut row, mut col) = match start_pos {
        Some(pos) => pos,
        None => return (0, Vec::new(), 0, 0),
    };

    // Initialize direction to up (0)
    let mut direction = 0;

    // Initialize visited set with starting position
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((row, col));

    // Initialize the Vec for visited positions, excluding starting position
    let mut visited_positions: Vec<(usize, usize)> = Vec::new();
    let mut wall_i: usize = 0;
    let mut wall_j: usize = 0;

    loop {
        // Get current direction deltas
        let (dr, dc) = DIRECTIONS[direction];

        // Compute next position
        let next_row = row + dr;
        let next_col = col + dc;

        // Check if next position is out of bounds
        if next_row < 0
            || next_row >= num_rows as i32
            || next_col < 0
            || next_col >= num_cols as i32
        {
            wall_i = direction;
            wall_j = match direction {
                0 | 2 => (col - 1) as usize,
                1 | 3 => (row - 1) as usize,
                _ => unreachable!(),
            };
            visited_positions.pop();
            break;
        }

        // Check if next position is an obstacle
        if grid[next_row as usize][next_col as usize] == '#' {
            // Turn clockwise
            direction = (direction + 1) % 4;
            continue;
        }

        // Move to next position
        row = next_row;
        col = next_col;

        // Insert the new position into the visited set
        // If the position is newly visited, add it to the Vec
        if visited.insert((row, col)) {
            visited_positions.push(((row - 1) as usize, (col - 1) as usize));
        }
    }

    // Return the number of unique visited positions and the Vec
    (visited.len(), visited_positions, wall_i, wall_j)
}

// Part 2 optimization ideas:
// pre-process each cell to find the closest obstacle in each direction for movement (no offset)
// ^ Set up a 3D array graph (130x130x4) to store the closest obstacle in each direction for each cell
// ^ After placing an obstacle, anytime we'd want to get next obstacle for movement, we have to check if column or row matches, we might get intercepted by the new obstacle instead
// instead of cloning the grid, have one static with generation counter += 4 (because we have 4 directions)//
// run the initial loop to identify viable cells that can impact the guard's movement (only <25% of all cells would be valid)
// Idea: grid could be represented as a 2D bitmask (130 bit x 130 bit). In that case, from each position we can shift left/right to find the next obstacle in each direction
// ^ for the above to work, we'd also need vertical bitmask (too bad 130 bit doesn't fit in u128) - AHA! there's no way to enter the 1st or 130th row/column without exiting the grid
// because hitting the wall redirects the guard. we only need booleans if we have walls on left, right, up, down

#[aoc(day6, part2, naive)]
pub fn part2_naive(input: &str) -> usize {
    part1(input)
}

const GRID_SIZE: usize = 130;

// Idea: directions (d) are encoded as 0: up, 1: right, 2: down, 3: left
// To get vertical/horizontal bitmask we take d % 2 as index of `grid`
// To get the wall ahead in our current direction we simply use d as index of `walls`
const UP: usize = 0;
const RIGHT: usize = 1;
const DOWN: usize = 2;
const LEFT: usize = 3;

const VERTICAL: usize = 0;
const HORIZONTAL: usize = 1;

const MIN_POS: usize = 0;
const MAX_POS: usize = GRID_SIZE - 3;

static mut VISITED: [u8; 128 * 128 * 4] = [0; 128 * 128 * 4];
// i, j, direction
// static mut VISITED: [[[u8; 128]; 128]; 4] = [[[0; 128]; 128]; 4];
static mut TRUE: u8 = 0;

#[inline(always)]
fn visited_index(i: usize, j: usize, direction: usize) -> usize {
    (i * 128 + j) * 4 + direction
}

/// Represents the parsed grid with optimized data structures.
///
/// Directions (d) are encoded as:
/// 0: Up
/// 1: Right
/// 2: Down
/// 3: Left
///
/// To get the vertical/horizontal bitmask, take `d % 2` as the index of `grid`.
/// To get the wall ahead in the current direction, use `d` as the index of `walls`.
#[derive(Debug)]
pub struct Grid {
    /// 1st index is the direction (0: up, 1: right, 2: down, 3: left)
    /// 2nd index is the column/row inner index (0 to 127):
    /// For up/down - j, for left/right - i
    pub walls: [[bool; 128]; 4],

    /// 1st index is 0: vertical, 1: horizontal
    /// 2nd index is the column/row index (0 to 127):
    /// For vertical - j, for horizontal - i
    pub grid: [[u128; 128]; 2],

    /// Starting position of the guard within the inner grid (0-based indexing)
    pub start_pos: (usize, usize),
}

impl Grid {
    /// Parses the input string into a `Grid` struct.
    ///
    /// # Arguments
    ///
    /// * `input` - A string slice representing the grid.
    ///
    /// # Panics
    ///
    /// * Panics if the input is too short.
    /// * Panics if the guard's starting position is missing or duplicated.
    /// * Panics if newline characters are missing where expected.
    pub fn parse_input(input: &str) -> Self {
        // Each line has 130 characters plus a newline, totaling 131 bytes.
        // The last line may omit the newline, so we allow up to 130 * 131 - 1 bytes.
        debug_assert!(
            input.len() >= GRID_SIZE * (GRID_SIZE + 1) - 1,
            "Input is too short. Expected at least {} bytes, found {}.",
            GRID_SIZE * (GRID_SIZE + 1) - 1,
            input.len()
        );

        // Initialize all fields
        let mut walls = [[false; 128]; 4];
        let mut grid = [[0u128; 128]; 2];
        let mut start_pos = (usize::MAX, usize::MAX); // Placeholder

        // Convert the input string to bytes for efficient indexing
        let bytes = input.as_bytes();

        for i in 0..GRID_SIZE {
            // Calculate the start and end indices for the current line
            let line_start = i * (GRID_SIZE + 1);

            // Access the current line slice
            let line = &bytes[line_start..];

            // Process top_wall (line 0) and bottom_wall (line 129)
            if i == 0 {
                // Top wall: check columns 1 to 128 (indices 1 to 128)
                for j in 1..=GRID_SIZE - 2 {
                    if line[j] == b'#' {
                        walls[UP][j - 1] = true;
                    }
                }
                continue;
            } else if i == GRID_SIZE - 1 {
                // Bottom wall: check columns 1 to 128 (indices 1 to 128)
                for j in 1..=GRID_SIZE - 2 {
                    if line[j] == b'#' {
                        walls[DOWN][j - 1] = true;
                    }
                }
                continue;
            }

            // Inner rows: 1 to 128 (0-based indexing: 1..=128)
            let inner_row = i - 1; // 0-based index for inner rows (0..127)

            // Check left_wall and right_wall for this row
            if line[0] == b'#' {
                walls[LEFT][inner_row] = true;
            }
            if line[GRID_SIZE - 1] == b'#' {
                walls[RIGHT][inner_row] = true;
            }

            // Process inner columns: 1 to 128 (indices 1 to 128)
            for j in 1..=GRID_SIZE - 2 {
                let byte = line[j];
                let col_bit = j - 1; // 0-based index for columns (0..127)

                if byte == b'#' {
                    // Set the bit for this column in lines_bitmask (horizontal)
                    grid[HORIZONTAL][inner_row] |= 1 << col_bit;

                    // Set the bit for this row in columns_bitmask (vertical)
                    grid[VERTICAL][col_bit] |= 1 << inner_row;
                } else if byte == b'^' {
                    // Record the starting position (0-based indexing)
                    if start_pos.0 != usize::MAX || start_pos.1 != usize::MAX {
                        panic!("Multiple starting positions '^' found in the grid.");
                    }
                    start_pos = (inner_row, col_bit);
                }
                // Ignore other characters ('.' or any others)
            }
        }

        // After parsing, ensure that the start_pos was found
        debug_assert!(
            start_pos.0 != usize::MAX && start_pos.1 != usize::MAX,
            "Guard's starting position '^' not found in the grid."
        );

        Grid {
            walls,
            grid,
            start_pos,
        }
    }

    /// Finds the distance to the next obstacle in the given direction from position (i, j).
    ///
    /// # Arguments
    ///
    /// * `i` - Current row index.
    /// * `j` - Current column index.
    /// * `d` - Current direction (0: up, 1: right, 2: down, 3: left).
    ///
    /// # Returns
    ///
    /// * `Option<usize>` - Some(distance) if an obstacle is found, None otherwise.
    #[inline(always)]
    fn find_next_obstacle_distance(&self, i: usize, j: usize, d: usize) -> Option<usize> {
        let bitmask_i = d % 2; // 0: vertical, 1: horizontal
        let bitmask_j = (i * bitmask_i) + (j * (1 - bitmask_i)); // i if horizontal, j if vertical
        let bitmask = self.grid[bitmask_i][bitmask_j];
        match d {
            UP => {
                if i == 0 {
                    // At the top edge, no movement possible
                    return None;
                }
                let mask = bitmask & ((1u128 << i) - 1);
                if mask != 0 {
                    let leading = mask.leading_zeros();
                    let obstacle_pos = 127 - leading as usize;
                    if obstacle_pos < i {
                        Some(i - obstacle_pos - 1)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            RIGHT => {
                if j == MAX_POS {
                    // At the right edge, no movement possible
                    return None;
                }
                let mask = (bitmask & (!0u128 << (j + 1))) >> (j + 1);
                if mask != 0 {
                    let trailing = mask.trailing_zeros();
                    let obstacle_pos = trailing as usize + j + 1;
                    if obstacle_pos <= MAX_POS {
                        Some(obstacle_pos - j - 1)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            DOWN => {
                if i == MAX_POS {
                    // At the bottom edge, no movement possible
                    return None;
                }
                let mask = (bitmask & (!0u128 << (i + 1))) >> (i + 1);
                if mask != 0 {
                    let trailing = mask.trailing_zeros();
                    let obstacle_pos = trailing as usize + i + 1;
                    if obstacle_pos <= MAX_POS {
                        Some(obstacle_pos - i - 1)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            LEFT => {
                if j == 0 {
                    // At the left edge, no movement possible
                    return None;
                }
                let mask = bitmask & ((1u128 << j) - 1);
                if mask != 0 {
                    let leading = mask.leading_zeros();
                    let obstacle_pos = 127 - leading as usize;
                    if obstacle_pos < j {
                        Some(j - obstacle_pos - 1)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => unreachable!(), // Directions are encoded from 0 to 3
        }
    }

    #[allow(static_mut_refs)]
    pub unsafe fn is_looping(&self) -> (bool, usize) {
        let old_true = TRUE;
        TRUE = TRUE.wrapping_add(1);
        if old_true < TRUE {
            // wrapped - can't use old values
            VISITED.fill(0);
            TRUE = 1;
        }
        let (mut i, mut j) = self.start_pos;
        let mut d = 0; // Direction: 0: up, 1: right, 2: down, 3: left
        let mut loop_length: usize = 0;

        loop {
            loop_length += 1;
            let idx = visited_index(i, j, d);
            if VISITED[idx] == TRUE {
                // Loop detected
                return (true, loop_length);
            }
            VISITED[idx] = TRUE;

            let horizontal = d % 2; // 0: vertical, 1: horizontal
            let line_index = (i * horizontal) + (j * (1 - horizontal)); // i if horizontal, j if vertical

            // From line, get the next obstacle in the current direction
            // To do that, shift the bits left or right depending on the direction,
            // then find the most/least significant bit set (1) depending on the direction:
            // then count leading/trailing zeros to get the next obstacle in the current direction
            // if no obstacle in the current direction, AND if wall_ahead is false, return false (guard exits the grid)
            // otherwise based on direction, adjust the i/j to just before the next obstacle (or the wall ahead)
            let distance = self.find_next_obstacle_distance(i, j, d);

            if let Some(distance) = distance {
                match d {
                    UP => {
                        i -= distance;
                    }
                    RIGHT => {
                        j += distance;
                    }
                    DOWN => {
                        i += distance;
                    }
                    LEFT => {
                        j -= distance;
                    }
                    _ => unreachable!(),
                }
                d = (d + 1) % 4; // Turn clockwise
            } else if self.walls[d][line_index] {
                match d {
                    UP => {
                        i = MIN_POS;
                    }
                    RIGHT => {
                        j = MAX_POS;
                    }
                    DOWN => {
                        i = MAX_POS;
                    }
                    LEFT => {
                        j = MIN_POS;
                    }
                    _ => unreachable!(),
                }
                d = (d + 1) % 4; // Turn clockwise
            } else {
                // Guard exits the grid
                return (false, loop_length);
            }
        }
    }
}

#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    part1_naive(input)
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    let mut grid = Grid::parse_input(input);
    let (_sth, cells, wall_i, wall_j) = part1_impl(input);
    let mut count = 0;
    for cell in cells {
        let (i, j) = cell;
        grid.grid[0][j] |= 1 << i;
        grid.grid[1][i] |= 1 << j;
        unsafe {
            let (looping, length) = grid.is_looping();
            if looping {
                // println!("{}, {} - {}", i, j, length);
                count += 1;
            }
        }
        grid.grid[0][j] &= !(1 << i);
        grid.grid[1][i] &= !(1 << j);
    }
    // final for wall_i, wall_j
    grid.walls[wall_i][wall_j] = true;
    unsafe {
        if grid.is_looping().0 {
            // println!("{}, {} - {}", wall_i, wall_j, grid.is_looping().1);
            count += 1;
        }
    }
    // println!("Looping: {}", looping);
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    const DAY: u8 = 6; // FIXME: change day here
    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    #[test]
    fn test_part1_naive() {
        assert_eq!(part1_naive(INPUT), 41);
    }

    #[test]
    fn test_part2_naive() {
        assert_eq!(part2_naive(INPUT), 6);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 41);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 6);
    }

    #[test]
    fn test_compare_part1_with_file() {
        let paths = [
            format!("day{DAY}.txt"),
            format!("day{DAY}-alt1.txt"),
            format!("day{DAY}-alt2.txt"),
        ];
        let outputs: [usize; 3] = [4433, 0, 0];
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
        let outputs: [usize; 3] = [1516, 0, 0];
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
