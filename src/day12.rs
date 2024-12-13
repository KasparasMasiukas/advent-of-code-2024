#[aoc(day12, part1)]
pub fn part1(input: &str) -> u32 {
    unsafe { part1_impl(input.as_bytes()) }
}

const MAX_SIZE: usize = 140;
const MAX_LINE_LEN: usize = MAX_SIZE + 1; // Add \n
static mut VISITED: [u8; MAX_SIZE * MAX_LINE_LEN] = [0; MAX_SIZE * MAX_LINE_LEN];
static mut TRUE: u8 = 0;
static mut STACK: [usize; MAX_SIZE * MAX_LINE_LEN] = [0; MAX_SIZE * MAX_LINE_LEN];

#[inline(always)]
fn compute_height(total_len: usize) -> usize {
    let discriminant = 1 + 4 * total_len;
    let sqrt_discriminant = (discriminant as f64).sqrt();
    let height = (-1.0 + sqrt_discriminant) / 2.0;
    height.round() as usize
}

#[allow(static_mut_refs)]
unsafe fn part1_impl(grid: &[u8]) -> u32 {
    TRUE = TRUE.wrapping_add(1);

    let total_len = grid.len();
    let height = compute_height(total_len);
    let line_len = height + 1;

    let grid_ptr = grid.as_ptr();
    let visited_ptr = VISITED.as_mut_ptr();
    let stack_ptr = STACK.as_mut_ptr();

    let mut stack_top = stack_ptr;
    let mut total_price: u32 = 0;

    let mut i: usize = 0;
    while i < total_len {
        let current_char = *grid_ptr.add(i);

        if current_char != b'\n' && *visited_ptr.add(i) != TRUE {
            *visited_ptr.add(i) = TRUE;

            *stack_top = i;
            stack_top = stack_top.add(1);

            let mut area: u32 = 0;
            let mut perimeter: u32 = 0;
            let region_char = current_char;

            while stack_top != stack_ptr {
                stack_top = stack_top.sub(1);
                let current = *stack_top;
                area += 1;

                // Up
                let up = current.wrapping_sub(line_len);
                if up < total_len && *grid_ptr.add(up) == region_char {
                    if *visited_ptr.add(up) != TRUE {
                        *visited_ptr.add(up) = TRUE;
                        *stack_top = up;
                        stack_top = stack_top.add(1);
                    }
                } else {
                    perimeter += 1;
                }

                // Down
                let down = current + line_len;
                if down < total_len && *grid_ptr.add(down) == region_char {
                    if *visited_ptr.add(down) != TRUE {
                        *visited_ptr.add(down) = TRUE;
                        *stack_top = down;
                        stack_top = stack_top.add(1);
                    }
                } else {
                    perimeter += 1;
                }

                // Left
                let left = current.wrapping_sub(1);
                if left < total_len && *grid_ptr.add(left) == region_char {
                    if *visited_ptr.add(left) != TRUE {
                        *visited_ptr.add(left) = TRUE;
                        *stack_top = left;
                        stack_top = stack_top.add(1);
                    }
                } else {
                    perimeter += 1;
                }

                // Right
                let right = current + 1;
                if right < total_len && *grid_ptr.add(right) == region_char {
                    if *visited_ptr.add(right) != TRUE {
                        *visited_ptr.add(right) = TRUE;
                        *stack_top = right;
                        stack_top = stack_top.add(1);
                    }
                } else {
                    perimeter += 1;
                }
            }

            total_price += area * perimeter;
        }

        i += 1;
    }

    total_price
}

#[aoc(day12, part2)]
pub fn part2(input: &str) -> u32 {
    unsafe { part2_impl(input.as_bytes()) }
}

#[allow(static_mut_refs)]
unsafe fn part2_impl(grid: &[u8]) -> u32 {
    TRUE = TRUE.wrapping_add(1);

    let total_len = grid.len();
    let height = compute_height(total_len);
    let line_len = height + 1;

    let grid_ptr = grid.as_ptr();
    let visited_ptr = VISITED.as_mut_ptr();
    let stack_ptr = STACK.as_mut_ptr();
    let mut stack_top = stack_ptr;

    let mut total_price: u32 = 0;

    let directions: [isize; 4] = [-1, 1, -(line_len as isize), line_len as isize];

    let mut i: usize = 0;
    while i < total_len {
        let current_char = *grid_ptr.add(i);

        if current_char != b'\n' && *visited_ptr.add(i) != TRUE {
            *visited_ptr.add(i) = TRUE;
            *stack_top = i;
            stack_top = stack_top.add(1);

            let region_char = current_char;
            let mut area: u32 = 0;
            let mut sides: u32 = 0;

            while stack_top != stack_ptr {
                stack_top = stack_top.sub(1);
                let current = *stack_top;
                area += 1;

                for (d_i, &d) in directions.iter().enumerate() {
                    let neighbor = (current as isize + d) as usize; // Let it wrap

                    let neighbor_char = if neighbor < total_len {
                        *grid_ptr.add(neighbor)
                    } else {
                        b'\n'
                    };

                    if neighbor_char != region_char {
                        let (o1, o2) = if d_i < 2 {
                            // Horizontal
                            let o1 = current.wrapping_sub(line_len);
                            let o2 = neighbor.wrapping_sub(line_len);
                            (o1, o2)
                        } else {
                            // Vertical
                            let o1 = current.wrapping_sub(1);
                            let o2 = neighbor.wrapping_sub(1);
                            (o1, o2)
                        };

                        let o1_char = if o1 < total_len {
                            *grid_ptr.add(o1)
                        } else {
                            b'\n'
                        };

                        let o2_char = if o2 < total_len {
                            *grid_ptr.add(o2)
                        } else {
                            b'\n'
                        };

                        if o1_char != region_char
                            || (o1_char == region_char && o2_char == region_char)
                        {
                            sides += 1;
                        }
                    } else if neighbor < total_len && *visited_ptr.add(neighbor) != TRUE {
                        *visited_ptr.add(neighbor) = TRUE;
                        *stack_top = neighbor;
                        stack_top = stack_top.add(1);
                    }
                }
            }

            total_price += area * sides;
        }

        i += 1;
    }

    total_price
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    const DAY: u8 = 12;
    const INPUT1: &str = "AAAA
BBCD
BBCC
EEEC
";
    const INPUT2: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
";
    const INPUT3: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT1), 140);
        assert_eq!(part1(INPUT2), 772);
        assert_eq!(part1(INPUT3), 1930);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT1), 80);
        assert_eq!(part2(INPUT2), 436);
        assert_eq!(part2(INPUT3), 1206);
    }

    #[test]
    fn test_compare_part1_with_file() {
        let paths = [
            format!("day{DAY}.txt"),
            format!("day{DAY}-alt1.txt"),
            format!("day{DAY}-alt2.txt"),
        ];
        let outputs: [u32; 3] = [1370258, 1489582, 1370258];
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
        let outputs: [u32; 3] = [805814, 914966, 805814];
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
