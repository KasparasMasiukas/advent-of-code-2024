use memchr::{memchr, memchr2};
use std::ptr;

const GRID_SIZE: usize = 141;
const LINE_LEN: usize = GRID_SIZE + 1;
const LINE_LEN_ISIZE: isize = LINE_LEN as isize;
const VISITED_SIZE: usize = GRID_SIZE * LINE_LEN;
// TRUE/FALSE => cost = value - TRUE
static mut VISITED: [u32; VISITED_SIZE] = [0; VISITED_SIZE];
static mut TRUE: u32 = 0;
static mut NEXT: [([usize; VISITED_SIZE / 2], usize); 2] = [([0; VISITED_SIZE / 2], 0); 2];
static mut HISTORY: [(usize, u32); VISITED_SIZE] = [(0, 0); VISITED_SIZE];

#[inline(always)]
const fn manhattan_distance_xy(x1: usize, y1: usize, x2: usize, y2: usize) -> u32 {
    let delta_x = (x1 as isize - x2 as isize).abs() as u32;
    let delta_y = (y1 as isize - y2 as isize).abs() as u32;
    delta_x + delta_y
}

#[aoc(day20, part1)]
pub fn part1(input: &str) -> usize {
    unsafe { part1_impl(input.as_bytes()) }
}

#[aoc(day20, part2)]
pub fn part2(input: &str) -> usize {
    unsafe { part2_impl(input.as_bytes()) }
}

#[allow(static_mut_refs)]
unsafe fn part1_impl(input: &[u8]) -> usize {
    TRUE += 1;
    if TRUE > u32::MAX - u16::MAX as u32 {
        ptr::write_bytes(VISITED.as_mut_ptr(), 0, VISITED_SIZE);
    }

    let (start_pos, end_pos) = find_start_end(input);
    *VISITED.get_unchecked_mut(end_pos) = TRUE;
    let next = NEXT.get_unchecked_mut(0);
    next.0[0] = end_pos;
    next.1 = 1;
    (*NEXT.get_unchecked_mut(1)).1 = 0;
    let mut cost = 0u32;

    // Phase 1: need to reach at least 101 cost before we can start saving anything
    while cost < 101 {
        let queue = NEXT.get_unchecked_mut((cost & 1) as usize);
        cost += 1;
        let next_visited = TRUE + cost;
        let next = NEXT.get_unchecked_mut((cost & 1) as usize);
        for i in 0..queue.1 {
            let coords = *queue.0.get_unchecked(i);
            expand(left(coords), next_visited, input, next);
            expand(right(coords), next_visited, input, next);
            expand(up(coords), next_visited, input, next);
            expand(down(coords), next_visited, input, next);
        }
        (*queue).1 = 0;
    }

    // Phase 2: continue exploring until we reach start, but also start checking for cheats
    let mut cheats = 0;
    let mut cheat_threshold = TRUE; // Nodes that are less than this value are valid cheats
    loop {
        let queue = NEXT.get_unchecked_mut((cost & 1) as usize);
        cost += 1;
        let next_visited = TRUE + cost;
        cheat_threshold += 1;
        let next = NEXT.get_unchecked_mut((cost & 1) as usize);
        for i in 0..queue.1 {
            let coords = *queue.0.get_unchecked(i);

            // Left
            let next_coords = left(coords);
            if *input.get_unchecked(next_coords) > b'#' {
                let visited = VISITED.get_unchecked_mut(next_coords);
                if *visited < TRUE {
                    *visited = next_visited;
                    *(*next).0.get_unchecked_mut((*next).1) = next_coords;
                    (*next).1 += 1;
                }
            } else if next_coords % LINE_LEN > 2
                && (TRUE..cheat_threshold).contains(VISITED.get_unchecked(left(next_coords)))
            {
                cheats += 1;
            }

            // Right
            let next_coords = right(coords);
            if *input.get_unchecked(next_coords) > b'#' {
                let visited = VISITED.get_unchecked_mut(next_coords);
                if *visited < TRUE {
                    *visited = next_visited;
                    *(*next).0.get_unchecked_mut((*next).1) = next_coords;
                    (*next).1 += 1;
                }
            } else if next_coords % LINE_LEN < GRID_SIZE - 3
                && (TRUE..cheat_threshold).contains(VISITED.get_unchecked(right(next_coords)))
            {
                cheats += 1;
            }

            // Up
            let next_coords = up(coords);
            if *input.get_unchecked(next_coords) > b'#' {
                let visited = VISITED.get_unchecked_mut(next_coords);
                if *visited < TRUE {
                    *visited = next_visited;
                    *(*next).0.get_unchecked_mut((*next).1) = next_coords;
                    (*next).1 += 1;
                }
            } else if next_coords > LINE_LEN * 3 + 1
                && (TRUE..cheat_threshold).contains(VISITED.get_unchecked(up(next_coords)))
            {
                cheats += 1;
            }

            // Down
            let next_coords = down(coords);
            if *input.get_unchecked(next_coords) > b'#' {
                let visited = VISITED.get_unchecked_mut(next_coords);
                if *visited < TRUE {
                    *visited = next_visited;
                    *(*next).0.get_unchecked_mut((*next).1) = next_coords;
                    (*next).1 += 1;
                }
            } else if next_coords < VISITED_SIZE - LINE_LEN * 3 - 1
                && (TRUE..cheat_threshold).contains(VISITED.get_unchecked(down(next_coords)))
            {
                cheats += 1;
            }

            if coords == start_pos {
                TRUE += cost;
                return cheats;
            }
        }
        (*queue).1 = 0;
    }
}

#[allow(static_mut_refs)]
unsafe fn part2_impl(input: &[u8]) -> usize {
    TRUE += 1;
    if TRUE > u32::MAX - u16::MAX as u32 {
        ptr::write_bytes(VISITED.as_mut_ptr(), 0, VISITED_SIZE);
    }

    let (start_pos, end_pos) = find_start_end(input);
    *VISITED.get_unchecked_mut(end_pos) = TRUE;
    let next = NEXT.get_unchecked_mut(0);
    next.0[0] = end_pos;
    next.1 = 1;
    (*NEXT.get_unchecked_mut(1)).1 = 0;
    let mut cost = 0u32;
    let mut history_size = 0;

    // Phase 1: need to reach at least 101 cost before we can start saving anything
    while cost < 101 {
        let queue = NEXT.get_unchecked_mut((cost & 1) as usize);
        cost += 1;
        let next_visited = TRUE + cost;
        let next = NEXT.get_unchecked_mut((cost & 1) as usize);
        for i in 0..queue.1 {
            let coords = *queue.0.get_unchecked(i);
            expand(left(coords), next_visited, input, next);
            expand(right(coords), next_visited, input, next);
            expand(up(coords), next_visited, input, next);
            expand(down(coords), next_visited, input, next);

            *HISTORY.get_unchecked_mut(history_size) = (coords, next_visited - 1);
            history_size += 1;
        }
        (*queue).1 = 0;
    }

    // Phase 2: continue exploring until we reach start, but also start checking for cheats.
    // For now, history is relatively short - we can check the history for cheats
    let mut cheats = 0;
    let mut cheat_threshold = TRUE + cost - 99; // Distances that are less than this value are valid cheats
    while cost < 1000 {
        let queue = NEXT.get_unchecked_mut((cost & 1) as usize);
        cost += 1;
        let next_visited = TRUE + cost;
        cheat_threshold += 1;
        let next = NEXT.get_unchecked_mut((cost & 1) as usize);
        for i in 0..queue.1 {
            let coords = *queue.0.get_unchecked(i);
            let x = coords % LINE_LEN;
            let y = coords / LINE_LEN;

            // Check cheats
            let mut i = 0;
            let mut history_entry = *HISTORY.get_unchecked(i);
            while history_entry.1 < cheat_threshold {
                let dist = manhattan_distance_xy(
                    x,
                    y,
                    history_entry.0 % LINE_LEN,
                    history_entry.0 / LINE_LEN,
                );
                if dist <= 20 && history_entry.1 + dist < cheat_threshold {
                    cheats += 1;
                }
                i += 1;
                history_entry = *HISTORY.get_unchecked(i);
            }

            expand(left(coords), next_visited, input, next);
            expand(right(coords), next_visited, input, next);
            expand(up(coords), next_visited, input, next);
            expand(down(coords), next_visited, input, next);

            *HISTORY.get_unchecked_mut(history_size) = (coords, next_visited - 1);
            history_size += 1;
        }
        (*queue).1 = 0;
    }

    // Phase 3: like phase 2, but this time we have too big of a history - so start checking nearby cells instead of history
    loop {
        let queue = NEXT.get_unchecked_mut((cost & 1) as usize);
        cost += 1;
        let next_visited = TRUE + cost;
        cheat_threshold += 1;
        let next = NEXT.get_unchecked_mut((cost & 1) as usize);
        for i in 0..queue.1 {
            let coords = *queue.0.get_unchecked(i);

            // Check cheats
            count_cheats(
                coords % LINE_LEN,
                coords / LINE_LEN,
                cheat_threshold,
                &mut cheats,
            );

            if coords == start_pos {
                TRUE += cost;
                return cheats;
            }

            expand(left(coords), next_visited, input, next);
            expand(right(coords), next_visited, input, next);
            expand(up(coords), next_visited, input, next);
            expand(down(coords), next_visited, input, next);
        }
        (*queue).1 = 0;
    }
}

#[inline(always)]
#[allow(static_mut_refs)]
unsafe fn expand(
    next_coords: usize,
    next_visited: u32,
    input: &[u8],
    next: &mut ([usize; VISITED_SIZE / 2], usize),
) {
    let visited = VISITED.get_unchecked_mut(next_coords);
    if *input.get_unchecked(next_coords) > b'#' && *visited < TRUE {
        *visited = next_visited;
        *next.0.get_unchecked_mut(next.1) = next_coords;
        next.1 += 1;
    }
}

#[inline(always)]
#[allow(static_mut_refs)]
unsafe fn count_cheats(x: usize, y: usize, cheat_threshold: u32, cheats: &mut usize) {
    let y_min = y.saturating_sub(20).max(1);
    let y_max = (y + 20).min(GRID_SIZE - 2);

    for new_y in y_min..=y_max {
        let dy = (new_y as isize - y as isize).abs() as usize;
        let max_dx = 20 - dy;

        let x_min = x.saturating_sub(max_dx).max(1);
        let x_max = (x + max_dx).min(GRID_SIZE - 2);

        for new_x in x_min..=x_max {
            let dx = (new_x as isize - x as isize).abs() as usize;
            let dist = (dx + dy) as u32;

            let next_coords = new_y * LINE_LEN + new_x;
            let visited_val = *VISITED.get_unchecked(next_coords);
            if visited_val >= TRUE && visited_val + dist < cheat_threshold {
                *cheats += 1;
            }
        }
    }
}

unsafe fn find_start_end(input: &[u8]) -> (usize, usize) {
    let first_pos = memchr2(b'S', b'E', input).unwrap();
    if *input.get_unchecked(first_pos) == b'S' {
        let end_pos = memchr(b'E', &input[first_pos..]).unwrap() + first_pos;
        (first_pos, end_pos)
    } else {
        let start_pos = memchr(b'S', &input[first_pos..]).unwrap() + first_pos;
        (start_pos, first_pos)
    }
}

#[inline(always)]
const fn left(coords: usize) -> usize {
    coords - 1
}

#[inline(always)]
const fn right(coords: usize) -> usize {
    coords + 1
}

#[inline(always)]
const fn up(coords: usize) -> usize {
    coords - LINE_LEN
}

#[inline(always)]
fn down(coords: usize) -> usize {
    coords + LINE_LEN
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    const DAY: u8 = 20;
    const INPUT: &str = ""; // FIXME: add example input here

    #[test]
    #[ignore]
    fn test_part1() {
        assert_eq!(part1(INPUT), 0);
    }

    #[test]
    #[ignore]
    fn test_part2() {
        assert_eq!(part2(INPUT), 0);
    }

    #[test]
    fn test_compare_part1_with_file() {
        let paths = [
            format!("day{DAY}.txt"),
            format!("day{DAY}-alt1.txt"),
            format!("day{DAY}-alt2.txt"),
        ];
        let outputs = [1411, 1521, 1497];
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
        let outputs = [1010263, 1013106, 1030809];
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
