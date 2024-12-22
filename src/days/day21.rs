use std::collections::HashMap;

use crate::{
    harness::input::RawInput,
    util::{grid::Grid, idx2::Idx2Extensions, search::bfs},
};

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
struct State1 {
    inner_robot: [usize; 2],
    middle_robot: [usize; 2],
    upper_robot: [usize; 2],
    i: usize,
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
struct State {
    inner_robot: [usize; 2],
    outer_robots: [[usize; 2]; NUM_OUTER_ROBOTS],
    goal_steps: usize,
}

pub fn solve_part1(input: RawInput) -> usize {
    319 * get_best(&"319A".chars().collect::<Vec<_>>())
        + 985 * get_best(&"985A".chars().collect::<Vec<_>>())
        + 340 * get_best(&"340A".chars().collect::<Vec<_>>())
        + 489 * get_best(&"489A".chars().collect::<Vec<_>>())
        + 964 * get_best(&"964A".chars().collect::<Vec<_>>())
}

pub fn solve_part2(input: RawInput) -> usize {
    todo!("{}", &input.as_str()[..0])
}

const NUM_OUTER_ROBOTS: usize = 2;

fn get_best(goal: &[char]) -> usize {
    let mut input_costs = HashMap::new();
    input_costs.insert('^', 1);
    input_costs.insert('>', 1);
    input_costs.insert('v', 1);
    input_costs.insert('<', 1);
    for _ in 0..NUM_OUTER_ROBOTS - 1 {
        input_costs = get_directional_input_costs(&input_costs);
    }
    let numeric_costs = get_numeric_input_costs(&input_costs);
    goal.iter().map(|&c| numeric_costs[&c]).sum()
}

// Returns a map of the costs to press each button ONCE.
// fn get_next_input_costs(prev_costs: &HashMap<char, usize>) -> HashMap<char, usize> {
//     let mut next_costs = HashMap::new();
//     next_costs.insert('^', prev_costs[&'<'] + prev_costs[&'>'] + 1);
//     next_costs.insert('>', prev_costs[&'v'] + prev_costs[&'^'] + 1);
//     next_costs.insert(
//         'v',
//         prev_costs[&'<'] + prev_costs[&'v'] + prev_costs[&'^'] + prev_costs[&'>'] + 1,
//     );
//     next_costs.insert(
//         '<',
//         prev_costs[&'<'] + prev_costs[&'v'] + prev_costs[&'^'] + prev_costs[&'>'] + 3,
//     );
//     next_costs
// }

fn get_input_cost_for_location(prev_costs: &HashMap<char, usize>, [i, j]: [usize; 2]) -> usize {
    let mut cost = 1;
    if i > 0 {
        cost += prev_costs[&'v'] + prev_costs[&'^'] + 2 * (i - 1);
    }
    if j > 0 {
        cost += prev_costs[&'<'] + prev_costs[&'>'] + 2 * (j - 1);
    }
    cost
}

fn get_directional_input_costs(prev_costs: &HashMap<char, usize>) -> HashMap<char, usize> {
    [('^', [0, 1]), ('>', [1, 0]), ('v', [1, 1]), ('<', [1, 2])]
        .into_iter()
        .map(|(c, [i, j])| (c, get_input_cost_for_location(prev_costs, [i, j])))
        .collect()
}

fn get_numeric_locations() -> HashMap<char, usize> {
    [('0', [0, 1]), ('1')].into_iter().collect()
}

fn get_best2(goal: &[char]) -> usize {
    let inner_keypad =
        Grid::parse_chars("789\n456\n123\n 0A").map(|&c| if c == ' ' { None } else { Some(c) });
    let outer_keypad =
        Grid::parse_chars(" ^A\n<v>").map(|&c| if c == ' ' { None } else { Some(c) });
    let initial_state = State {
        inner_robot: [3, 2],
        outer_robots: [[0, 2]; NUM_OUTER_ROBOTS],
        goal_steps: 0,
    };
    let search_result = bfs::search(
        initial_state,
        |state| {
            let &State {
                inner_robot,
                outer_robots,
                goal_steps,
            } = state;
            let robot_to_activate = outer_robots
                .iter()
                .enumerate()
                .find(|&(_, &robot)| outer_keypad[robot].unwrap() != 'A');
            let mut next_states = vec![];
            // I, Robot
            for neighbor in outer_keypad.orthogonal_neighbors(outer_robots[0]) {
                if outer_keypad[neighbor].is_some() {
                    let mut next_outer_robots = outer_robots;
                    next_outer_robots[0] = neighbor;
                    next_states.push(State {
                        inner_robot,
                        outer_robots: next_outer_robots,
                        goal_steps,
                    });
                }
            }
            if let Some((i, &robot)) = robot_to_activate {
                let key = outer_keypad[robot].unwrap();
                if i == outer_robots.len() - 1 {
                    let next_inner_robot = inner_robot.add(direction_for_char(key));
                    if inner_keypad.is_in_bounds(next_inner_robot)
                        && inner_keypad[next_inner_robot].is_some()
                    {
                        next_states.push(State {
                            inner_robot: next_inner_robot,
                            outer_robots,
                            goal_steps,
                        });
                    }
                } else {
                    let next_robot = outer_robots[i + 1].add(direction_for_char(key));
                    if outer_keypad.is_in_bounds(next_robot) && outer_keypad[next_robot].is_some() {
                        let mut next_outer_robots = outer_robots;
                        next_outer_robots[i + 1] = next_robot;
                        next_states.push(State {
                            inner_robot,
                            outer_robots: next_outer_robots,
                            goal_steps,
                        });
                    }
                }
            } else if inner_keypad[inner_robot].unwrap() == goal[goal_steps] {
                next_states.push(State {
                    inner_robot,
                    outer_robots,
                    goal_steps: goal_steps + 1,
                });
            }
            next_states
        },
        |state| state.goal_steps == 4,
    );
    search_result.goal_state().unwrap().distance
}

fn get_best1(goal: &[char]) -> usize {
    let inner_keypad =
        Grid::parse_chars("789\n456\n123\n 0A").map(|&c| if c == ' ' { None } else { Some(c) });
    let outer_keypad =
        Grid::parse_chars(" ^A\n<v>").map(|&c| if c == ' ' { None } else { Some(c) });
    let search_result = bfs::search(
        State1 {
            inner_robot: [3, 2],
            middle_robot: [0, 2],
            upper_robot: [0, 2],
            i: 0,
        },
        |state| {
            let &State1 {
                inner_robot,
                middle_robot,
                upper_robot,
                i,
            } = state;
            let mut next_states = Vec::new();
            for neighbor in outer_keypad.orthogonal_neighbors(upper_robot) {
                if outer_keypad[neighbor].is_some() {
                    next_states.push(State1 {
                        inner_robot,
                        middle_robot,
                        upper_robot: neighbor,
                        i,
                    });
                }
            }
            let upper_key = outer_keypad[upper_robot].unwrap();
            let middle_key = outer_keypad[middle_robot].unwrap();
            let inner_key = inner_keypad[inner_robot].unwrap();

            match upper_key {
                'A' => match middle_key {
                    'A' => {
                        if inner_key == goal[i] {
                            next_states.push(State1 {
                                inner_robot,
                                middle_robot,
                                upper_robot,
                                i: i + 1,
                            });
                        }
                    }
                    '>' => {
                        let next_inner_robot = inner_robot.add([0, 1]);
                        if inner_keypad.is_in_bounds(next_inner_robot)
                            && inner_keypad[next_inner_robot].is_some()
                        {
                            next_states.push(State1 {
                                inner_robot: next_inner_robot,
                                middle_robot,
                                upper_robot,
                                i,
                            });
                        }
                    }
                    '^' => {
                        let next_inner_robot = inner_robot.add([usize::MAX, 0]);
                        if inner_keypad.is_in_bounds(next_inner_robot)
                            && inner_keypad[next_inner_robot].is_some()
                        {
                            next_states.push(State1 {
                                inner_robot: next_inner_robot,
                                middle_robot,
                                upper_robot,
                                i,
                            });
                        }
                    }
                    '<' => {
                        let next_inner_robot = inner_robot.add([0, usize::MAX]);
                        if inner_keypad.is_in_bounds(next_inner_robot)
                            && inner_keypad[next_inner_robot].is_some()
                        {
                            next_states.push(State1 {
                                inner_robot: next_inner_robot,
                                middle_robot,
                                upper_robot,
                                i,
                            });
                        }
                    }
                    'v' => {
                        let next_inner_robot = inner_robot.add([1, 0]);
                        if inner_keypad.is_in_bounds(next_inner_robot)
                            && inner_keypad[next_inner_robot].is_some()
                        {
                            next_states.push(State1 {
                                inner_robot: next_inner_robot,
                                middle_robot,
                                upper_robot,
                                i,
                            });
                        }
                    }
                    _ => panic!("unexpected middle key: {middle_key}"),
                },
                '>' => {
                    let next_middle_robot = middle_robot.add([0, 1]);
                    if outer_keypad.is_in_bounds(next_middle_robot)
                        && outer_keypad[next_middle_robot].is_some()
                    {
                        next_states.push(State1 {
                            inner_robot,
                            middle_robot: next_middle_robot,
                            upper_robot,
                            i,
                        });
                    }
                }
                '^' => {
                    let next_middle_robot = middle_robot.add([usize::MAX, 0]);
                    if outer_keypad.is_in_bounds(next_middle_robot)
                        && outer_keypad[next_middle_robot].is_some()
                    {
                        next_states.push(State1 {
                            inner_robot,
                            middle_robot: next_middle_robot,
                            upper_robot,
                            i,
                        });
                    }
                }
                '<' => {
                    let next_middle_robot = middle_robot.add([0, usize::MAX]);
                    if outer_keypad.is_in_bounds(next_middle_robot)
                        && outer_keypad[next_middle_robot].is_some()
                    {
                        next_states.push(State1 {
                            inner_robot,
                            middle_robot: next_middle_robot,
                            upper_robot,
                            i,
                        });
                    }
                }
                'v' => {
                    let next_middle_robot = middle_robot.add([1, 0]);
                    if outer_keypad.is_in_bounds(next_middle_robot)
                        && outer_keypad[next_middle_robot].is_some()
                    {
                        next_states.push(State1 {
                            inner_robot,
                            middle_robot: next_middle_robot,
                            upper_robot,
                            i,
                        });
                    }
                }
                _ => panic!("unexpected middle key: {middle_key}"),
            }
            next_states
        },
        |state| state.i == 4,
    );
    search_result.goal_state().unwrap().distance
}

fn direction_for_char(c: char) -> [usize; 2] {
    match c {
        '>' => [0, 1],
        '^' => [usize::MAX, 0],
        '<' => [0, usize::MAX],
        'v' => [1, 0],
        _ => panic!("unexpected direction char: {c}"),
    }
}
