use std::{collections::HashSet, hash::Hash};

use crate::{harness::input::RawInput, util::grid::Grid};

const NEG_1: usize = -1_isize as usize;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct State {
    pos: [usize; 2],
    dir: [usize; 2],
}

pub fn solve_part1(input: RawInput) -> usize {
    let (grid, walls, state) = get_initial_state(input);
    get_visited(&grid, &walls, state).len()
}

pub fn solve_part2(input: RawInput) -> usize {
    let (grid, walls, initial_state) = get_initial_state(input);
    let mut count = 0;
    for ij in get_visited(&grid, &walls, initial_state) {
        let mut state = initial_state;
        let mut seen_states: HashSet<State> = HashSet::new();
        let mut walls = walls.clone();
        walls.insert(ij);
        loop {
            if seen_states.contains(&state) {
                count += 1;
                break;
            }
            seen_states.insert(state);

            let Some(next_state) = get_next_state(&grid, &walls, state) else {
                break;
            };
            state = next_state;
        }
    }

    count
}

fn get_initial_state(input: RawInput) -> (Grid<char>, HashSet<[usize; 2]>, State) {
    let grid = Grid::parse_chars(input.as_str());
    let pos = grid.indices().find(|&pos| grid[pos] == '^').unwrap();
    let walls = grid.indices().filter(|&pos| grid[pos] == '#').collect();
    (
        grid,
        walls,
        State {
            pos,
            dir: [NEG_1, 0],
        },
    )
}

fn get_next_state(grid: &Grid<char>, walls: &HashSet<[usize; 2]>, state: State) -> Option<State> {
    let State { pos, dir } = state;
    let next_pos = [pos[0] + dir[0], pos[1] + dir[1]];
    if walls.contains(&next_pos) {
        let dir = [dir[1], -(dir[0] as isize) as usize];
        Some(State { pos, dir })
    } else if next_pos[0] < grid.nrows() && next_pos[1] < grid.ncols() {
        Some(State { pos: next_pos, dir })
    } else {
        None
    }
}

fn get_visited(
    grid: &Grid<char>,
    walls: &HashSet<[usize; 2]>,
    mut state: State,
) -> HashSet<[usize; 2]> {
    let mut visited = HashSet::new();
    loop {
        visited.insert(state.pos);
        let Some(next_state) = get_next_state(grid, walls, state) else {
            return visited;
        };
        state = next_state;
    }
}
