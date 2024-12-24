use std::{collections::HashMap, convert::Infallible, str::FromStr};

use arrayvec::ArrayString;

use crate::{harness::input::RawInput, regex};

type Name = ArrayString<3>;

#[derive(Copy, Clone, Debug)]
enum Op {
    And,
    Or,
    Xor,
}

impl FromStr for Op {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(Self::And),
            "OR" => Ok(Self::Or),
            "XOR" => Ok(Self::Xor),
            _ => panic!("invalid op: {}", s),
        }
    }
}

impl Op {
    fn eval(self, in1: bool, in2: bool) -> bool {
        match self {
            Self::And => in1 && in2,
            Self::Or => in1 || in2,
            Self::Xor => in1 != in2,
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Gate {
    op: Op,
    in1: Name,
    in2: Name,
    out: Name,
}

pub fn solve_part1(input: RawInput) -> usize {
    let (initial_wires, gates) = parse(input);
    eval(&initial_wires, &gates)
}

pub fn solve_part2(input: RawInput) -> String {
    let (_, gates) = parse(input);
    let mut error_wires = vec![];
    for i in 1..44 {
        if !is_error_index(&gates, i) {
            continue;
        }
        if let Some(wires) = fix_error(&gates, i) {
            error_wires.extend(wires);
        }
    }
    error_wires.sort_unstable();
    error_wires.join(",")
}

fn eval(initial_wires: &HashMap<Name, bool>, gates: &[Gate]) -> usize {
    let mut gates_by_dependency = HashMap::<Name, Vec<usize>>::new();
    for (i, gate) in gates.iter().enumerate() {
        gates_by_dependency.entry(gate.in1).or_default().push(i);
        gates_by_dependency.entry(gate.in2).or_default().push(i);
    }
    let mut wires = HashMap::<Name, bool>::new();
    let mut remaining_dependency_count = vec![2; gates.len()];
    let mut pending = vec![];
    for (&wire, &value) in initial_wires {
        wires.insert(wire, value);
        if let Some(dependents) = gates_by_dependency.get(&wire) {
            for &i in dependents {
                remaining_dependency_count[i] -= 1;
                if remaining_dependency_count[i] == 0 {
                    pending.push(gates[i]);
                }
            }
        }
    }
    while let Some(gate) = pending.pop() {
        let in1 = wires.get(&gate.in1).copied().unwrap();
        let in2 = wires.get(&gate.in2).copied().unwrap();
        let out = gate.op.eval(in1, in2);
        wires.insert(gate.out, out);
        if let Some(dependents) = gates_by_dependency.get(&gate.out) {
            for &i in dependents {
                remaining_dependency_count[i] -= 1;
                if remaining_dependency_count[i] == 0 {
                    pending.push(gates[i]);
                }
            }
        }
    }
    let mut z_wires = wires
        .keys()
        .filter(|&name| name.starts_with("z"))
        .collect::<Vec<_>>();
    z_wires.sort_unstable();
    z_wires.reverse();
    let mut out = 0;
    for wire in z_wires {
        out <<= 1;
        out |= wires[wire] as usize;
    }
    out
}

fn eval_sum(gates: &[Gate], x: usize, y: usize) -> usize {
    let mut initial_wires = HashMap::<Name, bool>::new();
    for (i, bit) in to_bits(x).into_iter().enumerate() {
        initial_wires.insert(wire_name('x', i), bit);
    }
    for (i, bit) in to_bits(y).into_iter().enumerate() {
        initial_wires.insert(wire_name('y', i), bit);
    }
    eval(&initial_wires, gates)
}

fn is_error_index(gates: &[Gate], i: usize) -> bool {
    let x = 1 << i;
    eval_sum(gates, x, 0) != x
}

fn get_wires_at_digit(gates: &[Gate], i: usize) -> Vec<Name> {
    let downstream_of_i = get_downstream(wire_name('x', i), gates);
    let downstream_of_i_plus_1 = get_downstream(wire_name('x', i + 1), gates);
    downstream_of_i
        .into_iter()
        .filter(|&name| !downstream_of_i_plus_1.contains(&name))
        .collect::<Vec<_>>()
}

fn fix_error(gates: &[Gate], error_index: usize) -> Option<[Name; 2]> {
    let wires_to_mess_with = get_wires_at_digit(gates, error_index);
    for i in 0..wires_to_mess_with.len() {
        for j in i + 1..wires_to_mess_with.len() {
            let wire1 = wires_to_mess_with[i];
            let wire2 = wires_to_mess_with[j];
            let mut new_gates = gates.to_vec();
            for gate in &mut new_gates {
                if gate.out == wire1 {
                    gate.out = wire2;
                } else if gate.out == wire2 {
                    gate.out = wire1;
                }
            }
            let mut found_error = false;
            let addends = (0..4).map(|n| n << (error_index - 1));
            for x in addends.clone() {
                for y in addends.clone() {
                    let z = eval_sum(&new_gates, x, y);
                    if x + y != z {
                        found_error = true;
                    }
                }
            }
            if !found_error {
                return Some([wire1, wire2]);
            }
        }
    }
    None
}

fn get_downstream(wire: Name, gates: &[Gate]) -> Vec<Name> {
    let mut downstream = vec![];
    let mut pending = vec![wire];
    while let Some(wire) = pending.pop() {
        for gate in gates {
            if gate.in1 == wire || gate.in2 == wire {
                let out = gate.out;
                if !downstream.contains(&out) {
                    downstream.push(out);
                    pending.push(out);
                }
            }
        }
    }
    downstream
}

fn wire_name(prefix: char, i: usize) -> Name {
    Name::from_str(&format!("{}{:02}", prefix, i)).unwrap()
}

fn to_bits(mut x: usize) -> Vec<bool> {
    let mut out = Vec::with_capacity(45);
    for _ in 0..45 {
        out.push(x & 1 == 1);
        x >>= 1;
    }
    out
}

fn parse(input: RawInput) -> (HashMap<Name, bool>, Vec<Gate>) {
    let (initial_wires, gates) = input.split_once_on_empty_line();
    let initial_wires = initial_wires
        .per_line(|line| {
            let (name, value) = line.split_once(": ");
            let name = name.single::<Name>();
            let value = value.as_str() == "1";
            (name, value)
        })
        .collect();
    let gates = gates
        .per_line(|line| {
            let (in1, op, in2, out) =
                line.parse_with_regex::<(Name, Op, Name, Name)>(regex!(r"(.+) (.+) (.+) -> (.+)"));
            Gate { op, in1, in2, out }
        })
        .collect();
    (initial_wires, gates)
}
