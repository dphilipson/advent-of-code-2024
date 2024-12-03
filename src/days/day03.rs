use crate::{harness::input::RawInput, regex};

pub fn solve_part1(input: RawInput) -> usize {
    let re = regex!(r"mul\((\d+),(\d+)\)");
    re.captures_iter(input.as_str())
        .map(|m| m[1].parse::<usize>().unwrap() * m[2].parse::<usize>().unwrap())
        .sum()
}

pub fn solve_part2(input: RawInput) -> usize {
    let re = regex!(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)");
    let mut enabled = true;
    let mut sum = 0;
    re.captures_iter(input.as_str()).for_each(|m| {
        if m[0].starts_with("do(") {
            enabled = true;
        } else if m[0].starts_with("don't(") {
            enabled = false;
        } else if enabled {
            sum += m[1].parse::<usize>().unwrap() * m[2].parse::<usize>().unwrap();
        }
    });
    sum
}
