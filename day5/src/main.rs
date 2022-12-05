use std::{
    collections::{HashMap, VecDeque},
    fs::read_to_string,
};

fn main() {
    let input = read_to_string("input.txt").unwrap();

    // the most difficult part is to create the stacks from the input

    // first split the crates and instrutions as those will be handled differently
    let (crates_section, instructions_section) = input.split_once("\n\n").unwrap();
    let mut crates = crates_section.split("\n").collect::<Vec<&str>>();
    let instructions = instructions_section.split("\n").collect::<Vec<&str>>();

    // move the crates into stacks
    //
    // first, get the number of stacks and initialize
    // that is from the last line of the input
    let mut num_stacks: usize = 0;

    // get the largest stack index
    for el in crates.last().unwrap().chars().rev() {
        if el.is_numeric() {
            num_stacks = el.to_digit(10).unwrap() as usize;
            break;
        }
    }

    // after getting the index the last line with numbers is no longer needed
    crates.pop();

    // initialize stacks, indexed starting from 1 as in the instructions
    let mut stacks = HashMap::<usize, VecDeque<&str>>::new();
    for i in 1..=num_stacks {
        stacks.insert(i, VecDeque::new());
    }

    for row in crates
        .iter()
        .map(|x| x.split(" ").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>()
        .iter()
    {
        // keep track of what is the destined stack based on the number of spaces
        // four spaces or another number means the index is to be incremented
        let mut crate_index = 1;
        let mut iterator = row.iter().peekable();
        // have a peek of next element
        while iterator.len() != 0 {
            // if it is "" then drop the spaces and increment the col
            if iterator.peek() == Some(&&"") {
                for _ in 0..4 {
                    iterator.next();
                }
            } else {
                // in case of actual crate, push it in!
                let actual_crate = iterator.next().unwrap();
                stacks
                    .entry(crate_index)
                    .and_modify(|stack| stack.push_front(*actual_crate));
            }
            crate_index += 1;
        }
    }

    // now the instructions, parse just the numbers
    let mut instruction_sets = instructions
        .iter()
        .map(|x| x.split(" ").collect::<Vec<&str>>())
        .map(|x| {
            x.iter()
                .filter(|&e| match e.parse::<usize>() {
                    Ok(_) => true,
                    Err(_) => false,
                })
                .map(|&e| e.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    instruction_sets.pop(); // the last element is an empty array

    for mut set in instruction_sets.clone() {
        // every instruction set has exactly three nums - n, from, to
        let to = set.pop().unwrap();
        let from = set.pop().unwrap();
        let n = set.pop().unwrap();
        for _ in 0..n {
            let element = stacks.get_mut(&from).unwrap().pop_back().unwrap();
            stacks.get_mut(&to).unwrap().push_back(element);
        }
    }
    let mut message = "".to_string();
    for i in 1..=num_stacks {
        let the_crate = stacks.get(&i).unwrap().back().unwrap();
        let letter = the_crate.chars().nth(1).unwrap();
        message = format!("{}{}", message, letter);
    }

    println!("first part message: {}", message);
}
