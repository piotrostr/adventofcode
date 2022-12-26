#[derive(Debug)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug)]
pub struct Instruction {
    pub direction: Direction,
    pub n_steps: usize,
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
pub struct Point {
    pub x: i32,
    pub y: i32,
}
