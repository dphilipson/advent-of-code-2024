use crate::harness::input::RawInput;

pub fn solve_part1(input: RawInput) -> u64 {
    let (mut lefts, mut rights) = get_lists(input);
    lefts.sort();
    rights.sort();
    lefts
        .iter()
        .zip(rights.iter())
        .map(|(&left, &right)| left.abs_diff(right))
        .sum()
}

pub fn solve_part2(input: RawInput) -> u64 {
    let (lefts, rights) = get_lists(input);
    lefts
        .into_iter()
        .map(|left| rights.iter().filter(|&&right| right == left).count() as u64 * left)
        .sum()
}

fn get_lists(input: RawInput) -> (Vec<u64>, Vec<u64>) {
    let mut lefts = vec![];
    let mut rights = vec![];
    input
        .per_line(|line| line.split_whitespace::<u64>())
        .for_each(|nums| {
            lefts.push(nums[0]);
            rights.push(nums[1]);
        });
    (lefts, rights)
}
