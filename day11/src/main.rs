use num_bigint::BigUint;
use std::fs::read_to_string;

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
    num: BigUint,
}

impl Operation {
    pub fn apply() {}
}

#[derive(Debug)]
pub struct Test {
    divisible_by: BigUint,
    monkey_target_index_when_true: i32,
    monkey_target_index_when_false: i32,
}

#[derive(Debug)]
pub struct Monkey {
    _index: i32,
    items_inspected: BigUint,
    items: Vec<BigUint>,
    operation: Operation,
    test: Test,
}

impl Monkey {
    pub fn parse(monkey_entry: String) -> Self {
        let mut lines = monkey_entry.lines();
        let (_, index_str_raw) = lines.next().unwrap().split_once(' ').unwrap();
        let mut index_str = index_str_raw.to_string();
        index_str.pop(); // pop ":" at the end
        let starting_items_str = lines.next().unwrap().split(": ").last().unwrap();
        let starting_items = starting_items_str
            .split(',')
            .map(|x| x.trim().parse::<u32>().unwrap())
            .map(BigUint::from)
            .collect();
        let (_, op_str) = lines.next().unwrap().split_once('=').unwrap();
        let operation: Operation;
        if op_str.contains('*') {
            let op_value = op_str.split('*').last().unwrap().trim();
            if op_value == "old" {
                operation = Operation {
                    op: OperationType::MultiplySelf,
                    num: BigUint::from(0u32),
                };
            } else {
                operation = Operation {
                    op: OperationType::Multiply,
                    num: BigUint::from(op_value.parse::<u32>().unwrap()),
                };
            }
        } else {
            let op_value = op_str.split('+').last().unwrap().trim();
            if op_value == "old" {
                operation = Operation {
                    op: OperationType::AddSelf,
                    num: BigUint::from(0u32),
                };
            } else {
                operation = Operation {
                    op: OperationType::Add,
                    num: BigUint::from(op_value.parse::<u32>().unwrap()),
                };
            }
        }
        let test = Test {
            divisible_by: BigUint::from(
                lines
                    .next()
                    .unwrap()
                    .split("by")
                    .last()
                    .unwrap()
                    .trim()
                    .parse::<u32>()
                    .unwrap(),
            ),
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
            _index: index_str.parse().unwrap(),
            items: starting_items,
            items_inspected: BigUint::from(0u32),
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
        .collect::<Vec<Monkey>>();

    // key is to find a different way to keep the worry_level from going exponential,
    // it used to be dividing by three
    //
    // I got stuck here and had to look it up, but the property of modulo operation is that
    // a product of all of the divisors can be used to factor the number, resulting in something in
    // a sense of modular arithmetic in cryptography
    //
    // the number modulo divisor product is congruent to the number modulo for each divisor, making
    // the number retain its properties while not exploding in size
    let divisor_product: BigUint = monkeys
        .iter()
        .map(|monkey| &monkey.test.divisible_by)
        .product();

    for _ in 0..10_000 {
        for i in 0..monkeys.len() {
            while !monkeys[i].items.is_empty() {
                let worry_level = monkeys[i].items.remove(0);
                monkeys[i].items_inspected += BigUint::from(1u32);
                let mut item = match monkeys[i].operation.op {
                    OperationType::Add => worry_level + &monkeys[i].operation.num,
                    OperationType::AddSelf => &worry_level + &worry_level,
                    OperationType::Multiply => &worry_level * &monkeys[i].operation.num,
                    OperationType::MultiplySelf => &worry_level * &worry_level,
                };

                // line below is all it takes to prevent the numbers from exploding
                item %= &divisor_product;

                if &item % &monkeys[i].test.divisible_by == BigUint::from(0u32) {
                    let index = monkeys[i].test.monkey_target_index_when_true as usize;
                    monkeys[index].items.push(item);
                } else {
                    let index = monkeys[i].test.monkey_target_index_when_false as usize;
                    monkeys[index].items.push(item);
                }
            }
        }
    }

    // after all of the rounds (debug)
    for monkey in monkeys.iter() {
        println!(
            "monkey {} inspected items {} times",
            monkey._index, monkey.items_inspected
        );
    }

    // sort by items inspected
    monkeys.sort_by(|a, b| b.items_inspected.cmp(&a.items_inspected));

    println!(
        "after 10_000 rounds: {:?}",
        monkeys
            .iter()
            .take(2)
            .map(|monkey| monkey.items_inspected.clone())
            .reduce(|acc, curr| acc * curr)
            .unwrap()
    );
}
