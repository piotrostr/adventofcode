use crate::common::check_if_are_adjacent;

use super::common::{Direction, Instruction, Point};
use std::collections::HashSet;

#[derive(Debug)]
pub struct Bridge {
    last_head_position: Point,
    head_position: Point,
    tail_position: Point,
    tail_positions_set: HashSet<Point>,
}

impl Bridge {
    pub fn unique_positions(&self) -> usize {
        self.tail_positions_set.len()
    }

    pub fn new() -> Self {
        Self {
            last_head_position: Point { x: 0, y: 0 },
            head_position: Point { x: 0, y: 0 },
            tail_position: Point { x: 0, y: 0 },
            tail_positions_set: HashSet::<Point>::new(),
        }
    }

    pub fn handle_instruction(&mut self, instruction: &Instruction) {
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

            if !check_if_are_adjacent(&self.tail_position, &self.head_position) {
                self.tail_position = self.last_head_position;
            }

            self.tail_positions_set.insert(self.tail_position);

            steps_remaining -= 1;
        }
    }
}
