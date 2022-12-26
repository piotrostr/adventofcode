mod bridge;
mod common;
mod large_bridge;

use bridge::Bridge;
use common::Instruction;
use large_bridge::LargeBridge;
use std::fs::read_to_string;

fn main() {
    let mut bridge = Bridge::new();
    read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(Instruction::parse)
        .for_each(|instruction| {
            bridge.handle_instruction(instruction);
        });

    println!("{}", bridge.unique_positions());

    let mut _large_bridge = LargeBridge::new();
}
