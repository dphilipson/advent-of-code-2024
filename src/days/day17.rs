use crate::harness::input::RawInput;

#[derive(Debug, Clone)]
struct State {
    a: usize,
    b: usize,
    c: usize,
    instructions: Vec<u8>,
    instruction_pointer: usize,
    out: Vec<u8>,
    failed: bool,
}

impl State {
    fn update(&mut self) -> bool {
        if self.instruction_pointer >= self.instructions.len() {
            return false;
        }
        let instruction = self.instructions[self.instruction_pointer];
        let operand = self.instructions[self.instruction_pointer + 1];
        let Some(combo) = self.combo(operand) else {
            self.failed = true;
            return false;
        };
        match instruction {
            0 => {
                self.a >>= combo;
            }
            1 => {
                self.b ^= operand as usize;
            }
            2 => {
                self.b = combo % 8;
            }
            3 => {
                if self.a != 0 {
                    self.instruction_pointer = operand as usize - 2;
                }
            }
            4 => {
                self.b ^= self.c;
            }
            5 => {
                self.out.push((combo % 8) as u8);
            }
            6 => {
                self.b = self.a >> combo;
            }
            7 => {
                self.c = self.a >> combo;
            }
            _ => panic!("Invalid instruction {instruction}"),
        }
        self.instruction_pointer += 2;
        true
    }

    fn combo(&self, o: u8) -> Option<usize> {
        match o {
            0 => Some(0),
            1 => Some(1),
            2 => Some(2),
            3 => Some(3),
            4 => Some(self.a),
            5 => Some(self.b),
            6 => Some(self.c),
            7 => None,
            _ => panic!("Invalid combo operand {o}"),
        }
    }

    fn parse(input: RawInput) -> Self {
        let mut lines = input.as_str().lines();
        let a = lines
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .parse()
            .unwrap();
        let b = lines
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .parse()
            .unwrap();
        let c = lines
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .parse()
            .unwrap();
        lines.next().unwrap();
        let instructions = lines
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split(',')
            .map(|c| c.parse().unwrap())
            .collect();
        Self {
            a,
            b,
            c,
            instructions,
            instruction_pointer: 0,
            out: Vec::new(),
            failed: false,
        }
    }

    fn get_output(mut self) -> Option<Vec<u8>> {
        while self.update() {}
        if self.failed {
            None
        } else {
            Some(self.out)
        }
    }
}

pub fn solve_part1(input: RawInput) -> String {
    State::parse(input)
        .get_output()
        .unwrap()
        .into_iter()
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

pub fn solve_part2(input: RawInput) -> usize {
    let state = State::parse(input);
    get_best_input(state.clone(), state.instructions.len() - 1, 0).unwrap()
}

fn get_best_input(mut state: State, i: usize, acc: usize) -> Option<usize> {
    for a_lowest_bits in 0..8 {
        state.a = 8 * acc + a_lowest_bits;
        let Some(out) = state.clone().get_output() else {
            continue;
        };
        if out == state.instructions[i..] {
            if i == 0 {
                return Some(state.a);
            }
            if let Some(out) = get_best_input(state.clone(), i - 1, state.a) {
                return Some(out);
            }
        }
    }
    None
}
