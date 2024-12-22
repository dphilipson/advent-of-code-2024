use crate::harness::input::RawInput;

const NUMERIC_KEYPAD: &[(char, [isize; 2])] = &[
    ('0', [3, 1]),
    ('1', [2, 0]),
    ('2', [2, 1]),
    ('3', [2, 2]),
    ('4', [1, 0]),
    ('5', [1, 1]),
    ('6', [1, 2]),
    ('7', [0, 0]),
    ('8', [0, 1]),
    ('9', [0, 2]),
    ('A', [3, 2]),
];

pub fn solve_part1(input: RawInput) -> usize {
    solve(input, 2)
}

pub fn solve_part2(input: RawInput) -> usize {
    solve(input, 25)
}

fn solve(input: RawInput, num_outer_robots: usize) -> usize {
    let costs = get_final_costs(num_outer_robots);
    input
        .per_line(move |line| {
            let code = line.as_str();
            let code_number = code[0..3].parse::<usize>().unwrap();
            let chars = code.chars().collect::<Vec<_>>();
            code_number * costs.get_numeric_code_cost(&chars)
        })
        .sum()
}

/// Represents the costs to prep the robot to move in the given direction, i.e.
/// the parent on the necessary arrow key and all its ancestors are on A, and
/// then prep the robot to press the key it's over, i.e. all ancestors on 'A'.
/// For the diagonals, the cost is to have the parent land on each of the
/// diagonal parts. The "hard" diagonals are the ones that require the robot to
/// press the diagonal parts in specifically that order.
///
/// Note that these costs do not include the cost of actually pressing the
/// buttons. So for example if you want to move 1 right, 2 up, you should take
/// `up_right + 3`.
#[derive(Copy, Clone, Debug, Default)]
struct DirectionalCosts {
    right: usize,
    up_right: usize,
    up: usize,
    up_left: usize,
    left: usize,
    down_left: usize,
    down: usize,
    down_right: usize,
    hard_right_up: usize,
    hard_up_left: usize,
    hard_down_left: usize,
    hard_right_down: usize,
}

impl DirectionalCosts {
    fn next_costs(&self) -> Self {
        let hard_right_up = 4 + self.down + self.up_left + self.right;
        let hard_up_left = 6 + self.left + self.hard_down_left + self.hard_right_up;
        let hard_down_left = 6 + self.down_left + self.left + self.hard_right_up;
        let hard_right_down = 4 + self.down + self.left + self.up_right;
        Self {
            right: 2 + self.down + self.up,
            up_right: hard_right_up.min(4 + self.left + self.down_right + self.up),
            up: 2 + self.left + self.right,
            up_left: hard_up_left.min(6 + self.hard_down_left + self.hard_right_up + self.right),
            left: 6 + self.hard_down_left + self.hard_right_up,
            down_left: hard_down_left.min(6 + self.hard_down_left + self.right + self.up_right),
            down: 4 + self.down_left + self.up_right,
            down_right: hard_right_down.min(4 + self.down_left + self.right + self.up),
            hard_right_up,
            hard_up_left,
            hard_down_left,
            hard_right_down,
        }
    }

    fn get_numeric_code_cost(&self, chars: &[char]) -> usize {
        let mut total_cost = 4;
        let mut prev_char = 'A';
        for &c in chars {
            total_cost += self.get_numeric_single_key_cost(prev_char, c);
            prev_char = c;
        }
        total_cost
    }

    fn get_numeric_single_key_cost(&self, from_char: char, to_char: char) -> usize {
        let from_pos = numeric_key_position(from_char);
        let to_pos = numeric_key_position(to_char);
        let di = to_pos[0] - from_pos[0];
        let dj = to_pos[1] - from_pos[1];
        let setup_cost = match (di.signum(), dj.signum()) {
            (0, 1) => self.right,
            (-1, 1) => self.up_right,
            (-1, 0) => self.up,
            (-1, -1) => {
                if from_pos[0] == 3 && to_pos[1] == 0 {
                    // Don't cross empty space.
                    self.hard_up_left
                } else {
                    self.up_left
                }
            }
            (0, -1) => self.left,
            (1, -1) => self.down_left,
            (1, 0) => self.down,
            (1, 1) => {
                if from_pos[1] == 0 && to_pos[0] == 3 {
                    // Don't cross empty space.
                    self.hard_right_down
                } else {
                    self.down_right
                }
            }
            _ => panic!("invalid direction"),
        };
        setup_cost + di.unsigned_abs() + dj.unsigned_abs()
    }
}

fn numeric_key_position(c: char) -> [isize; 2] {
    NUMERIC_KEYPAD
        .iter()
        .find(|(key, _)| *key == c)
        .map(|(_, pos)| *pos)
        .expect("char should be in numeric keypad")
}

fn get_final_costs(num_outer_robots: usize) -> DirectionalCosts {
    let mut costs = DirectionalCosts::default();
    for _ in 0..num_outer_robots {
        costs = costs.next_costs();
    }
    costs
}
