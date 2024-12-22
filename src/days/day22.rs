use std::collections::HashMap;

use crate::harness::input::RawInput;

pub fn solve_part1(input: RawInput) -> usize {
    input
        .per_line(|line| line.single::<usize>())
        .map(|seed| {
            let mut n = seed;
            for _ in 0..2000 {
                n = next_secret(n);
            }
            n
        })
        .sum()
}

pub fn solve_part2(input: RawInput) -> usize {
    let priceses = input
        .per_line(|line| line.single::<usize>())
        .map(|seed| {
            let mut n = seed;
            let mut prices = Vec::new();
            prices.push((n % 10) as isize);
            for _ in 0..2000 {
                n = next_secret(n);
                prices.push((n % 10) as isize);
            }
            prices
        })
        .collect::<Vec<_>>();
    let mut total_bananas_by_seq = HashMap::new();
    for prices in &priceses {
        let mut bananas_by_seq = HashMap::new();
        for i in 0..prices.len() - 5 {
            let seq = [
                prices[i + 1] - prices[i],
                prices[i + 2] - prices[i + 1],
                prices[i + 3] - prices[i + 2],
                prices[i + 4] - prices[i + 3],
            ];
            let bananas = prices[i + 4];
            bananas_by_seq.entry(seq).or_insert(bananas);
        }
        for (seq, bananas) in bananas_by_seq {
            *total_bananas_by_seq.entry(seq).or_insert(0) += bananas;
        }
    }
    total_bananas_by_seq.values().copied().max().unwrap() as usize
}

fn next_secret(mut n: usize) -> usize {
    n ^= (n << 6) % (1 << 24);
    n ^= (n >> 5) % (1 << 24);
    n ^= (n << 11) % (1 << 24);
    n
}
