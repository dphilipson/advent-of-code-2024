use std::io::Write;
use std::{collections::HashSet, fs::File};

use crate::{harness::input::RawInput, regex};

pub fn solve_part1(input: RawInput) -> usize {
    let width = 101;
    let height = 103;
    let robots = input
        .per_line(|line| {
            line.parse_with_regex::<(isize, isize, isize, isize)>(regex!(
                r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)"
            ))
        })
        .map(|(px, py, vx, vy)| {
            [
                pos_mod(px + 100 * vx, width),
                pos_mod(py + 100 * vy, height),
            ]
        })
        .collect::<Vec<_>>();
    let mid_x = width / 2;
    let mid_y = height / 2;
    let upleft = robots
        .iter()
        .copied()
        .filter(|&[x, y]| x < mid_x && y < mid_y)
        .count();
    let upright = robots
        .iter()
        .copied()
        .filter(|&[x, y]| x > mid_x && y < mid_y)
        .count();
    let downleft = robots
        .iter()
        .copied()
        .filter(|&[x, y]| x < mid_x && y > mid_y)
        .count();
    let downright = robots
        .iter()
        .copied()
        .filter(|&[x, y]| x > mid_x && y > mid_y)
        .count();
    upleft * upright * downleft * downright
}

pub fn solve_part2(input: RawInput) -> usize {
    let width = 101;
    let height = 103;
    let robots = input
        .per_line(|line| {
            line.parse_with_regex::<(isize, isize, isize, isize)>(regex!(
                r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)"
            ))
        })
        .collect::<Vec<_>>();
    let mut file = File::create("day14-out.txt").unwrap();
    for time in 0..10000 {
        if time % 101 != 77 {
            continue;
        }
        let new_robots = robots
            .iter()
            .map(|&(px, py, vx, vy)| {
                [
                    pos_mod(px + time * vx, width),
                    pos_mod(py + time * vy, height),
                ]
            })
            .collect::<HashSet<_>>();
        writeln!(&mut file, "Time: {time}\n").unwrap();
        for y in 0..height {
            for x in 0..width {
                write!(
                    &mut file,
                    "{}",
                    if new_robots.contains(&[x, y]) {
                        '#'
                    } else {
                        '.'
                    }
                )
                .unwrap();
            }
            writeln!(&mut file).unwrap();
        }
    }
    todo!()
}

fn pos_mod(a: isize, b: isize) -> isize {
    (a % b + b) % b
}
