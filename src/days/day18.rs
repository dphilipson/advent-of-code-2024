use std::collections::HashSet;

use crate::{
    harness::input::RawInput,
    util::{coords::Coord2, search::bfs},
};

const WIDTH: usize = 70;
const HEIGHT: usize = 70;

pub fn solve_part1(input: RawInput) -> usize {
    let corrupted = parse_corrupted(input);
    get_path_len(&corrupted[..1024].iter().copied().collect()).unwrap()
}

pub fn solve_part2(input: RawInput) -> usize {
    let corrupted = parse_corrupted(input);
    let mut low = 0;
    let mut high = corrupted.len();
    while low < high - 1 {
        let mid = (low + high) / 2;
        if get_path_len(&corrupted[..mid].iter().copied().collect()).is_some() {
            low = mid;
        } else {
            high = mid;
        }
    }
    println!("{:?}", corrupted[low]);
    todo!()
}

fn parse_corrupted(input: RawInput) -> Vec<Coord2<usize>> {
    input
        .per_line(|line| {
            let (x, y) = line.split_once(",");
            Coord2(x.single(), y.single())
        })
        .collect()
}

fn get_path_len(corrupted: &HashSet<Coord2<usize>>) -> Option<usize> {
    bfs::search(
        Coord2(0, 0),
        |&p| {
            p.orthogonal_neighbors()
                .into_iter()
                .filter(|&n| !corrupted.contains(&n) && n.0 <= 70 && n.1 <= 70)
                .collect::<Vec<_>>()
        },
        |&p| p == Coord2(70, 70),
    )
    .path_to_goal()
    .map(|path| path.len() - 1)
}
