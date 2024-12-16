mod day16dijkstra;

use std::collections::HashSet;

use crate::{
    harness::input::RawInput,
    util::{
        grid::Grid,
        idx2::Idx2Extensions,
        search::{bfs, dijkstra},
    },
};

pub fn solve_part1(input: RawInput) -> usize {
    let grid = Grid::parse_bytes(input.as_str());
    let walls = grid
        .indices()
        .filter(|&ij| grid[ij] == b'#')
        .collect::<HashSet<_>>();
    let start = grid.indices().find(|&ij| grid[ij] == b'S').unwrap();
    let end = grid.indices().find(|&ij| grid[ij] == b'E').unwrap();

    let result = dijkstra::search(
        (start, [0, 1]),
        |&(ij, dir)| {
            let mut next_states = Vec::new();
            if !walls.contains(&ij.add(dir)) {
                next_states.push(((ij.add(dir), dir), 1));
            }
            for new_dir in [dir.rotate_clockwise(), dir.rotate_counterclockwise()] {
                if !walls.contains(&ij.add(new_dir)) {
                    next_states.push(((ij.add(new_dir), new_dir), 1001));
                }
            }
            next_states
        },
        |&(ij, _)| ij == end,
    );
    result.goal_state().unwrap().distance
}

pub fn solve_part2(input: RawInput) -> usize {
    let grid = Grid::parse_bytes(input.as_str());
    let walls = grid
        .indices()
        .filter(|&ij| grid[ij] == b'#')
        .collect::<HashSet<_>>();
    let start = grid.indices().find(|&ij| grid[ij] == b'S').unwrap();
    let end = grid.indices().find(|&ij| grid[ij] == b'E').unwrap();

    let result = day16dijkstra::search(
        (start, [0, 1]),
        |&(ij, dir)| {
            if ij == end {
                return vec![(([0, 0], [0, 0]), 1)];
            }
            if ij == [0, 0] {
                return vec![(([1, 1], [0, 0]), 1)];
            }
            let mut next_states = Vec::new();
            if !walls.contains(&ij.add(dir)) {
                next_states.push(((ij.add(dir), dir), 1));
            }
            for new_dir in [dir.rotate_clockwise(), dir.rotate_counterclockwise()] {
                if !walls.contains(&ij.add(new_dir)) {
                    next_states.push(((ij.add(new_dir), new_dir), 1001));
                }
            }
            next_states
        },
        |&state| state == ([1, 1], [0, 0]),
    );

    let mut on_optimal_paths = HashSet::new();
    bfs::search(
        result.seen_states.len() - 2,
        |&index| {
            let state = &result.seen_states[index];
            let ij = state.state.0;
            on_optimal_paths.insert(ij);
            state.prev_indices.clone()
        },
        |_| false,
    );
    on_optimal_paths.len() - 1
}
