use memchr::memchr_iter;

#[aoc(day10, part1)]
pub fn part1(input: &str) -> u16 {
    unsafe { part1_impl(input.as_bytes()) }
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> u16 {
    unsafe { part2_impl(input.as_bytes()) }
}

/// Assume grid is max 64x64
const MEMO_SIZE: usize = 64 * 64;

unsafe fn part1_impl(grid: &[u8]) -> u16 {
    let line_len = get_line_len(&grid);
    let mut memo: [u16; MEMO_SIZE] = [u16::MAX; MEMO_SIZE];
    let mut total_paths: u16 = 0;

    // Iterate over all '9's and perform DFS to count paths back to '0's.
    for pos in memchr_iter(b'9', grid) {
        let id = pos as u16;
        memo[pos] = id;
        total_paths += dfs9_seen(pos, grid, &mut memo, id, line_len, grid.len());
    }

    total_paths
}

unsafe fn part2_impl(grid: &[u8]) -> u16 {
    let line_len = get_line_len(&grid);
    let mut memo: [u16; MEMO_SIZE] = [0; MEMO_SIZE];
    let mut total_paths: u16 = 0;

    // Iterate over all '9's and perform DFS to count paths back to '0's.
    for pos in memchr_iter(b'9', grid) {
        total_paths += dfs9_memo(pos, grid, &mut memo, line_len, grid.len());
    }

    total_paths
}

#[inline(always)]
fn get_line_len(grid: &[u8]) -> usize {
    let first_newline = memchr::memchr(b'\n', grid).unwrap();
    first_newline + 1 // Includes \n
}

#[inline(always)]
unsafe fn dfs0_seen(
    _pos: usize,
    _grid: &[u8],
    _memo: &mut [u16],
    _id: u16,
    _line_len: usize,
    _grid_len: usize,
) -> u16 {
    1
}

macro_rules! define_dfs_seen {
    ($next_height:expr, $next_fn:ident, $fn_name:ident) => {
        unsafe fn $fn_name(
            pos: usize,
            grid: &[u8],
            memo: &mut [u16],
            id: u16,
            line_len: usize,
            grid_len: usize,
        ) -> u16 {
            let mut cnt: u16 = 0;

            // Left
            if pos >= 1
                && *grid.get_unchecked(pos - 1) == $next_height
                && *memo.get_unchecked(pos - 1) != id
            {
                let next_pos = pos - 1;
                *memo.get_unchecked_mut(next_pos) = id;
                cnt += $next_fn(next_pos, grid, memo, id, line_len, grid_len)
            }

            // Right
            let next_pos = pos + 1;
            if next_pos < grid_len
                && *grid.get_unchecked(next_pos) == $next_height
                && *memo.get_unchecked(next_pos) != id
            {
                *memo.get_unchecked_mut(next_pos) = id;
                cnt += $next_fn(next_pos, grid, memo, id, line_len, grid_len)
            }

            // Up
            if pos >= line_len
                && *grid.get_unchecked(pos - line_len) == $next_height
                && *memo.get_unchecked(pos - line_len) != id
            {
                let next_pos = pos - line_len;
                *memo.get_unchecked_mut(next_pos) = id;
                cnt += $next_fn(next_pos, grid, memo, id, line_len, grid_len)
            }

            // Down
            let next_pos = pos + line_len;
            if next_pos < grid_len
                && *grid.get_unchecked(next_pos) == $next_height
                && *memo.get_unchecked(next_pos) != id
            {
                *memo.get_unchecked_mut(next_pos) = id;
                cnt += $next_fn(next_pos, grid, memo, id, line_len, grid_len)
            }

            cnt
        }
    };
}

define_dfs_seen!(b'0', dfs0_seen, dfs1_seen);
define_dfs_seen!(b'1', dfs1_seen, dfs2_seen);
define_dfs_seen!(b'2', dfs2_seen, dfs3_seen);
define_dfs_seen!(b'3', dfs3_seen, dfs4_seen);
define_dfs_seen!(b'4', dfs4_seen, dfs5_seen);
define_dfs_seen!(b'5', dfs5_seen, dfs6_seen);
define_dfs_seen!(b'6', dfs6_seen, dfs7_seen);
define_dfs_seen!(b'7', dfs7_seen, dfs8_seen);
define_dfs_seen!(b'8', dfs8_seen, dfs9_seen);

#[inline(always)]
unsafe fn dfs0_memo(
    _pos: usize,
    _grid: &[u8],
    _memo: &mut [u16],
    _line_len: usize,
    _grid_len: usize,
) -> u16 {
    1
}

macro_rules! define_dfs_memo {
    ($next_height:expr, $next_fn:ident, $fn_name:ident) => {
        unsafe fn $fn_name(
            pos: usize,
            grid: &[u8],
            memo: &mut [u16],
            line_len: usize,
            grid_len: usize,
        ) -> u16 {
            if *memo.get_unchecked(pos) > 0 {
                return *memo.get_unchecked(pos);
            }

            let mut cnt: u16 = 0;

            // Left
            if pos > 0 && *grid.get_unchecked(pos - 1) == $next_height {
                cnt += $next_fn(pos - 1, grid, memo, line_len, grid_len)
            }

            // Right
            if pos + 1 < grid_len && *grid.get_unchecked(pos + 1) == $next_height {
                cnt += $next_fn(pos + 1, grid, memo, line_len, grid_len)
            }

            // Up
            if pos >= line_len && *grid.get_unchecked(pos - line_len) == $next_height {
                cnt += $next_fn(pos - line_len, grid, memo, line_len, grid_len)
            }

            // Down
            if pos + line_len < grid_len && *grid.get_unchecked(pos + line_len) == $next_height {
                cnt += $next_fn(pos + line_len, grid, memo, line_len, grid_len)
            }

            *memo.get_unchecked_mut(pos) = cnt;

            cnt
        }
    };
}

define_dfs_memo!(b'0', dfs0_memo, dfs1_memo);
define_dfs_memo!(b'1', dfs1_memo, dfs2_memo);
define_dfs_memo!(b'2', dfs2_memo, dfs3_memo);
define_dfs_memo!(b'3', dfs3_memo, dfs4_memo);
define_dfs_memo!(b'4', dfs4_memo, dfs5_memo);
define_dfs_memo!(b'5', dfs5_memo, dfs6_memo);
define_dfs_memo!(b'6', dfs6_memo, dfs7_memo);
define_dfs_memo!(b'7', dfs7_memo, dfs8_memo);
define_dfs_memo!(b'8', dfs8_memo, dfs9_memo);

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    const DAY: u8 = 10;

    #[test]
    fn test_compare_part1_with_file() {
        let paths = [
            format!("day{DAY}.txt"),
            format!("day{DAY}-alt1.txt"),
            format!("day{DAY}-alt2.txt"),
        ];
        let outputs: [u16; 3] = [461, 472, 820];
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
        let outputs: [u16; 3] = [875, 969, 1786];
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
