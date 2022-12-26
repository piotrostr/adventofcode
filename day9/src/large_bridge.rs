use super::common::{Instruction, Point};
use std::collections::VecDeque;

/// there is no head position and no tail, as the head is just going to be the `self.knots.back()`
/// and tail `self.knots.front()`
pub struct LargeBridge {
    _knots: VecDeque<Point>,
}

impl LargeBridge {
    pub fn new() -> Self {
        Self {
            _knots: VecDeque::<Point>::new(),
        }
    }

    pub fn _handle_instruction(&mut self, _instruction: Instruction) {
        // TODO
    }
}
