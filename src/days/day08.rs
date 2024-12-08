use std::collections::{HashMap, HashSet};

use crate::{harness::input::RawInput, util::grid::Grid};

pub fn solve_part1(input: RawInput) -> usize {
    solve(input, 1..=1)
}

pub fn solve_part2(input: RawInput) -> usize {
    solve(input, 0..)
}

fn solve(input: RawInput, allowed_hops: impl Iterator<Item = usize> + Clone) -> usize {
    let grid = Grid::parse_chars(input.as_str());
    let mut locs_by_char = HashMap::<char, Vec<[usize; 2]>>::new();
    for ij in grid.indices() {
        let c = grid[ij];
        if c != '.' {
            locs_by_char.entry(c).or_default().push(ij);
        }
    }
    let mut antinodes = HashSet::new();
    for locs in locs_by_char.values() {
        if locs.len() > 1 {
            for &ij1 in locs {
                for &ij2 in locs {
                    if ij1 == ij2 {
                        continue;
                    }
                    let [i1, j1] = ij1;
                    let [i2, j2] = ij2;
                    let [d1, d2] = [i2 - i1, j2 - j1];
                    for n in allowed_hops.clone() {
                        let a = [i2 + n * d1, j2 + n * d2];
                        if !grid.is_in_bounds(a) {
                            break;
                        }
                        antinodes.insert(a);
                    }
                }
            }
        }
    }
    antinodes.len()
}
