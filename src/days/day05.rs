use std::collections::HashSet;

use crate::{harness::input::RawInput, regex};

pub fn solve_part1(input: RawInput) -> usize {
    let (rules, updates) = parse(input);
    updates
        .iter()
        .filter(|pages| is_valid(pages, &rules))
        .map(|pages| pages[pages.len() / 2])
        .sum()
}

pub fn solve_part2(input: RawInput) -> usize {
    let (rules, updates) = parse(input);
    updates
        .into_iter()
        .filter(|pages| !is_valid(pages, &rules))
        .map(|mut pages| {
            let mut found_error = true;
            while found_error {
                found_error = false;
                for i in 0..pages.len() {
                    for j in i + 1..pages.len() {
                        if rules.contains(&(pages[j], pages[i])) {
                            pages.swap(i, j);
                            found_error = true;
                        }
                    }
                }
            }
            pages
        })
        .map(|pages| pages[pages.len() / 2])
        .sum()
}

fn parse(input: RawInput) -> (HashSet<(usize, usize)>, Vec<Vec<usize>>) {
    let (rules, updates) = input.split_once_on_empty_line();
    let rules = rules
        .per_line(|line| line.parse_with_regex(regex!(r"(\d+)\|(\d+)")))
        .collect();
    let updates = updates.per_line(|line| line.split(",")).collect();
    (rules, updates)
}

fn is_valid(pages: &[usize], rules: &HashSet<(usize, usize)>) -> bool {
    for i in 0..pages.len() {
        for j in i + 1..pages.len() {
            if rules.contains(&(pages[j], pages[i])) {
                return false;
            }
        }
    }
    true
}
