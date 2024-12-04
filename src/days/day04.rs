use crate::{harness::input::RawInput, util::grid::Grid};

const NEG_1: usize = usize::MAX;

pub fn solve_part1(input: RawInput) -> usize {
    let grid = Grid::parse_chars(input.as_str());
    let directions = [[0, 1], [1, 0], [1, 1], [1, NEG_1]];
    let mut count = 0;
    for i in 0..grid.nrows() {
        for j in 0..grid.ncols() {
            for direction in directions {
                if let Some(letters) = get_letters(&grid, [i, j], direction) {
                    if letters == ['X', 'M', 'A', 'S'] || letters == ['S', 'A', 'M', 'X'] {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

pub fn solve_part2(input: RawInput) -> usize {
    let grid = Grid::parse_chars(input.as_str());
    let mut count = 0;
    let directions = [[1, 1], [1, NEG_1]];
    for i in 1..grid.nrows() - 1 {
        for j in 1..grid.ncols() - 1 {
            let is_x = directions.into_iter().all(|direction| {
                let [di, dj] = direction;
                if let Some(letters) =
                    get_letters(&grid, [i.wrapping_sub(di), j.wrapping_sub(dj)], direction)
                {
                    letters == ['M', 'A', 'S'] || letters == ['S', 'A', 'M']
                } else {
                    false
                }
            });
            if is_x {
                count += 1;
            }
        }
    }
    count
}

fn get_letters<const N: usize>(
    grid: &Grid<char>,
    [i, j]: [usize; 2],
    [di, dj]: [usize; 2],
) -> Option<[char; N]> {
    let mut letters: [char; N] = ['\0'; N];
    for (k, letter) in letters.iter_mut().enumerate().take(N) {
        let i = i.wrapping_add(k.wrapping_mul(di));
        let j = j.wrapping_add(k.wrapping_mul(dj));
        if i >= grid.nrows() || j >= grid.ncols() {
            return None;
        }
        *letter = grid[[i, j]];
    }
    Some(letters)
}
