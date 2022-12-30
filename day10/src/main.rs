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
    cycles: HashMap<usize, i16>, // { index -> x }
    crt: [[char; 40]; 6],
    // `x` take values of the `x` register in range of [0: 40]
}

impl Screen {
    pub fn new() -> Self {
        Self {
            x: 1,
            cycle_awaiting: None,
            cycle_index: 1,
            cycles: HashMap::new(),
            crt: [['.'; 40]; 6],
        }
    }

    pub fn handle_cycle(&mut self, cycle: Cycle) {
        // whenever insert the cycle, also write to cathode
        self.cycles.insert(self.cycle_index, self.x);
        self.write_px_to_crt();

        // always increment cycle index once
        self.cycle_index += 1;

        if let Some(cycle_awaiting) = &self.cycle_awaiting {
            match cycle_awaiting.instruction {
                Instruction::Addx => {
                    self.x += cycle_awaiting.value.unwrap();

                    self.cycles.insert(self.cycle_index, self.x);
                    self.write_px_to_crt();

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

    pub fn print_sprite(&self) {
        for i in 0..=40 {
            if i == self.x || i == self.x + 1 || i == self.x - 1 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }

    pub fn write_px_to_crt(&mut self) {
        let cycle_index = self.cycle_index - 1;
        let x = cycle_index % 40;
        let y = cycle_index / 40;
        if x as i16 == self.x || x as i16 == self.x + 1 || x as i16 == self.x - 1 {
            self.crt[y][x] = '#';
        }
    }

    /// the method prints out the CRTs (cathode-ray tubes)
    ///
    /// this should only be called when all of the cycles are processed
    pub fn produce_image(&self) {
        println!();
        for row in self.crt {
            for el in row {
                print!("{}", el)
            }
            println!()
        }
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
            let signal = screen.cycles.get(&strength).unwrap().to_owned();
            println!("signal at {}: {}", strength, signal);
            signal * i16::try_from(strength).ok().unwrap()
        })
        .iter()
        .copied()
        .reduce(|acc, curr| acc + curr)
        .unwrap();

    println!("sum of six strenghts: {}", sum_of_six_strengths);

    // second part is about visualizing the screen
    screen.produce_image();
}
