use std::collections::HashMap;

use crate::harness::input::RawInput;

pub fn solve_part1(input: RawInput) -> usize {
    let (mut lefts, mut rights) = get_lists(input);
    lefts.sort();
    rights.sort();
    lefts
        .iter()
        .zip(rights.iter())
        .map(|(&left, &right)| left.abs_diff(right))
        .sum()
}

pub fn solve_part2(input: RawInput) -> usize {
    let (lefts, rights) = get_lists(input);
    let mut right_counts = HashMap::new();
    for right in rights {
        *right_counts.entry(right).or_insert(0) += 1;
    }
    lefts
        .into_iter()
        .map(|left| right_counts.get(&left).copied().unwrap_or_default() * left)
        .sum()
}

fn get_lists(input: RawInput) -> (Vec<usize>, Vec<usize>) {
    let mut lefts = vec![];
    let mut rights = vec![];
    input
        .per_line(|line| line.split_whitespace::<usize>())
        .for_each(|nums| {
            lefts.push(nums[0]);
            rights.push(nums[1]);
        });
    (lefts, rights)
}
