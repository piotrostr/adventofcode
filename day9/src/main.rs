mod bridge;
mod common;
mod large_bridge;

use bridge::Bridge;
use common::Instruction;
use large_bridge::LargeBridge;
use std::fs::read_to_string;

fn main() {
    let mut bridge = Bridge::new();
    let mut large_bridge = LargeBridge::new();
    read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(Instruction::parse)
        .for_each(|instruction| {
            bridge.handle_instruction(&instruction);
            large_bridge.handle_instruction(&instruction);
        });

    println!("unique positions:");
    println!("small bridge {}", bridge.unique_positions());
    println!("large bridge {}", large_bridge.unique_positions());
}
