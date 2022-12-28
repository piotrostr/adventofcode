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

pub fn check_if_are_adjacent(knot_one: &Point, knot_two: &Point) -> bool {
    // overlap
    if knot_one.x == knot_two.x && knot_one.y == knot_two.y {
        return true;
    }

    // top/bottom
    if knot_one.x == knot_two.x && (knot_one.y == knot_two.y + 1 || knot_one.y == knot_two.y - 1) {
        return true;
    }

    // sides
    if knot_one.y == knot_two.y && (knot_one.x == knot_two.x + 1 || knot_one.x == knot_two.x - 1) {
        return true;
    }

    // corners
    if (knot_one.x == knot_two.x + 1 || knot_one.x == knot_two.x - 1)
        && (knot_one.y == knot_two.y + 1 || knot_one.y == knot_two.y - 1)
    {
        return true;
    }

    false
}
