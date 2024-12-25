use crate::{harness::input::RawInput, util::grid::Grid};

pub fn solve_part1(input: RawInput) -> usize {
    let mut locks = vec![];
    let mut keys = vec![];
    input.as_str().split("\n\n").for_each(|group| {
        let grid = Grid::parse_chars(group);
        if grid[[0, 0]] == '#' {
            let mut lock = [0; 5];
            for j in 0..5 {
                lock[j] = (1..).take_while(|&i| grid[[i, j]] == '#').count();
            }
            locks.push(lock);
        } else {
            let mut key = [0; 5];
            for j in 0..5 {
                key[j] = 5 - (1..).take_while(|&i| grid[[i, j]] == '.').count();
            }
            keys.push(key);
        }
    });
    let mut count = 0;
    for lock in locks {
        for &key in &keys {
            let can_fit = (0..5).all(|j| key[j] + lock[j] <= 5);
            if can_fit {
                count += 1;
            }
        }
    }
    count
}

pub fn solve_part2(input: RawInput) -> usize {
    todo!("{}", &input.as_str()[..0])
}
