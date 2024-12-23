use std::collections::{HashMap, HashSet};

use arrayvec::ArrayString;

use crate::{harness::input::RawInput, util::search::bfs};

type Name = ArrayString<2>;

pub fn solve_part1(input: RawInput) -> usize {
    let neighbors = parse_neighbors(input);
    let names = Vec::from_iter(neighbors.keys());
    let mut count = 0;
    for i1 in 0..names.len() {
        for i2 in i1 + 1..names.len() {
            for i3 in i2 + 1..names.len() {
                let name1 = names[i1];
                let name2 = names[i2];
                let name3 = names[i3];
                if (name1.starts_with("t") || name2.starts_with("t") || name3.starts_with("t"))
                    && neighbors[name1].contains(name2)
                    && neighbors[name2].contains(name3)
                    && neighbors[name3].contains(name1)
                {
                    count += 1;
                }
            }
        }
    }
    count
}

pub fn solve_part2(input: RawInput) -> String {
    let neighbors = parse_neighbors(input);
    let mut largest_group = vec![];
    for &name in neighbors.keys() {
        let result = bfs::search(
            vec![name],
            |group| {
                let &last = group.last().unwrap();
                neighbors[&last]
                    .iter()
                    .filter(|&&n| {
                        last < n
                            && group
                                .iter()
                                .all(|in_group| neighbors[in_group].contains(&n))
                    })
                    .map(|&n| {
                        let mut new_group = group.clone();
                        new_group.push(n);
                        new_group
                    })
                    .collect::<Vec<_>>()
            },
            |_| false,
        );
        for seen_state in result.seen_states {
            if seen_state.state.len() > largest_group.len() {
                largest_group = seen_state.state;
            }
        }
    }
    largest_group.join(",")
}

fn parse_neighbors(input: RawInput) -> HashMap<Name, HashSet<Name>> {
    let mut neighbors = HashMap::new();
    input
        .per_line(|line| {
            let (a, b) = line.split_once("-");
            (
                Name::from(a.as_str()).unwrap(),
                Name::from(b.as_str()).unwrap(),
            )
        })
        .for_each(|(name1, name2)| {
            neighbors
                .entry(name1)
                .or_insert_with(HashSet::new)
                .insert(name2);
            neighbors
                .entry(name2)
                .or_insert_with(HashSet::new)
                .insert(name1);
        });
    neighbors
}
