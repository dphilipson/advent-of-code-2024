use crate::{harness::input::RawInput, regex, util::re};

pub fn solve_part1(input: RawInput) -> usize {
    solve(input, 0)
}

pub fn solve_part2(input: RawInput) -> usize {
    solve(input, 10000000000000)
}

fn solve(input: RawInput, added_position: isize) -> usize {
    let button_regex = regex!(r"Button .: X\+(\d+), Y\+(\d+)");
    let prize_regex = regex!(r"Prize: X=(\d+), Y=(\d+)");
    input
        .as_str()
        .split("\n\n")
        .map(|claw| {
            let mut lines = claw.lines();
            let (ax, ay) =
                re::parse_with_regex::<(isize, isize)>(button_regex, lines.next().unwrap())
                    .unwrap();
            let (bx, by) =
                re::parse_with_regex::<(isize, isize)>(button_regex, lines.next().unwrap())
                    .unwrap();
            let (px, py) =
                re::parse_with_regex::<(isize, isize)>(prize_regex, lines.next().unwrap()).unwrap();
            let (px, py) = (px + added_position, py + added_position);
            let determinant = ax * by - bx * ay;
            if determinant == 0 {
                return 0;
            }
            let numerator_a = by * px - bx * py;
            let numerator_b = -ay * px + ax * py;
            if numerator_a % determinant != 0 || numerator_b % determinant != 0 {
                return 0;
            }
            let a = numerator_a / determinant;
            let b = numerator_b / determinant;
            if a < 0 || b < 0 {
                return 0;
            }
            (3 * a + b) as usize
        })
        .sum()
}
