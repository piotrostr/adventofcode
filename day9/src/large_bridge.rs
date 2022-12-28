use crate::common::check_if_are_adjacent;

use super::common::{Direction, Instruction, Point};
use std::{
    collections::{HashSet, VecDeque},
    mem::swap,
};

/// there is no head position and no tail, as the head is just going to be the `self.knots.back()`
/// and tail `self.knots.front()`
pub struct LargeBridge {
    knots: VecDeque<Point>,
    tail_positions_set: HashSet<Point>,
}

impl LargeBridge {
    pub fn new() -> Self {
        Self {
            knots: VecDeque::<Point>::from(
                (0..=9)
                    .map(|_| Point { x: 0, y: 0 })
                    .collect::<Vec<Point>>(),
            ),
            tail_positions_set: HashSet::<Point>::new(),
        }
    }

    pub fn unique_positions(&self) -> usize {
        self.tail_positions_set.len()
    }

    /// This is going to be slightly different from the previous implementation, every knot is
    /// going to behave as the tail, meaning it will be following the next node (used to be head)
    ///
    /// The check_if_adjacent method is be refactored to take two points as parameters, keeping the
    /// same logic, yet now it can be applied iteratively to all of the points
    pub fn handle_instruction(&mut self, instruction: &Instruction) {
        let mut steps_remaining = instruction.n_steps;
        while steps_remaining > 0 {
            let head_position = *self.knots.back().unwrap();
            match instruction.direction {
                Direction::Right => {
                    *self.knots.back_mut().unwrap() = Point {
                        x: head_position.x + 1,
                        y: head_position.y,
                    };
                }
                Direction::Left => {
                    *self.knots.back_mut().unwrap() = Point {
                        x: head_position.x - 1,
                        y: head_position.y,
                    };
                }
                Direction::Up => {
                    *self.knots.back_mut().unwrap() = Point {
                        x: head_position.x,
                        y: head_position.y + 1,
                    };
                }
                Direction::Down => {
                    *self.knots.back_mut().unwrap() = Point {
                        x: head_position.x,
                        y: head_position.y - 1,
                    };
                }
            }

            // the error is here, I implemented the logic here as in the snake game
            //
            // for this application, the knots are stretching in a similar direction as the head,
            // but not following it directly, more-so trying to hold the line
            let mut iterator = self.knots.iter_mut().rev();
            let mut knot_one = iterator.next().unwrap();
            let mut new_position = head_position;
            while iterator.len() > 0 {
                let knot_two = iterator.next().unwrap();
                // the method below might be redundant altogether for the sake of incrementing it
                // as in the example
                //
                // only the last knot before the head follows directly, similar as tail in the
                // previous part of the question
                //
                // the rest of the knots are just trying to hold the line
                if !check_if_are_adjacent(knot_one, knot_two) {
                    swap(&mut (*knot_two), &mut new_position);
                }
                knot_one = knot_two;
            }

            steps_remaining -= 1;

            self.tail_positions_set.insert(*self.knots.front().unwrap());

            self.visualize();
        }
    }

    pub fn visualize(&self) {
        let mut min_x = 0;
        let mut max_x = 0;
        let mut min_y = 0;
        let mut max_y = 0;

        for knot in self.knots.iter() {
            if knot.x < min_x {
                min_x = knot.x;
            }
            if knot.x > max_x {
                max_x = knot.x;
            }
            if knot.y < min_y {
                min_y = knot.y;
            }
            if knot.y > max_y {
                max_y = knot.y;
            }
        }

        for y in (min_y..=max_y).rev() {
            for x in min_x..=max_x {
                let point = Point { x, y };
                if self.knots.contains(&point) {
                    if point == *self.knots.front().unwrap() {
                        print!("T");
                    } else if point == *self.knots.back().unwrap() {
                        print!("H");
                    } else {
                        print!("X");
                    }
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }
}
