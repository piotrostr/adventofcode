use std::{collections::VecDeque, fs::read_to_string};

#[derive(Debug)]
enum OperationType {
    Add,
    Multiply,
    AddSelf,
    MultiplySelf,
}

#[derive(Debug)]
pub struct Operation {
    op: OperationType,
    num: i32,
}

impl Operation {
    pub fn apply() {}
}

#[derive(Debug)]
pub struct Test {
    divisible_by: i32,
    monkey_target_index_when_true: i32,
    monkey_target_index_when_false: i32,
}

#[derive(Debug)]
pub struct Monkey {
    index: i32,
    starting_items: Vec<i32>,
    operation: Operation,
    test: Test,
}

impl Monkey {
    pub fn parse(monkey_entry: String) -> Self {
        let mut lines = monkey_entry.lines();
        let (_, index_str_raw) = lines.next().unwrap().split_once(' ').unwrap();
        let mut index_str = index_str_raw.to_string();
        index_str.pop(); // pop ":" at the end
        let starting_items_str = lines.next().unwrap().split(' ').last().unwrap();
        let starting_items = starting_items_str
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let (_, op_str) = lines.next().unwrap().split_once('=').unwrap();
        let operation: Operation;
        if op_str.contains('*') {
            let op_value = op_str.split('*').last().unwrap().trim();
            if op_value == "old" {
                operation = Operation {
                    op: OperationType::MultiplySelf,
                    num: 0,
                };
            } else {
                operation = Operation {
                    op: OperationType::Multiply,
                    num: op_value.parse::<i32>().unwrap(),
                };
            }
        } else {
            let op_value = op_str.split('+').last().unwrap().trim();
            if op_value == "old" {
                operation = Operation {
                    op: OperationType::AddSelf,
                    num: 0,
                };
            } else {
                operation = Operation {
                    op: OperationType::Add,
                    num: op_value.parse::<i32>().unwrap(),
                };
            }
        }
        let test = Test {
            divisible_by: lines
                .next()
                .unwrap()
                .split("by")
                .last()
                .unwrap()
                .trim()
                .parse::<i32>()
                .unwrap(),
            monkey_target_index_when_true: lines
                .next()
                .unwrap()
                .split("monkey")
                .last()
                .unwrap()
                .trim()
                .parse::<i32>()
                .unwrap(),
            monkey_target_index_when_false: lines
                .next()
                .unwrap()
                .split("monkey")
                .last()
                .unwrap()
                .trim()
                .parse::<i32>()
                .unwrap(),
        };
        Monkey {
            index: index_str.parse().unwrap(),
            starting_items,
            operation,
            test,
        }
    }
}

fn main() {
    let mut monkeys = read_to_string("input.txt")
        .unwrap()
        .split("\n\n")
        .map(|monkey_entry| Monkey::parse(monkey_entry.to_string()))
        .collect::<VecDeque<Monkey>>();

    while !monkeys.is_empty() {
        let monkey = monkeys.pop_front().unwrap();
        println!("{:?}", monkey.index);
    }
}
