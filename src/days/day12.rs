use std::collections::HashSet;

use crate::{
    harness::input::RawInput,
    util::{grid::Grid, idx2, search::bfs},
};

pub fn solve_part1(input: RawInput) -> usize {
    solve(input, get_perimeter)
}

pub fn solve_part2(input: RawInput) -> usize {
    solve(input, count_corners)
}

fn solve(input: RawInput, get_price: fn(&Grid<u8>, &HashSet<[usize; 2]>) -> usize) -> usize {
    let grid = Grid::parse_bytes(input.as_str());
    let mut added = HashSet::<[usize; 2]>::new();
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
        added.extend(&region);
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

fn count_corners(_: &Grid<u8>, region: &HashSet<[usize; 2]>) -> usize {
    let mut corners = 0;
    for &ij in region {
        for dir in idx2::DIRECTIONS {
            let next_dir = idx2::rotate_counterclockwise(dir);
            let has_side1 = region.contains(&idx2::add(ij, dir));
            let has_side2 = region.contains(&idx2::add(ij, next_dir));
            let has_diagonal = region.contains(&idx2::add(ij, idx2::add(dir, next_dir)));
            let is_convex_corner = !has_side1 && !has_side2;
            let is_concave_corner = has_side1 && has_side2 && !has_diagonal;
            if is_convex_corner || is_concave_corner {
                corners += 1;
            }
        }
    }
    corners
}
