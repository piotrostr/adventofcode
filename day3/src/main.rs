use std::collections::HashMap;

/// parse_input_string parses copy-pasted input from stdin and returns it as a string
pub fn parse_input_string() -> String {
    let mut input = String::new();
    loop {
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                let as_split = input.split("\n").collect::<Vec<&str>>();
                if as_split[as_split.len() - 2] == "" && as_split[as_split.len() - 1] == "" {
                    break;
                }
            }
            Err(e) => {
                panic!("Error {}", e.to_string());
            }
        }
    }
    return input;
}

pub fn get_priority(item: char) -> usize {
    for (index, ch) in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .enumerate()
    {
        if ch == item {
            return index + 1;
        }
    }
    return 0;
}

fn main() {
    let input = parse_input_string();

    let rucksacks = input.split("\n").collect::<Vec<&str>>();

    let mut total_priorioty = 0;
    for rucksack in rucksacks {
        let mut items = HashMap::new();
        let (first_compartment, second_compartment) = rucksack.split_at(rucksack.len() / 2);
        for item in first_compartment.chars() {
            items.insert(item, 1);
        }
        for item in second_compartment.chars() {
            if items.contains_key(&item) {
                let priority = get_priority(item);
                total_priorioty += priority;
                // break here in order not to add the item twice,
                // even if it occurs twice it is only shared once
                //
                // if the case of multiple shared elements, there will have to be another map to
                // prevent double-adding the items instead of break here
                break;
            }
        }
    }
    println!("{}", total_priorioty);
}
