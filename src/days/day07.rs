use std::ops::{Add, Mul};

use crate::harness::input::RawInput;

pub fn solve_part1(input: RawInput) -> usize {
    solve(input, &[Add::add, Mul::mul])
}

pub fn solve_part2(input: RawInput) -> usize {
    solve(input, &[Add::add, Mul::mul, concat])
}

fn solve(input: RawInput, ops: &[fn(usize, usize) -> usize]) -> usize {
    input
        .per_line(move |line| {
            let (goal, terms) = line.split_once(": ");
            (goal.single::<usize>(), terms.split_whitespace::<usize>())
        })
        .filter(|(goal, terms)| can_make_goal(*goal, terms, ops, 0))
        .map(|(goal, _)| goal)
        .sum()
}

fn can_make_goal(
    goal: usize,
    terms: &[usize],
    ops: &[fn(usize, usize) -> usize],
    current: usize,
) -> bool {
    current == goal
        || (!terms.is_empty()
            && current < goal
            && ops
                .iter()
                .any(|op| can_make_goal(goal, &terms[1..], ops, op(current, terms[0]))))
}

fn concat(a: usize, b: usize) -> usize {
    format!("{a}{b}").parse().unwrap()
}
