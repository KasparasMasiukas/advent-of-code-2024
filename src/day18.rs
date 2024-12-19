use std::hint::unreachable_unchecked;
use std::mem::MaybeUninit;
use std::ptr;
use std::str;

#[aoc(day18, part1)]
pub fn part1(input: &str) -> usize {
    unsafe { part1_impl(input.as_bytes()) }
}

#[aoc(day18, part2)]
pub fn part2(_input: &str) -> &'static str {
    unsafe { part2_impl(_input.as_bytes()) }
}

const GRID_SIZE: usize = 71;
const LINE_LEN: usize = GRID_SIZE + 1;
const PADDING: usize = LINE_LEN;
const START_COORDS: usize = encode_coords(0, 1);
const END_COORDS: usize = encode_coords(70, 71);
const P1_BLOCKS: usize = 1024;
const P2_START_FROM_BLOCKS: usize = 3200; // 3450 total, but we can risk it for the biscuit

const VISITED_SIZE: usize = GRID_SIZE * LINE_LEN + PADDING * 2;
static mut VISITED: [u16; VISITED_SIZE] = {
    let mut arr = [0; GRID_SIZE * LINE_LEN + PADDING * 2];
    reset_visited(&mut arr);
    arr
};

static mut TRUE: u16 = 0;
static mut NEXT: [([usize; VISITED_SIZE / 2], usize); 2] = [([0; VISITED_SIZE / 2], 0); 2];
static mut STACK: [usize; VISITED_SIZE] = [0; VISITED_SIZE];
static mut STACK_SIZE: usize = 0;

const fn reset_visited(visited: &mut [u16; VISITED_SIZE]) {
    let mut i = 0;
    while i < PADDING {
        visited[i] = u16::MAX;
        visited[VISITED_SIZE - 1 - i] = u16::MAX;
        i += 1;
    }
    i = 0;
    while i < GRID_SIZE {
        visited[i * LINE_LEN + LINE_LEN - 1] = u16::MAX;
        i += 1;
    }
}

#[allow(static_mut_refs)]
unsafe fn part1_impl(input: &[u8]) -> usize {
    TRUE = TRUE.wrapping_add(1);
    if TRUE == u16::MAX {
        ptr::write_bytes(VISITED.as_mut_ptr(), 0, VISITED_SIZE);
        reset_visited(&mut VISITED);
        TRUE = 1;
    }
    let mut ptr = input.as_ptr();
    for _ in 0..P1_BLOCKS {
        let (x, y) = parse_line(&mut ptr);
        let coords = encode_coords(x, y + 1);
        *VISITED.get_unchecked_mut(coords) = TRUE;
    }

    VISITED[START_COORDS] = TRUE;
    let next = NEXT.get_unchecked_mut(0);
    next.0[0] = START_COORDS;
    next.1 = 1;
    (*NEXT.get_unchecked_mut(1)).1 = 0;
    let mut cost = 0;
    loop {
        let queue = NEXT.get_unchecked_mut(cost & 1);
        cost += 1;
        let next = NEXT.get_unchecked_mut(cost & 1);
        for i in 0..queue.1 {
            let coords = *queue.0.get_unchecked(i);
            if coords == END_COORDS {
                return cost - 1;
            }

            // Left
            let next_coords = left(coords);
            if VISITED[next_coords] < TRUE {
                VISITED[next_coords] = TRUE;
                *(*next).0.get_unchecked_mut((*next).1) = next_coords;
                (*next).1 += 1;
            }

            // Right
            let next_coords = right(coords);
            if VISITED[next_coords] < TRUE {
                VISITED[next_coords] = TRUE;
                *(*next).0.get_unchecked_mut((*next).1) = next_coords;
                (*next).1 += 1;
            }

            // Up
            let next_coords = up(coords);
            if VISITED[next_coords] < TRUE {
                VISITED[next_coords] = TRUE;
                *(*next).0.get_unchecked_mut((*next).1) = next_coords;
                (*next).1 += 1;
            }

            // Down
            let next_coords = down(coords);
            if VISITED[next_coords] < TRUE {
                VISITED[next_coords] = TRUE;
                *(*next).0.get_unchecked_mut((*next).1) = next_coords;
                (*next).1 += 1;
            }
        }
        (*queue).1 = 0;
    }
}

#[allow(static_mut_refs)]
unsafe fn part2_impl(input: &[u8]) -> &'static str {
    TRUE = TRUE.wrapping_add(2);
    if TRUE == 0 || TRUE == u16::MAX {
        ptr::write_bytes(VISITED.as_mut_ptr(), 0, VISITED_SIZE);
        reset_visited(&mut VISITED);
        TRUE = 2;
    }
    let mut ptr = input.as_ptr();
    let mut blocks: MaybeUninit<[usize; P2_START_FROM_BLOCKS]> = MaybeUninit::uninit();
    let mut blocks_ptr = blocks.as_mut_ptr() as *mut MaybeUninit<usize>;
    for _ in 0..P2_START_FROM_BLOCKS {
        let (x, y) = parse_line(&mut ptr);
        let coords = encode_coords(x, y + 1);
        *VISITED.get_unchecked_mut(coords) = TRUE + 1;
        blocks_ptr.write(MaybeUninit::new(coords));
        blocks_ptr = blocks_ptr.add(1);
    }
    let blocks = blocks.assume_init();

    VISITED[START_COORDS] = TRUE;
    STACK[0] = START_COORDS;
    STACK_SIZE = 1;
    expand();
    for i in (0..P2_START_FROM_BLOCKS).rev() {
        let block_coords = *blocks.get_unchecked(i);
        let visited = VISITED.get_unchecked_mut(block_coords);
        *visited = TRUE - 1;
        if consider(block_coords) {
            *STACK.get_unchecked_mut(STACK_SIZE) = block_coords;
            STACK_SIZE += 1;
            *visited = TRUE;
            if expand() {
                return decode_coords(block_coords);
            }
        }
    }
    unreachable_unchecked();
}

#[inline(always)]
#[allow(static_mut_refs)]
unsafe fn expand() -> bool {
    while STACK_SIZE > 0 {
        STACK_SIZE -= 1;
        let coords = *STACK.get_unchecked(STACK_SIZE);
        if coords == END_COORDS {
            return true;
        }

        // Left
        let next_coords = left(coords);
        let visited = VISITED.get_unchecked_mut(next_coords);
        if *visited < TRUE {
            *visited = TRUE;
            *STACK.get_unchecked_mut(STACK_SIZE) = next_coords;
            STACK_SIZE += 1;
        }

        // Right
        let next_coords = right(coords);
        let visited = VISITED.get_unchecked_mut(next_coords);
        if *visited < TRUE {
            *visited = TRUE;
            *STACK.get_unchecked_mut(STACK_SIZE) = next_coords;
            STACK_SIZE += 1;
        }

        // Up
        let next_coords = up(coords);
        let visited = VISITED.get_unchecked_mut(next_coords);
        if *visited < TRUE {
            *visited = TRUE;
            *STACK.get_unchecked_mut(STACK_SIZE) = next_coords;
            STACK_SIZE += 1;
        }

        // Down
        let next_coords = down(coords);
        let visited = VISITED.get_unchecked_mut(next_coords);
        if *visited < TRUE {
            *visited = TRUE;
            *STACK.get_unchecked_mut(STACK_SIZE) = next_coords;
            STACK_SIZE += 1;
        }
    }
    false
}

/// Consider if a block should be expanded
/// It should be expanded if it's adjacent to any visited cell -
/// it means it may open up new paths
#[inline(always)]
#[allow(static_mut_refs)]
unsafe fn consider(block_coords: usize) -> bool {
    *VISITED.get_unchecked(left(block_coords)) == TRUE
        || *VISITED.get_unchecked(right(block_coords)) == TRUE
        || *VISITED.get_unchecked(up(block_coords)) == TRUE
        || *VISITED.get_unchecked(down(block_coords)) == TRUE
}

#[inline(always)]
unsafe fn parse_line(ptr: *mut *const u8) -> (usize, usize) {
    let mut p = *ptr;

    let mut c = *p;
    let mut first_num = c - b'0';
    p = p.add(1);

    c = *p;
    if c != b',' {
        first_num = first_num * 10 + (c - b'0');
        p = p.add(1);
    }

    p = p.add(1); // ,

    c = *p;
    let mut second_num = c - b'0';
    p = p.add(1);

    c = *p;
    if c != b'\n' {
        second_num = second_num * 10 + (c - b'0');
        p = p.add(1);
    }

    p = p.add(1);

    *ptr = p;

    (first_num as usize, second_num as usize)
}

#[inline(always)]
const fn encode_coords(x: usize, y: usize) -> usize {
    // Remember to add 1 to y to account for padding
    y * LINE_LEN + x
}

static mut BUFFER: [u8; 5] = [0; 5];
#[inline(always)]
#[allow(static_mut_refs)]
unsafe fn decode_coords(coords: usize) -> &'static str {
    let mut out_size = 0;
    let x = coords % LINE_LEN;
    let y = coords / LINE_LEN - 1;

    if x >= 10 {
        *BUFFER.get_unchecked_mut(out_size) = b'0' + (x / 10) as u8;
        out_size += 1;
    }
    *BUFFER.get_unchecked_mut(out_size) = b'0' + (x % 10) as u8;
    *BUFFER.get_unchecked_mut(out_size + 1) = b',';
    out_size += 2;

    if y >= 10 {
        *BUFFER.get_unchecked_mut(out_size) = b'0' + (y / 10) as u8;
        out_size += 1;
    }
    *BUFFER.get_unchecked_mut(out_size) = b'0' + (y % 10) as u8;
    out_size += 1;

    str::from_utf8_unchecked(&BUFFER[..out_size])
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

#[allow(static_mut_refs)]
#[allow(dead_code)]
unsafe fn visualize_grid() {
    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let coords = encode_coords(x, y + 1);
            let visited = VISITED[coords];
            if visited == TRUE + 1 {
                print!("#");
            } else if visited == TRUE {
                print!("O");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    const DAY: u8 = 18;

    #[test]
    fn test_parse_line() {
        let lines = b"7,33
17,9
19,20
65,61
11,56
4,41
40,47
8,41
";
        let mut ptr = lines.as_ptr();
        let (a, b) = unsafe { parse_line(&mut ptr) };
        assert_eq!(a, 7);
        assert_eq!(b, 33);
        let (a, b) = unsafe { parse_line(&mut ptr) };
        assert_eq!(a, 17);
        assert_eq!(b, 9);
        let (a, b) = unsafe { parse_line(&mut ptr) };
        assert_eq!(a, 19);
        assert_eq!(b, 20);
        let (a, b) = unsafe { parse_line(&mut ptr) };
        assert_eq!(a, 65);
        assert_eq!(b, 61);
        let (a, b) = unsafe { parse_line(&mut ptr) };
        assert_eq!(a, 11);
        assert_eq!(b, 56);
        let (a, b) = unsafe { parse_line(&mut ptr) };
        assert_eq!(a, 4);
        assert_eq!(b, 41);
        let (a, b) = unsafe { parse_line(&mut ptr) };
        assert_eq!(a, 40);
        assert_eq!(b, 47);
        let (a, b) = unsafe { parse_line(&mut ptr) };
        assert_eq!(a, 8);
        assert_eq!(b, 41);
    }

    #[test]
    fn test_compare_part1_with_file() {
        let paths = [
            format!("day{DAY}.txt"),
            format!("day{DAY}-alt1.txt"),
            format!("day{DAY}-alt2.txt"),
        ];
        let outputs = [326, 416, 288];
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
        let outputs = ["18,62", "50,23", "52,5"];
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
