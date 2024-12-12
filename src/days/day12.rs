use std::collections::HashSet;

use crate::{
    harness::input::RawInput,
    util::{grid::Grid, search::bfs},
};

pub fn solve_part1(input: RawInput) -> usize {
    solve(input, get_perimeter)
}

pub fn solve_part2(input: RawInput) -> usize {
    solve(input, count_sides)
}

fn solve(input: RawInput, get_price: fn(&Grid<u8>, &HashSet<[usize; 2]>) -> usize) -> usize {
    let grid = Grid::parse_bytes(input.as_str());
    let mut added = HashSet::new();
    let mut out = 0;
    for ij in grid.indices() {
        if added.contains(&ij) {
            continue;
        }
        let result = bfs::search(
            ij,
            |&ij| {
                grid.orthogonal_neighbors(ij)
                    .filter(|&ij2| grid[ij2] == grid[ij])
                    .collect::<Vec<_>>()
            },
            |_| false,
        );
        let region = result
            .seen_states
            .iter()
            .map(|state| state.state)
            .collect::<HashSet<_>>();
        for &ij in &region {
            added.insert(ij);
        }
        out += get_price(&grid, &region) * region.len();
    }
    out
}

fn get_perimeter(grid: &Grid<u8>, region: &HashSet<[usize; 2]>) -> usize {
    let mut perimeter = 0;
    for &ij in region {
        perimeter += 4 - grid
            .orthogonal_neighbors(ij)
            .filter(|&ij2| grid[ij2] == grid[ij])
            .count();
    }
    perimeter
}

fn count_sides(_: &Grid<u8>, region: &HashSet<[usize; 2]>) -> usize {
    let mut counted = HashSet::new();
    let mut sides = 0;
    for &ij in region {
        for dir in [[0, 1], [0, usize::MAX], [1, 0], [usize::MAX, 0]] {
            if counted.contains(&(ij, dir)) {
                continue;
            }
            let must_be_empty_dir = [dir[1], 0 - dir[0]];
            let mut edge = vec![];
            for sign in [1, usize::MAX] {
                for d in 0.. {
                    let new_ij = [ij[0] + sign * d * dir[0], ij[1] + sign * d * dir[1]];
                    if !region.contains(&new_ij)
                        || region.contains(&[
                            new_ij[0] + must_be_empty_dir[0],
                            new_ij[1] + must_be_empty_dir[1],
                        ])
                    {
                        break;
                    }
                    edge.push(new_ij);
                }
            }
            if !edge.is_empty() {
                sides += 1;
                for &ij in &edge {
                    counted.insert((ij, dir));
                }
            }
        }
    }
    sides
}
