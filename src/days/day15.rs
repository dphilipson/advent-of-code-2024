use std::collections::HashSet;

use crate::{
    harness::input::RawInput,
    util::{grid::Grid, idx2::Idx2Extensions},
};

pub fn solve_part1(input: RawInput) -> usize {
    let (grid, dirs) = input.split_once_on_empty_line();
    let grid = Grid::parse_bytes(grid.as_str());
    let walls = grid
        .indices()
        .filter(|&idx| grid[idx] == b'#')
        .collect::<HashSet<_>>();
    let mut boxes = grid
        .indices()
        .filter(|&idx| grid[idx] == b'O')
        .collect::<HashSet<_>>();
    let mut robot = grid.indices().find(|&idx| grid[idx] == b'@').unwrap();
    let dirs = dirs
        .per_line(|line| {
            line.bytes().into_iter().map(|b| match b {
                b'>' => [0, 1],
                b'^' => [usize::MAX, 0],
                b'<' => [0, usize::MAX],
                b'v' => [1, 0],
                _ => unreachable!(),
            })
        })
        .flatten()
        .collect::<Vec<_>>();

    for &dir in &dirs[0..] {
        let mut loc = robot.add(dir);
        let mut box_count = 0;
        while boxes.contains(&loc) {
            box_count += 1;
            loc = loc.add(dir);
        }
        if walls.contains(&loc) {
            continue;
        }
        robot = robot.add(dir);
        if box_count > 0 {
            boxes.remove(&robot);
            boxes.insert(loc);
        }
    }
    boxes.into_iter().map(|[i, j]| 100 * i + j).sum()
}

pub fn solve_part2(input: RawInput) -> usize {
    let (grid, dirs) = input.split_once_on_empty_line();
    let grid = Grid::parse_bytes(grid.as_str());
    let walls = grid
        .indices()
        .filter(|&idx| grid[idx] == b'#')
        .flat_map(|[i, j]| [[i, 2 * j], [i, 2 * j + 1]])
        .collect::<HashSet<_>>();
    let mut boxes = grid
        .indices()
        .filter(|&idx| grid[idx] == b'O')
        .map(|[i, j]| [i, 2 * j])
        .collect::<HashSet<_>>();
    let mut robot = grid.indices().find(|&idx| grid[idx] == b'@').unwrap();
    robot[1] *= 2;
    let dirs = dirs
        .per_line(|line| {
            line.bytes().into_iter().map(|b| match b {
                b'>' => [0, 1],
                b'^' => [usize::MAX, 0],
                b'<' => [0, usize::MAX],
                b'v' => [1, 0],
                _ => unreachable!(),
            })
        })
        .flatten()
        .collect::<Vec<_>>();
    for dir in dirs {
        if dir[0] == 0 {
            // left-right
            let mut loc = robot.add(dir);
            let mut pushed_boxes = vec![];
            while let Some(overlapping_box) = get_overlapping_box(&boxes, loc) {
                pushed_boxes.push(overlapping_box);
                loc = loc.add(dir.scalar_mul(2));
            }
            if walls.contains(&loc) {
                continue;
            }
            robot = robot.add(dir);
            for &pushed_box in &pushed_boxes {
                boxes.remove(&pushed_box);
                boxes.insert(pushed_box.add(dir));
            }
        } else {
            // up-down
            let Some(first_pushed_box) = get_overlapping_box(&boxes, robot.add(dir)) else {
                if !walls.contains(&robot.add(dir)) {
                    robot = robot.add(dir);
                }
                continue;
            };
            let mut pushed_boxes = HashSet::new();
            let mut just_pushed = HashSet::new();
            pushed_boxes.insert(first_pushed_box);
            just_pushed.insert(first_pushed_box);
            let mut hit_wall = false;
            while !just_pushed.is_empty() {
                let mut new_just_pushed = HashSet::new();
                for &pushed_box in &just_pushed {
                    let pushed_loc = pushed_box.add(dir);
                    let other_pushed_loc = [pushed_loc[0], pushed_loc[1] + 1];
                    if walls.contains(&pushed_loc) || walls.contains(&other_pushed_loc) {
                        hit_wall = true;
                        break;
                    }
                    if let Some(overlapping_box) = get_overlapping_box(&boxes, pushed_loc) {
                        new_just_pushed.insert(overlapping_box);
                        pushed_boxes.insert(overlapping_box);
                    }
                    if let Some(overlapping_box) = get_overlapping_box(&boxes, other_pushed_loc) {
                        new_just_pushed.insert(overlapping_box);
                        pushed_boxes.insert(overlapping_box);
                    }
                }
                if hit_wall {
                    break;
                }
                just_pushed = new_just_pushed;
            }
            if hit_wall {
                continue;
            }
            robot = robot.add(dir);
            let to_add = pushed_boxes
                .iter()
                .map(|&pushed_box| pushed_box.add(dir))
                .collect::<Vec<_>>();
            for &pushed_box in &pushed_boxes {
                boxes.remove(&pushed_box);
            }
            boxes.extend(to_add);
        }
    }
    print_grid(&walls, &boxes, robot);
    boxes.into_iter().map(|[i, j]| 100 * i + j).sum()
}

fn get_overlapping_box(boxes: &HashSet<[usize; 2]>, [i, j]: [usize; 2]) -> Option<[usize; 2]> {
    if boxes.contains(&[i, j]) {
        return Some([i, j]);
    }
    let other_spot = [i, j - 1];
    if boxes.contains(&other_spot) {
        Some(other_spot)
    } else {
        None
    }
}

fn print_grid(walls: &HashSet<[usize; 2]>, boxes: &HashSet<[usize; 2]>, robot: [usize; 2]) {
    let height = walls.iter().map(|&[i, _]| i).max().unwrap() + 1;
    let width = walls.iter().map(|&[_, j]| j).max().unwrap() + 1;
    let mut lines = vec![];
    for _ in 0..height {
        lines.push(vec!['.'; width]);
    }
    for &wall in walls {
        lines[wall[0]][wall[1]] = '#';
    }
    for &bawks in boxes {
        lines[bawks[0]][bawks[1]] = '[';
        lines[bawks[0]][bawks[1] + 1] = ']';
    }
    lines[robot[0]][robot[1]] = '@';
    let out = lines
        .into_iter()
        .map(|line| line.into_iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n");
    println!("{out}\n");
}

// fn get_other_box_spot([i, j]: [usize; 2]) -> [usize; 2] {
//     [i, get_other_column(j)]
// }

// fn get_other_column(j: usize) -> usize {
//     if j % 2 == 0 {
//         j + 1
//     } else {
//         j - 1
//     }
// }
