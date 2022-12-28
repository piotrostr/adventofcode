use super::common::{Direction, Instruction, Point};
use std::collections::{HashSet, VecDeque};

/// there is no head position and no tail, as the head is just going to be the `self.knots.back()`
/// and tail `self.knots.front()`
pub struct LargeBridge {
    knots: VecDeque<Point>,
    tail_positions_set: HashSet<Point>,
}

impl LargeBridge {
    pub fn new() -> Self {
        Self {
            knots: VecDeque::<Point>::from(vec![Point { x: 0, y: 0 }]),
            tail_positions_set: HashSet::<Point>::new(),
        }
    }

    pub fn unique_positions(&self) -> usize {
        0
    }

    /// This is going to be slightly different from the previous implementation, every knot is
    /// going to behave as the tail, meaning it will be following the next node (used to be head)
    ///
    /// The check_if_adjacent method is be refactored to take two points as parameters, keeping the
    /// same logic, yet now it can be applied iteratively to all of the points
    pub fn handle_instruction(&mut self, instruction: &Instruction) {
        let mut steps_remaining = instruction.n_steps;
        while steps_remaining > 0 {
            match instruction.direction {
                Direction::Up => {}
                Direction::Down => {}
                Direction::Left => {}
                Direction::Right => {}
            }

            steps_remaining -= 1;

            self.tail_positions_set.insert(*self.knots.front().unwrap());
        }
    }
}
