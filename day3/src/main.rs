use std::{collections::HashMap, io::Read};

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

pub fn read_input_string_from_file() -> String {
    let mut input = String::new();
    let mut file = std::fs::File::open("input.txt").unwrap();
    file.read_to_string(&mut input).unwrap();
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

    // first part of the question
    let mut total_priorioty = 0;
    for rucksack in rucksacks.clone() {
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
                // in case of multiple shared elements, there will have to be another map to
                // prevent double-adding the items instead of break here
                break;
            }
        }
    }
    println!("first part: {}", total_priorioty);

    // second part of the question
    let mut total_priority = 0;
    let groups = rucksacks.chunks(3).collect::<Vec<&[&str]>>();
    for group in groups {
        let mut all_items = HashMap::<char, usize>::new();
        for elf in group {
            let mut items = HashMap::new();
            // get the single occurences to later transfer
            // the maximum occurrences could be one
            //
            // would be okay to use an array here as well
            for item in elf.chars() {
                items.entry(item).or_insert(1);
            }

            // transfer into the final map
            for (&k, &v) in items.iter() {
                all_items
                    .entry(k)
                    .and_modify(|count| *count += v)
                    .or_insert(v);
            }
        }

        let mut values = all_items.iter().collect::<Vec<(&char, &usize)>>();

        // could also iterate through the values here and check instead of sorting and getting the max

        // might look tricky, is quite simple though
        // a.1 means a[1], the cmp is to return `Ordering` in the `sort_by` operation
        // `a.cmp(b)` means the elements will be sorted in ascending manner
        values.sort_by(|a: &(&char, &usize), b: &(&char, &usize)| a.1.cmp(b.1));

        // check for emptiness, sometimes the input might be with ["", ""] arrays after parsing
        // no big deal, ideally the input can be pre-processed beforehand to avoid re-checking for
        // this condition
        if values.is_empty() {
            continue;
        }

        // get the only value that occured three times
        let common_badge = values.pop().unwrap().0;

        let priority = get_priority(*common_badge);
        total_priority += priority;
    }
    println!("second part: {}", total_priority);
}
