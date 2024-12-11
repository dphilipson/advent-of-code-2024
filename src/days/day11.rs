use std::collections::HashMap;

use crate::harness::input::RawInput;

pub fn solve_part1(input: RawInput) -> usize {
    solve(input, 25)
}

pub fn solve_part2(input: RawInput) -> usize {
    solve(input, 75)
}

fn solve(input: RawInput, times: usize) -> usize {
    let stones = input.single_line(|line| line.split_whitespace::<usize>());
    let mut cache = HashMap::new();
    stones
        .into_iter()
        .map(|n| get_count(n, times, &mut cache))
        .sum()
}

fn get_count(n: usize, times: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
    if times == 0 {
        return 1;
    }
    if let Some(&count) = cache.get(&(n, times)) {
        return count;
    }
    let count = if n == 0 {
        get_count(1, times - 1, cache)
    } else if let Some([a, b]) = split_stone(n) {
        get_count(a, times - 1, cache) + get_count(b, times - 1, cache)
    } else {
        get_count(n * 2024, times - 1, cache)
    };
    cache.insert((n, times), count);
    count
}

fn split_stone(n: usize) -> Option<[usize; 2]> {
    let s = n.to_string();
    if s.len() % 2 == 0 {
        let (a, b) = s.split_at(s.len() / 2);
        Some([a.parse().unwrap(), b.parse().unwrap()])
    } else {
        None
    }
}
