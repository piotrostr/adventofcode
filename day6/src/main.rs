use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let mut index: usize = 4;
    let chars = input.chars().collect::<Vec<char>>();
    while index != input.len() {
        let window = chars[(index - 4)..index].to_vec();
        // use a HashSet to find if all of the numbers are unique
        let set = HashSet::<&char>::from_iter(window.iter());
        if set.len() == 4 {
            println!("first four unique at: {}", index);
            break;
        }
        index += 1;
    }
}
