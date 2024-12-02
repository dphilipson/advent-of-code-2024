use crate::harness::input::RawInput;

pub fn solve_part1(input: RawInput) -> usize {
    input
        .per_line(|line| line.split_whitespace::<usize>())
        .filter(|nums| is_safe(nums))
        .count()
}

pub fn solve_part2(input: RawInput) -> usize {
    input
        .per_line(|line| line.split_whitespace::<usize>())
        .filter(|nums| {
            (0..nums.len()).any(|i| {
                let mut nums = nums.clone();
                nums.remove(i);
                is_safe(&nums)
            })
        })
        .count()
}

fn is_safe(nums: &[usize]) -> bool {
    let is_increasing = nums[0] < nums[1];
    nums.iter()
        .zip(nums.iter().skip(1))
        .all(|(&a, &b)| (a < b) == is_increasing && (1..=3).contains(&a.abs_diff(b)))
}
