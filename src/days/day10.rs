use crate::{
    harness::input::RawInput,
    util::{grid::Grid, search::bfs},
};

pub fn solve_part1(input: RawInput) -> usize {
    let grid = Grid::parse_digits(input.as_str());
    grid.indices()
        .filter(|&idx| grid[idx] == 0)
        .map(|trailhead| {
            bfs::search(
                trailhead,
                |&current| {
                    grid.orthogonal_neighbors(current)
                        .filter(|&neighbor| grid[neighbor] == grid[current] + 1)
                        .collect::<Vec<_>>()
                },
                |_| false,
            )
            .seen_states
            .into_iter()
            .filter(|state| grid[state.state] == 9)
            .count()
        })
        .sum()
}

pub fn solve_part2(input: RawInput) -> usize {
    let grid = Grid::parse_digits(input.as_str());
    grid.indices()
        .filter(|&idx| grid[idx] == 0)
        .map(|trailhead| get_counts(&grid, trailhead))
        .sum()
}

fn get_counts(grid: &Grid<usize>, location: [usize; 2]) -> usize {
    if grid[location] == 9 {
        return 1;
    }
    grid.orthogonal_neighbors(location)
        .filter(|&neighbor| grid[neighbor] == grid[location] + 1)
        .map(|neighbor| get_counts(grid, neighbor))
        .sum()
}
