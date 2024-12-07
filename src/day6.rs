use std::collections::HashSet;

/// Directions represented as (delta_row, delta_col)
const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

/// Simulates the guard's movement and counts unique visited spaces.
///
/// # Arguments
///
/// * `input` - A string slice representing the grid.
///
/// # Returns
///
/// * `usize` - The number of unique spaces visited by the guard.
#[aoc(day6, part1, naive)]
pub fn part1_naive(input: &str) -> usize {
    // Parse the input into a grid of characters
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let num_rows = grid.len();
    if num_rows == 0 {
        return 0;
    }
    let num_cols = grid[0].len();

    // Find the starting position and initial direction
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

    // If no starting position found, return 0
    let (mut row, mut col) = match start_pos {
        Some(pos) => pos,
        None => return 0,
    };

    // Initialize direction to up (0)
    let mut direction = 0;

    // Initialize visited set with starting position
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((row, col));

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
        visited.insert((row, col));
    }

    // Return the number of unique visited positions
    visited.len()
}

/// Simulates the guard's movement and detects if it gets stuck in a loop.
/// Returns true if the guard is stuck, false otherwise.
///
/// # Arguments
///
/// * `grid` - A reference to the grid of characters.
/// * `start_pos` - The starting position of the guard.
/// * `start_dir` - The starting direction index (0: up, 1: right, 2: down, 3: left).
/// * `visited` - A mutable slice to track visited states.
///
/// # Returns
///
/// * `bool` - True if the guard is stuck in a loop, False otherwise.
fn is_guard_stuck(
    grid: &[Vec<char>],
    start_pos: (i32, i32),
    start_dir: usize,
    visited: &mut [bool],
    num_rows: usize,
    num_cols: usize,
) -> bool {
    let mut row = start_pos.0;
    let mut col = start_pos.1;
    let mut direction = start_dir;

    loop {
        // Encode the current state
        let state_index = (row as usize * num_cols + col as usize) * 4 + direction;
        if visited[state_index] {
            // Loop detected
            return true;
        }
        visited[state_index] = true;

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
            // Guard exits the grid
            return false;
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
    }
}

// Part 2 optimization ideas:
// TODO: pre-process each cell to find the closest obstacle in each direction for movement (no offset)
// ^ Set up a 3D array graph (130x130x4) to store the closest obstacle in each direction for each cell
// ^ After placing an obstacle, anytime we'd want to get next obstacle for movement, we have to check if column or row matches, we might get intercepted by the new obstacle instead
// TODO: instead of cloning the grid, have one static with generation counter += 4 (because we have 4 directions)//
// TODO: run the initial loop to identify viable cells that can impact the guard's movement (only <25% of all cells would be valid)
// Idea: grid could be represented as a 2D bitmask (130 bit x 130 bit). In that case, from each position we can shift left/right to find the next obstacle in each direction
// ^ for the above to work, we'd also need vertical bitmask (too bad 130 bit doesn't fit in u128) - AHA! there's no way to enter the 1st or 130th row/column without exiting the grid
// because hitting the wall redirects the guard. we only need booleans if we have walls on left, right, up, down

/// Counts the number of empty spots where placing an obstacle would cause the guard to get stuck.
///
/// # Arguments
///
/// * `input` - A string slice representing the grid.
///
/// # Returns
///
/// * `usize` - The number of empty spots that cause the guard to get stuck when blocked.
#[aoc(day6, part2, naive)]
pub fn part2_naive(input: &str) -> usize {
    // Parse the input into a grid of characters
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let num_rows = grid.len();
    if num_rows == 0 {
        return 0;
    }
    let num_cols = grid[0].len();

    // Find the starting position and initial direction
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

    // If no starting position found, return 0
    let (start_row, start_col) = match start_pos {
        Some(pos) => pos,
        None => return 0,
    };

    // Initialize direction to up (0)
    let start_dir = 0;

    // Collect all empty cells
    let mut empty_cells: Vec<(usize, usize)> = Vec::new();
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, &ch) in row.iter().enumerate() {
            if ch == '.' {
                empty_cells.push((row_idx, col_idx));
            }
        }
    }

    let mut stuck_count = 0;

    // Preallocate the visited array
    let total_states = num_rows * num_cols * 4;
    let mut visited = vec![false; total_states];

    for &(row, col) in &empty_cells {
        let original_char = grid[row][col];
        let mut mutable_grid = grid.clone();
        mutable_grid[row][col] = '#';

        // Reset the visited array
        for v in visited.iter_mut() {
            *v = false;
        }

        // Simulate the guard's movement
        if is_guard_stuck(
            &mutable_grid,
            (start_row, start_col),
            start_dir,
            &mut visited,
            num_rows,
            num_cols,
        ) {
            stuck_count += 1;
        }
    }

    stuck_count
}

#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    part1_naive(input)
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    part2_naive(input)
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
