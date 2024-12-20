use std::collections::HashMap;

use crate::{
    harness::input::RawInput,
    util::{grid::Grid, search::bfs},
};

pub fn solve_part1(input: RawInput) -> usize {
    solve(input, 2)
}

pub fn solve_part2(input: RawInput) -> usize {
    solve(input, 20)
}

pub fn solve(input: RawInput, max_cheat_time: usize) -> usize {
    let grid = Grid::parse_chars(input.as_str());
    let start = grid.indices().find(|&idx| grid[idx] == 'S').unwrap();
    let end = grid.indices().find(|&idx| grid[idx] == 'E').unwrap();
    let distances_from_start = get_shortest_distances(&grid, start);
    let distances_from_end = get_shortest_distances(&grid, end);
    let no_cheat_best = distances_from_start[&end];
    let mut count = 0;
    for ij1 in grid.indices() {
        if grid[ij1] == '#' {
            continue;
        }
        for i2 in
            ij1[0].saturating_sub(max_cheat_time)..=(ij1[0] + max_cheat_time).min(grid.nrows() - 1)
        {
            for j2 in ij1[1].saturating_sub(max_cheat_time)
                ..=(ij1[1] + max_cheat_time).min(grid.ncols() - 1)
            {
                let ij2 = [i2, j2];
                if grid[ij2] == '#' {
                    continue;
                }
                let cheat_duration = ij1[0].abs_diff(ij2[0]) + ij1[1].abs_diff(ij2[1]);
                if cheat_duration > max_cheat_time {
                    continue;
                }
                let cheated_best =
                    distances_from_start[&ij1] + distances_from_end[&ij2] + cheat_duration;
                if cheated_best + 100 <= no_cheat_best {
                    count += 1;
                }
            }
        }
    }
    count
}

fn get_shortest_distances(grid: &Grid<char>, start: [usize; 2]) -> HashMap<[usize; 2], usize> {
    bfs::search(
        start,
        |&ij| {
            grid.orthogonal_neighbors(ij)
                .filter(|&idx| grid[idx] != '#')
                .collect::<Vec<_>>()
        },
        |_| false,
    )
    .seen_states
    .iter()
    .map(|seen| (seen.state, seen.distance))
    .collect()
}
