use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug)]
enum Instruction {
    Addx,
    Noop,
}

#[derive(Debug)]
pub struct Screen {
    x: i16,
    cycle_awaiting: Option<Cycle>,
    cycle_index: usize,
    history: HashMap<usize, i16>, // { index -> x }
}

impl Screen {
    pub fn new() -> Self {
        Self {
            x: 1,
            cycle_awaiting: None,
            cycle_index: 1,
            history: HashMap::new(),
        }
    }

    pub fn handle_cycle(&mut self, cycle: Cycle) {
        self.history.insert(self.cycle_index, self.x);

        // always increment cycle index once
        self.cycle_index += 1;

        if let Some(cycle_awaiting) = &self.cycle_awaiting {
            match cycle_awaiting.instruction {
                Instruction::Addx => {
                    self.x += cycle_awaiting.value.unwrap();
                    self.history.insert(self.cycle_index, self.x);
                    // increment cycle index again if there is addx instruction
                    self.cycle_index += 1;
                }
                Instruction::Noop => {
                    // do nothing
                }
            }
        }

        self.cycle_awaiting = Some(cycle);
    }
}

impl Default for Screen {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct Cycle {
    instruction: Instruction,
    value: Option<i16>, // None if instruction is not `Instruction::Addx`
}

impl Cycle {
    pub fn parse(line: &str) -> Self {
        if let Some((_, value)) = line.split_once(' ') {
            Self {
                instruction: Instruction::Addx,
                value: Some(value.parse::<i16>().unwrap()),
            }
        } else {
            Self {
                instruction: Instruction::Noop,
                value: None,
            }
        }
    }
}

fn main() {
    let mut screen = Screen::default();

    read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(Cycle::parse)
        .for_each(|cycle| screen.handle_cycle(cycle));

    let six_strengths: [usize; 6] = [20, 60, 100, 140, 180, 220];

    let sum_of_six_strengths = six_strengths
        .map(|strength| {
            let signal = screen.history.get(&strength).unwrap().to_owned();
            println!("signal at {}: {}", strength, signal);
            signal * i16::try_from(strength).ok().unwrap()
        })
        .iter()
        .copied()
        .reduce(|acc, curr| acc + curr)
        .unwrap();

    println!("sum of six strenghts: {}", sum_of_six_strengths);
}
