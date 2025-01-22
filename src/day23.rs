#[aoc(day23, part1)]
pub fn part1(input: &str) -> usize {
    unsafe { part1_impl(input.as_bytes()) }
}

unsafe fn part1_impl(input: &[u8]) -> usize {
    #[inline(always)]
    unsafe fn set_edge(adj: *mut u8, i: u16, j: u16) {
        let idx = i as usize * 676 + j as usize;
        let byte = adj.add(idx >> 3);
        let mask = 1 << (idx & 7);
        *byte |= mask;
    }
    #[inline(always)]
    unsafe fn test_edge(adj: *const u8, i: u16, j: u16) -> bool {
        let idx = i as usize * 676 + j as usize;
        let byte = *adj.add(idx >> 3);
        let mask = 1 << (idx & 7);
        (byte & mask) != 0
    }
    #[inline(always)]
    unsafe fn set_used(u: *mut u8, i: u16) {
        let b = u.add((i >> 3) as usize);
        let m = 1 << (i & 7);
        *b |= m;
    }
    #[inline(always)]
    unsafe fn test_used(u: *const u8, i: u16) -> bool {
        let b = *u.add((i >> 3) as usize);
        let m = 1 << (i & 7);
        (b & m) != 0
    }

    let mut adj = [0u8; 456_976 / 8];
    let mut used = [0u8; 676 / 8 + 1];
    let mut p = input.as_ptr();
    let end = p.add(input.len());
    while p < end {
        if end.offset_from(p) < 5 {
            break;
        }
        let n1 = parse_node(&mut p);
        p = p.add(1);
        let n2 = parse_node(&mut p);
        set_edge(adj.as_mut_ptr(), n1, n2);
        set_edge(adj.as_mut_ptr(), n2, n1);
        set_used(used.as_mut_ptr(), n1);
        set_used(used.as_mut_ptr(), n2);
        if p < end && *p == b'\n' {
            p = p.add(1);
        }
    }
    let mut nodes = [0u16; 676];
    let mut count = 0;
    for i in 0..676 {
        if test_used(used.as_ptr(), i) {
            nodes[count] = i;
            count += 1;
        }
    }
    let mut result = 0;
    for i in 0..count {
        let ni = nodes[i];
        for j in (i + 1)..count {
            let nj = nodes[j];
            if !test_edge(adj.as_ptr(), ni, nj) {
                continue;
            }
            for k in (j + 1)..count {
                let nk = nodes[k];
                if test_edge(adj.as_ptr(), ni, nk) && test_edge(adj.as_ptr(), nj, nk) {
                    if ni / 26 == 19 || nj / 26 == 19 || nk / 26 == 19 {
                        result += 1;
                    }
                }
            }
        }
    }
    result
}

#[aoc(day23, part2)]
pub fn part2(input: &str) -> &'static str {
    unsafe { part2_impl(input.as_bytes()) }
}

static mut BUFFER: [u8; 1024] = [0; 1024];

type BitSet = [u8; 85];

unsafe fn adjacency_set<'a>(adjacency_sets: *mut u8, i: u16) -> &'a mut BitSet {
    let offset = i as usize * 85;
    let ptr = adjacency_sets.add(offset);
    &mut *(ptr as *mut BitSet)
}

#[allow(static_mut_refs)]
unsafe fn part2_impl(input: &[u8]) -> &'static str {
    let mut adj = [0u8; 456_976 / 8]; // 676*676 bits = 456,976 bits => /8 bytes
    let mut used = [0u8; 676 / 8 + 1];
    let mut p = input.as_ptr();
    let end = p.add(input.len());

    while p < end {
        if end.offset_from(p) < 5 {
            break;
        }
        let n1 = parse_node(&mut p);
        p = p.add(1);
        let n2 = parse_node(&mut p);

        set_edge(adj.as_mut_ptr(), n1, n2);
        set_edge(adj.as_mut_ptr(), n2, n1);

        set_used(used.as_mut_ptr(), n1);
        set_used(used.as_mut_ptr(), n2);

        if p < end && *p == b'\n' {
            p = p.add(1);
        }
    }

    static mut ADJ_SETS_BUF: [u8; 676 * 85] = [0; 676 * 85];
    ADJ_SETS_BUF.fill(0);

    let mut used_nodes = [0u16; 676];
    let mut used_count = 0;
    for i in 0..676 {
        if test_used(used.as_ptr(), i) {
            used_nodes[used_count] = i;
            used_count += 1;
        }
    }

    for ui in 0..used_count {
        let i = used_nodes[ui];
        let i_adj = adjacency_set(ADJ_SETS_BUF.as_mut_ptr(), i);
        for uj in 0..used_count {
            let j = used_nodes[uj];
            if test_edge(adj.as_ptr(), i, j) {
                set_bit(i_adj, j);
            }
        }
    }

    let mut p = [0u8; 85];
    for ui in 0..used_count {
        let i = used_nodes[ui];
        set_bit(&mut p, i);
    }
    let mut x = [0u8; 85];
    let mut r = Vec::with_capacity(676);

    let mut best_clique = Vec::new();
    best_clique.reserve(676);

    bron_kerbosch_pivot(
        &mut r,
        &mut p,
        &mut x,
        ADJ_SETS_BUF.as_ptr(),
        &mut best_clique,
    );

    best_clique.sort_unstable();

    let mut idx = 0;
    for (i, node_id) in best_clique.iter().enumerate() {
        if i > 0 {
            BUFFER[idx] = b',';
            idx += 1;
        }
        let (c1, c2) = decode_node(*node_id);
        BUFFER[idx] = c1;
        BUFFER[idx + 1] = c2;
        idx += 2;
    }

    core::str::from_utf8_unchecked(&BUFFER[..idx])
}

unsafe fn bron_kerbosch_pivot(
    r: &mut Vec<u16>,
    p: &mut BitSet,
    x: &mut BitSet,
    adjacency_sets: *const u8,
    best_clique: &mut Vec<u16>,
) {
    if bitset_is_empty(p) && bitset_is_empty(x) {
        if r.len() > best_clique.len() {
            best_clique.clear();
            best_clique.extend_from_slice(r);
        }
        return;
    }

    // Choose a pivot u from p ∪ x (just pick the first bit we find).
    let mut px = *p;
    bitset_or(&mut px, x);
    let u = bitset_first_set(&px);
    if u == 0xFFFF {
        return;
    }

    let mut temp = *p;
    let u_neighbors = adjacency_set(adjacency_sets as *mut u8, u);
    bitset_not_in_place(&mut temp, u_neighbors);

    // For each v in temp ( = p \ N(u) )
    let mut v = bitset_first_set(&temp);
    while v != 0xFFFF {
        // r ∪ {v}
        r.push(v);

        // new_p = p ∩ N(v)
        let mut new_p = *p;
        let v_neighbors = adjacency_set(adjacency_sets as *mut u8, v);
        bitset_and(&mut new_p, v_neighbors);

        // new_x = x ∩ N(v)
        let mut new_x = *x;
        bitset_and(&mut new_x, v_neighbors);

        bron_kerbosch_pivot(r, &mut new_p, &mut new_x, adjacency_sets, best_clique);

        r.pop();
        bitset_clear_bit(p, v);
        bitset_set_bit(x, v);

        v = bitset_next_set(&temp, v + 1);
    }
}

#[inline(always)]
unsafe fn set_edge(adj: *mut u8, i: u16, j: u16) {
    let idx = i as usize * 676 + j as usize;
    let byte = adj.add(idx >> 3);
    let mask = 1 << (idx & 7);
    *byte |= mask;
}

#[inline(always)]
unsafe fn test_edge(adj: *const u8, i: u16, j: u16) -> bool {
    let idx = i as usize * 676 + j as usize;
    let byte = *adj.add(idx >> 3);
    let mask = 1 << (idx & 7);
    (byte & mask) != 0
}

#[inline(always)]
unsafe fn set_used(u: *mut u8, i: u16) {
    let b = u.add((i >> 3) as usize);
    let m = 1 << (i & 7);
    *b |= m;
}

#[inline(always)]
unsafe fn test_used(u: *const u8, i: u16) -> bool {
    let b = *u.add((i >> 3) as usize);
    let m = 1 << (i & 7);
    (b & m) != 0
}

#[inline(always)]
fn set_bit(bits: &mut BitSet, j: u16) {
    let byte_index = j as usize >> 3;
    let bit_index = j & 7;
    bits[byte_index] |= 1 << bit_index;
}

#[inline(always)]
fn bitset_clear_bit(bits: &mut BitSet, j: u16) {
    let byte_index = j as usize >> 3;
    let bit_index = j & 7;
    bits[byte_index] &= !(1 << bit_index);
}

#[inline(always)]
fn bitset_set_bit(bits: &mut BitSet, j: u16) {
    set_bit(bits, j);
}

#[inline(always)]
fn bitset_test_bit(bits: &BitSet, j: u16) -> bool {
    let byte_index = j as usize >> 3;
    let bit_index = j & 7;
    (bits[byte_index] & (1 << bit_index)) != 0
}

#[inline(always)]
fn bitset_and(dest: &mut BitSet, src: &BitSet) {
    for i in 0..85 {
        dest[i] &= src[i];
    }
}

#[inline(always)]
fn bitset_or(dest: &mut BitSet, src: &BitSet) {
    for i in 0..85 {
        dest[i] |= src[i];
    }
}

#[inline(always)]
fn bitset_not_in_place(dest: &mut BitSet, remove: &BitSet) {
    for i in 0..85 {
        dest[i] &= !remove[i];
    }
}

#[inline(always)]
fn bitset_is_empty(bits: &BitSet) -> bool {
    bits.iter().all(|&b| b == 0)
}

#[inline(always)]
fn bitset_first_set(bits: &BitSet) -> u16 {
    for (i, byte) in bits.iter().enumerate() {
        if *byte != 0 {
            let b = *byte;
            let tz = b.trailing_zeros() as u16;
            let index = i as u16 * 8 + tz;
            return index;
        }
    }
    0xFFFF
}

#[inline(always)]
fn bitset_next_set(bits: &BitSet, start: u16) -> u16 {
    let start_byte = (start >> 3) as usize;
    let start_bit = start & 7;
    if start_byte >= 85 {
        return 0xFFFF;
    }

    let mut b = bits[start_byte] >> start_bit;
    if b != 0 {
        let tz = b.trailing_zeros() as u16;
        return (start_byte as u16) * 8 + start_bit + tz;
    }

    for i in (start_byte + 1)..85 {
        if bits[i] != 0 {
            let tz = bits[i].trailing_zeros() as u16;
            return i as u16 * 8 + tz;
        }
    }
    0xFFFF
}

#[inline(always)]
fn decode_node(n: u16) -> (u8, u8) {
    let a = n / 26;
    let b = n % 26;
    ((a as u8) + b'a', (b as u8) + b'a')
}

#[inline(always)]
unsafe fn parse_node(ptr: *mut *const u8) -> u16 {
    let p = *ptr;
    let a = *p - b'a';
    let b = *p.add(1) - b'a';
    *ptr = p.add(2);
    (a as u16) * 26 + (b as u16)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    const DAY: u8 = 23;
    const INPUT: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), "co,de,ka,ta");
    }

    #[test]
    fn test_compare_part1_with_file() {
        let paths = [format!("day{DAY}.txt")];
        let outputs = [1437];
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
        let outputs = ["da,do,gx,ly,mb,ns,nt,pz,sc,si,tp,ul,vl"];
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
