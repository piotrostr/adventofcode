use std::{collections::HashSet, fs::read_to_string};

#[derive(Debug)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug)]
pub struct Instruction {
    direction: Direction,
    n_steps: usize,
}

impl Instruction {
    pub fn parse(line: &str) -> Self {
        let (direction, n_steps) = line.split_once(' ').unwrap();
        Self {
            direction: parse_direction(direction),
            n_steps: n_steps.parse::<usize>().unwrap(),
        }
    }
}

pub fn parse_direction(direction: &str) -> Direction {
    match direction {
        "R" => Direction::Right,
        "L" => Direction::Left,
        "U" => Direction::Up,
        "D" => Direction::Down,
        _ => unreachable!(),
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Bridge {
    last_head_position: Point,
    head_position: Point,
    tail_position: Point,
    tail_positions_set: HashSet<Point>,
}

impl Bridge {
    pub fn new() -> Self {
        Self {
            last_head_position: Point { x: 0, y: 0 },
            head_position: Point { x: 0, y: 0 },
            tail_position: Point { x: 0, y: 0 },
            tail_positions_set: HashSet::<Point>::new(),
        }
    }

    pub fn handle_instruction(&mut self, instruction: Instruction) {
        let mut steps_remaining = instruction.n_steps;
        while steps_remaining > 0 {
            self.last_head_position = self.head_position;

            match instruction.direction {
                Direction::Right => {
                    self.head_position = Point {
                        x: self.head_position.x + 1,
                        ..self.head_position
                    };
                }
                Direction::Left => {
                    self.head_position = Point {
                        x: self.head_position.x - 1,
                        ..self.head_position
                    };
                }
                Direction::Up => {
                    self.head_position = Point {
                        y: self.head_position.y + 1,
                        ..self.head_position
                    };
                }
                Direction::Down => {
                    self.head_position = Point {
                        y: self.head_position.y - 1,
                        ..self.head_position
                    };
                }
            }

            if !self.tail_is_adjacent_to_head() {
                self.tail_position = self.last_head_position;
            }

            self.tail_positions_set.insert(self.tail_position);

            steps_remaining -= 1;
        }
    }

    pub fn tail_is_adjacent_to_head(&self) -> bool {
        // overlap
        if self.tail_position.x == self.head_position.x
            && self.tail_position.y == self.head_position.y
        {
            return true;
        }

        // top/bottom
        if self.tail_position.x == self.head_position.x
            && (self.tail_position.y == self.head_position.y + 1
                || self.tail_position.y == self.head_position.y - 1)
        {
            return true;
        }

        // sides
        if self.tail_position.y == self.head_position.y
            && (self.tail_position.x == self.head_position.x + 1
                || self.tail_position.x == self.head_position.x - 1)
        {
            return true;
        }

        // corners
        if (self.tail_position.x == self.head_position.x + 1
            || self.tail_position.x == self.head_position.x - 1)
            && (self.tail_position.y == self.head_position.y + 1
                || self.tail_position.y == self.head_position.y - 1)
        {
            return true;
        }

        false
    }
}

fn main() {
    let mut bridge = Bridge::new();
    read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(Instruction::parse)
        .for_each(|instruction| {
            bridge.handle_instruction(instruction);
        });

    println!("{}", bridge.tail_positions_set.len());
}
