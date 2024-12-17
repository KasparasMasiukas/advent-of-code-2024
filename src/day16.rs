#[aoc(day16, part1)]
pub fn part1(input: &str) -> u32 {
    unsafe { part1_impl(input.as_bytes()) }
}

#[aoc(day16, part2)]
pub fn part2(input: &str) -> usize {
    unsafe { part2_impl(input.as_bytes()) }
}

const GRID_SIZE: usize = 141;
const LINE_LEN: usize = GRID_SIZE + 1;
const START_POS: usize = (GRID_SIZE - 2) * LINE_LEN + 1;
const START_POS_DIR: usize = combine_pos_dir(START_POS, 0);
const END_POS: usize = LINE_LEN + GRID_SIZE - 2;

// Directions: 0=East, 1=South, 2=West, 3=North
const DIR: [isize; 4] = [1, LINE_LEN as isize, -1, -(LINE_LEN as isize)];

// Each cell has 4 directions
const STATE_COUNT: usize = GRID_SIZE * LINE_LEN * 4 + 4;

#[derive(Clone, Copy, Debug, Default)]
struct QueueItem {
    cost: u32,
    pos: usize,
    dir: usize,
}

#[inline(always)]
const fn combine_pos_dir(pos: usize, dir: usize) -> usize {
    (pos << 1) + (dir & 1)
}

// Only horizontals/verticals are checked for visited
static mut VISITED_DIST: [(u32, u32); STATE_COUNT / 2] = [(0, 0); STATE_COUNT / 2];
static mut VALID_SPOT: [u32; GRID_SIZE * LINE_LEN] = [0; GRID_SIZE * LINE_LEN];
static mut TRUE: u32 = 0;

#[allow(static_mut_refs)]
unsafe fn part1_impl(input: &[u8]) -> u32 {
    TRUE = TRUE.wrapping_add(1);
    if TRUE == 0 {
        std::ptr::write_bytes(VISITED_DIST.as_mut_ptr(), 0, STATE_COUNT);
        TRUE = 1;
    }

    *VISITED_DIST.get_unchecked_mut(START_POS_DIR) = (TRUE, 0);
    forward_queue_clear();
    turn_queue_clear();
    forward_queue_push(0, START_POS, 0);

    loop {
        // 1.. -> 1001 -> 2.. -> 1002 -> 3.. -> 1003 -> 4.. -> 1004
        // Handle forwards first as they are the cheapest
        // This may keep adding forwards but they are guaranteed to be cheaper than any turns
        while let Some((cost, pos, dir)) = forward_queue_pop() {
            if pos == END_POS {
                return cost;
            }
            expand_forward(cost, pos, dir, input);
        }

        // Expand all turns - this will only lead to new forwards being added.
        // Double turns are not possible in a maze
        // (based on input - after every turn there's at least one forward before the next turn).
        // Because of that, this loop can only populate the forwards queue
        while let Some((cost, pos, dir)) = turn_queue_pop() {
            if pos == END_POS {
                return cost;
            }
            expand_turn(cost, pos, dir, input);
        }
    }
}

#[inline(always)]
#[allow(static_mut_refs)]
unsafe fn expand_forward(cost: u32, pos: usize, dir: usize, input: &[u8]) {
    // Forward
    let new_pos = (pos as isize + DIR[dir]) as usize;
    let new_pos_dir = combine_pos_dir(new_pos, dir);
    let visited = VISITED_DIST.get_unchecked_mut(new_pos_dir);
    let new_cost = cost + 1;
    if *input.get_unchecked(new_pos) != b'#' && (visited.0 != TRUE || new_cost < visited.1) {
        forward_queue_push(new_cost, new_pos, dir);
        *visited = (TRUE, new_cost);
    }

    // Anti-clockwise
    let new_dir = (dir + 3) & 3;
    let new_pos = (pos as isize + DIR[new_dir]) as usize;
    let new_pos_dir = combine_pos_dir(new_pos, new_dir);
    let visited = VISITED_DIST.get_unchecked_mut(new_pos_dir);
    let new_cost = cost + 1001;
    if *input.get_unchecked(new_pos) != b'#' && (visited.0 != TRUE || new_cost < visited.1) {
        turn_queue_push(cost + 1001, new_pos, new_dir);
        *visited = (TRUE, new_cost);
    }

    // Clockwise
    let new_dir = (dir + 1) & 3;
    let new_pos = (pos as isize + DIR[new_dir]) as usize;
    let new_pos_dir = combine_pos_dir(new_pos, new_dir);
    let visited = VISITED_DIST.get_unchecked_mut(new_pos_dir);
    let new_cost = cost + 1001;
    if *input.get_unchecked(new_pos) != b'#' && (visited.0 != TRUE || new_cost < visited.1) {
        turn_queue_push(cost + 1001, new_pos, new_dir);
        *visited = (TRUE, new_cost);
    }
}

#[inline(always)]
#[allow(static_mut_refs)]
unsafe fn expand_turn(cost: u32, pos: usize, dir: usize, input: &[u8]) {
    // Forward
    let new_pos = (pos as isize + DIR[dir]) as usize;
    let new_pos_dir = combine_pos_dir(new_pos, dir);
    let visited = VISITED_DIST.get_unchecked_mut(new_pos_dir);
    let new_cost = cost + 1;
    if *input.get_unchecked(new_pos) != b'#' && (visited.0 != TRUE || new_cost < visited.1) {
        forward_queue_push(new_cost, new_pos, dir);
        *visited = (TRUE, new_cost);
    }
}

#[allow(static_mut_refs)]
unsafe fn part2_impl(input: &[u8]) -> usize {
    // Phase 1 - find the minimum cost to reach the end pos
    let min_cost_found = part1_impl(input);

    // Phase 2 - trace back valid paths from the end pos
    forward_queue_clear();
    turn_queue_clear();
    for i in 0..4 {
        // We may not have visited the end pos in all directions because we break early.
        // Just in case it's possible to reach solution in more than one direction, we should try all
        forward_queue_push(min_cost_found, END_POS, i);
    }

    let mut count = 0;
    // let mut canvas = [b' '; GRID_SIZE * LINE_LEN];
    while FORWARD_QUEUE_SIZE > 0 || TURN_QUEUE_SIZE > 0 {
        // This exploits the same fact as in part 1:
        // Forwards are cheaper than turns, and turns are always followed by forwards
        while let Some((cost, pos, dir)) = forward_queue_pop() {
            let valid_spot = VALID_SPOT.get_unchecked_mut(pos);
            if *valid_spot != TRUE {
                *valid_spot = TRUE;
                // canvas[pos] = b'O';
                count += 1;
            }
            if pos == START_POS {
                continue;
            }

            expand_backward(cost, pos, dir);
        }

        while let Some((cost, pos, dir)) = turn_queue_pop() {
            let valid_spot = VALID_SPOT.get_unchecked_mut(pos);
            if *valid_spot != TRUE {
                *valid_spot = TRUE;
                // canvas[pos] = b'O';
                count += 1;
            }
            if pos == START_POS {
                continue;
            }

            expand_backward_turn(cost, pos, dir);
        }
    }
    // Print canvas
    // for i in 0..GRID_SIZE {
    //     println!(
    //         "{}",
    //         std::str::from_utf8_unchecked(&canvas[i * LINE_LEN..(i + 1) * LINE_LEN])
    //     );
    // }

    count
}

#[inline(always)]
#[allow(static_mut_refs)]
unsafe fn expand_backward(cost: u32, pos: usize, dir: usize) {
    // Backward
    let new_pos = (pos as isize - DIR[dir]) as usize;
    let new_pos_dir = combine_pos_dir(new_pos, dir);
    let visited = VISITED_DIST.get_unchecked(new_pos_dir);
    if visited.0 == TRUE && visited.1 == cost - 1 {
        forward_queue_push(cost - 1, new_pos, dir);
    }

    // Backward and clockwise
    let new_dir = (dir + 1) & 3;
    let new_pos = (pos as isize - DIR[dir]) as usize;
    let new_pos_dir = combine_pos_dir(new_pos, new_dir);
    let visited = VISITED_DIST.get_unchecked(new_pos_dir);
    if visited.0 == TRUE && visited.1 == cost - 1001 {
        turn_queue_push(cost - 1001, new_pos, new_dir);
    }

    // Backward and anti-clockwise
    let new_dir = (dir + 3) & 3;
    let new_pos = (pos as isize - DIR[dir]) as usize;
    let new_pos_dir = combine_pos_dir(new_pos, new_dir);
    let visited = VISITED_DIST.get_unchecked(new_pos_dir);
    if visited.0 == TRUE && visited.1 == cost - 1001 {
        turn_queue_push(cost - 1001, new_pos, new_dir);
    }
}

#[inline(always)]
#[allow(static_mut_refs)]
unsafe fn expand_backward_turn(cost: u32, pos: usize, dir: usize) {
    // Backward
    let new_pos = (pos as isize - DIR[dir]) as usize;
    let new_pos_dir = combine_pos_dir(new_pos, dir);
    let visited = VISITED_DIST.get_unchecked(new_pos_dir);
    if visited.0 == TRUE && visited.1 == cost - 1 {
        forward_queue_push(cost - 1, new_pos, dir);
    }
}

static mut FORWARD_QUEUE: [QueueItem; STATE_COUNT] = [QueueItem {
    cost: 0,
    pos: 0,
    dir: 0,
}; STATE_COUNT];
static mut FORWARD_QUEUE_SIZE: usize = 0;
static mut TURN_QUEUE: [QueueItem; STATE_COUNT] = [QueueItem {
    cost: 0,
    pos: 0,
    dir: 0,
}; STATE_COUNT];
static mut TURN_QUEUE_SIZE: usize = 0;

#[inline(always)]
unsafe fn forward_queue_clear() {
    FORWARD_QUEUE_SIZE = 0;
}

#[inline(always)]
unsafe fn forward_queue_push(cost: u32, pos: usize, dir: usize) {
    FORWARD_QUEUE[FORWARD_QUEUE_SIZE] = QueueItem { cost, pos, dir };
    FORWARD_QUEUE_SIZE += 1;
}

#[inline(always)]
unsafe fn forward_queue_pop() -> Option<(u32, usize, usize)> {
    if FORWARD_QUEUE_SIZE == 0 {
        return None;
    }
    FORWARD_QUEUE_SIZE -= 1;
    let ret = FORWARD_QUEUE[FORWARD_QUEUE_SIZE];
    Some((ret.cost, ret.pos, ret.dir))
}

#[inline(always)]
unsafe fn turn_queue_clear() {
    TURN_QUEUE_SIZE = 0;
}

#[inline(always)]
unsafe fn turn_queue_push(cost: u32, pos: usize, dir: usize) {
    TURN_QUEUE[TURN_QUEUE_SIZE] = QueueItem { cost, pos, dir };
    TURN_QUEUE_SIZE += 1;
}

#[inline(always)]
unsafe fn turn_queue_pop() -> Option<(u32, usize, usize)> {
    if TURN_QUEUE_SIZE == 0 {
        return None;
    }
    TURN_QUEUE_SIZE -= 1;
    let ret = TURN_QUEUE[TURN_QUEUE_SIZE];
    Some((ret.cost, ret.pos, ret.dir))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    const DAY: u8 = 16;
    const INPUT: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";

    const INPUT2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
";

    #[test]
    #[ignore]
    fn test_part1() {
        assert_eq!(part1(INPUT), 7036);
        assert_eq!(part1(INPUT2), 11048);
    }

    #[test]
    #[ignore]
    fn test_part2() {
        assert_eq!(part2(INPUT), 45);
        assert_eq!(part2(INPUT2), 64);
    }

    #[test]
    fn test_compare_part1_with_file() {
        let paths = [
            format!("day{DAY}.txt"),
            format!("day{DAY}-alt1.txt"),
            format!("day{DAY}-alt2.txt"),
            format!("day{DAY}-alt3.txt"),
        ];
        let outputs = [94444, 85432, 115500, 93436];
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
            format!("day{DAY}-alt3.txt"),
        ];
        let outputs = [502, 465, 679, 486];
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
