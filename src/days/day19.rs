use crate::harness::input::RawInput;

pub fn solve_part1(input: RawInput) -> usize {
    let (towels, goals) = parse(input);
    goals
        .into_iter()
        .filter(|goal| count_ways_to_goal(&towels, goal, 0, &mut vec![None; goal.len()]) > 0)
        .count()
}

pub fn solve_part2(input: RawInput) -> usize {
    let (towels, goals) = parse(input);
    goals
        .into_iter()
        .map(|goal| count_ways_to_goal(&towels, &goal, 0, &mut vec![None; goal.len()]))
        .sum()
}

fn parse(input: RawInput) -> (Vec<Vec<char>>, Vec<Vec<char>>) {
    let (towels, goals) = input.split_once_on_empty_line();
    let towels = towels
        .single_line(|line| line.split::<String>(", "))
        .into_iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect();
    let goals = goals.per_line(|line| line.chars()).collect();
    (towels, goals)
}

fn count_ways_to_goal(
    towels: &[Vec<char>],
    goal: &[char],
    i: usize,
    cache: &mut [Option<usize>],
) -> usize {
    if i == goal.len() {
        return 1;
    }
    if let Some(cached) = cache[i] {
        return cached;
    }
    let mut ways = 0;
    for towel in towels {
        if is_prefix(towel, &goal[i..]) {
            ways += count_ways_to_goal(towels, goal, i + towel.len(), cache);
        }
    }
    cache[i] = Some(ways);
    ways
}

fn is_prefix(a: &[char], b: &[char]) -> bool {
    a.len() <= b.len() && a.iter().zip(b).all(|(a, b)| a == b)
}
