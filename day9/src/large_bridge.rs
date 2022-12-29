use super::common::{Direction, Instruction, Point};
use std::{collections::HashSet, thread::sleep, time::Duration};

/// there is no head position and no tail, as the head is just going to be the `self.knots.back()`
/// and tail `self.knots.front()`
pub struct LargeBridge {
    knots: [Point; 10],
    tail_positions_set: HashSet<Point>,
}

impl LargeBridge {
    pub fn new() -> Self {
        Self {
            knots: [Point { x: 0, y: 0 }; 10],
            tail_positions_set: HashSet::<Point>::new(),
        }
    }

    pub fn unique_positions(&self) -> usize {
        self.tail_positions_set.len()
    }

    pub fn handle_instruction(&mut self, instruction: &Instruction) {
        let mut steps_remaining = instruction.n_steps;
        while steps_remaining > 0 {
            let head = self.knots.first_mut().unwrap();
            match instruction.direction {
                Direction::Right => {
                    *head = Point {
                        x: head.x + 1,
                        y: head.y,
                    };
                }
                Direction::Left => {
                    *head = Point {
                        x: head.x - 1,
                        y: head.y,
                    };
                }
                Direction::Up => {
                    *head = Point {
                        x: head.x,
                        y: head.y + 1,
                    };
                }
                Direction::Down => {
                    *head = Point {
                        x: head.x,
                        y: head.y - 1,
                    };
                }
            }

            // the transformation has to be made based on the change of position of the previous
            // knot
            for i in 1..self.knots.len() {
                let diff = (
                    self.knots[i - 1].x - self.knots[i].x,
                    self.knots[i - 1].y - self.knots[i].y,
                );
                // credit for the snippet below is https://fasterthanli.me/series/advent-of-code-2022/part-9
                // was stuck for too long on this one
                let adjustment = match diff {
                    // overlapping
                    (0, 0) => (0, 0),
                    // touching up/left/down/right
                    (0, 1) | (1, 0) | (0, -1) | (-1, 0) => (0, 0),
                    // touching diagonally
                    (1, 1) | (1, -1) | (-1, 1) | (-1, -1) => (0, 0),
                    // need to move up/left/down/right
                    (0, 2) => (0, 1),
                    (0, -2) => (0, -1),
                    (2, 0) => (1, 0),
                    (-2, 0) => (-1, 0),
                    // need to move to the right diagonally
                    (2, 1) => (1, 1),
                    (2, -1) => (1, -1),
                    // need to move to the left diagonally
                    (-2, 1) => (-1, 1),
                    (-2, -1) => (-1, -1),
                    // need to move up/down diagonally
                    (1, 2) => (1, 1),
                    (-1, 2) => (-1, 1),
                    (1, -2) => (1, -1),
                    (-1, -2) => (-1, -1),
                    // need to move diagonally
                    (-2, -2) => (-1, -1),
                    (-2, 2) => (-1, 1),
                    (2, -2) => (1, -1),
                    (2, 2) => (1, 1),
                    _ => panic!("unexpected diff {:?}", diff),
                };
                self.knots[i] = Point {
                    x: self.knots[i].x + adjustment.0,
                    y: self.knots[i].y + adjustment.1,
                };
            }

            steps_remaining -= 1;

            self.tail_positions_set.insert(*self.knots.last().unwrap());

            // self.visualize();
        }
    }

    // this might be too small depending on input and lead to errors,
    // change the min/max coords when using with the main submission input
    pub fn _visualize(&self) {
        let min_x = -40;
        let max_x = 40;
        let min_y = -40;
        let max_y = 20;

        for y in (min_y..=max_y).rev() {
            for x in min_x..=max_x {
                let point = Point { x, y };
                if self.knots.contains(&point) {
                    if point == *self.knots.last().unwrap() {
                        print!("T");
                    } else if point == *self.knots.first().unwrap() {
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

        // wait a tiny bit and flush buffer to create an animation feel
        sleep(Duration::from_millis(150));
        print!("\x1B[2J\x1B[1;1H");
    }
}
