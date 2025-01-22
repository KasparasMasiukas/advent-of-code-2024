use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
enum Op {
    And,
    Or,
    Xor,
}

#[derive(Debug)]
struct Gate {
    in1: usize,
    in2: usize,
    out: usize,
    op: Op,
}

#[aoc(day24, part1)]
pub fn part1(input: &str) -> u64 {
    let (initial_part, gates_part) = input.split_once("\n\n").unwrap();

    let mut wire_names = Vec::new();
    let mut wire_to_index = FxHashMap::default();

    for line in initial_part.lines() {
        let (name, _) = line.split_once(": ").unwrap();
        let name = name.to_string();
        if !wire_to_index.contains_key(&name) {
            wire_to_index.insert(name.clone(), wire_names.len());
            wire_names.push(name);
        }
    }

    let mut gate_defs = Vec::new();
    for line in gates_part.lines() {
        let (left, out) = line.split_once(" -> ").unwrap();
        let out = out.to_string();
        let parts: Vec<&str> = left.split_whitespace().collect();
        let in1 = parts[0].to_string();
        let op = parts[1];
        let in2 = parts[2].to_string();
        gate_defs.push((in1, op, in2, out));
    }

    for (in1, _, in2, out) in &gate_defs {
        let names = [in1, in2, out];
        for name in names {
            if !wire_to_index.contains_key(name) {
                wire_to_index.insert(name.clone(), wire_names.len());
                wire_names.push(name.clone());
            }
        }
    }

    let mut gates = Vec::with_capacity(gate_defs.len());
    for (in1_str, op_str, in2_str, out_str) in gate_defs {
        let in1 = *wire_to_index.get(&in1_str).unwrap();
        let in2 = *wire_to_index.get(&in2_str).unwrap();
        let out = *wire_to_index.get(&out_str).unwrap();
        let op = match op_str {
            "AND" => Op::And,
            "OR" => Op::Or,
            "XOR" => Op::Xor,
            _ => panic!("Invalid op: {}", op_str),
        };
        gates.push(Gate { in1, in2, out, op });
    }

    let mut dependencies: Vec<Vec<usize>> = vec![Vec::new(); wire_names.len()];
    let mut unresolved_counts = vec![2; gates.len()];
    for (gate_idx, gate) in gates.iter().enumerate() {
        dependencies[gate.in1].push(gate_idx);
        dependencies[gate.in2].push(gate_idx);
    }

    let mut wire_values = vec![None; wire_names.len()];
    for line in initial_part.lines() {
        let (name, value_str) = line.split_once(": ").unwrap();
        let value = value_str.parse::<u8>().unwrap();
        let idx = *wire_to_index.get(name).unwrap();
        wire_values[idx] = Some(value);
    }

    let mut resolved_wires = VecDeque::new();
    for (idx, val) in wire_values.iter().enumerate() {
        if val.is_some() {
            resolved_wires.push_back(idx);
        }
    }
    let mut gates_queue = VecDeque::new();

    loop {
        while let Some(wire_idx) = resolved_wires.pop_front() {
            for &gate_idx in &dependencies[wire_idx] {
                let count = &mut unresolved_counts[gate_idx];
                *count -= 1;
                if *count == 0 {
                    gates_queue.push_back(gate_idx);
                }
            }
        }

        if gates_queue.is_empty() {
            break;
        }

        while let Some(gate_idx) = gates_queue.pop_front() {
            let gate = &gates[gate_idx];
            let in1_val = wire_values[gate.in1].unwrap();
            let in2_val = wire_values[gate.in2].unwrap();
            let result = match gate.op {
                Op::And => in1_val & in2_val,
                Op::Or => in1_val | in2_val,
                Op::Xor => in1_val ^ in2_val,
            };
            let out_idx = gate.out;
            if wire_values[out_idx].is_none() {
                wire_values[out_idx] = Some(result);
                resolved_wires.push_back(out_idx);
            }
        }
    }

    let mut result = 0;
    for (idx, name) in wire_names.iter().enumerate() {
        if name.starts_with('z') {
            let value = wire_values[idx].expect("Wire not computed");
            let suffix = &name[1..];
            let pos = suffix.parse::<u32>().unwrap();
            result |= (value as u64) << pos;
        }
    }

    result
}

#[aoc(day24, part2)]
pub fn part2(input: &str) -> String {
    let (_, gates_part) = input.split_once("\n\n").unwrap();
    let mut output_set = FxHashSet::default();

    let gates: Vec<Vec<&str>> = gates_part
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();

    for gate in &gates {
        let left = gate[0];
        let op = gate[1];
        let right = gate[2];
        output_set.insert((left, op));
        output_set.insert((right, op));
    }

    let mut swapped = FxHashSet::default();

    for gate in &gates {
        let left = gate[0];
        let op = gate[1];
        let right = gate[2];
        let to = gate[4];

        match op {
            "AND" => {
                // First AND (x00 and y00) is exempt
                if left != "x00" && right != "x00" {
                    // Check if output is connected to an OR gate
                    if !output_set.contains(&(to, "OR")) {
                        swapped.insert(to.to_string());
                    }
                }
            }
            "OR" => {
                // OR gates outputting to z (except z45) are invalid
                if to.starts_with("z") && to != "z45" {
                    swapped.insert(to.to_string());
                }
                // OR gates connected to another OR are invalid
                if output_set.contains(&(to, "OR")) {
                    swapped.insert(to.to_string());
                }
            }
            "XOR" => {
                // Check if it's a first-level XOR (inputs are x or y)
                let is_first_level = left.starts_with('x') || right.starts_with('x');
                if is_first_level {
                    // Exempt the first XOR (x00 and y00)
                    if left != "x00" && right != "x00" {
                        // Check if output is connected to an XOR gate
                        if !output_set.contains(&(to, "XOR")) {
                            swapped.insert(to.to_string());
                        }
                    }
                } else {
                    // Second-level XOR must output to z
                    if !to.starts_with('z') {
                        swapped.insert(to.to_string());
                    }
                }
            }
            _ => panic!("Unknown gate type: {}", op),
        }
    }

    let mut swapped_list: Vec<String> = swapped.into_iter().collect();
    swapped_list.sort_unstable();

    swapped_list.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    const DAY: u8 = 24;
    const INPUT1: &str = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT1), 2024);
    }

    #[test]
    fn test_compare_part1_with_file() {
        let paths = [format!("day{DAY}.txt")];
        let outputs = [49520947122770];
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
        let paths = [format!("day{DAY}.txt")];
        let outputs = ["gjc,gvm,qjj,qsb,wmp,z17,z26,z39"];
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
