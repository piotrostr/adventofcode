use std::{env::args, io};

/// parse_input_string parses copy-pasted input from stdin and returns it as a string
fn parse_input_string() -> String {
    let mut input = String::new();
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let as_split = input.split("\n").collect::<Vec<&str>>();
                if as_split[as_split.len() - 2] == "" && as_split[as_split.len() - 1] == "" {
                    break;
                }
            }
            Err(e) => {
                println!("Error {}", e.to_string());
                break;
            }
        }
    }
    return input;
}

fn main() {
    let input = parse_input_string();
    let mut guide = input.split("\n").collect::<Vec<&str>>();

    // pop the two returns at the end
    guide.pop();
    guide.pop();

    match args().nth(1) {
        Some(pattern) => {
            if pattern == "--first" {
                let mut total_score: i32 = 0;
                for strategy in guide {
                    let (opponent_move, player_move) = strategy.split_once(" ").unwrap();
                    let score = get_score(player_move, &opponent_move[1..opponent_move.len()]);
                    total_score += score;
                }
                println!("{}", total_score);
            }
        }
        // second
        None => {
            let mut total_score: i32 = 0;
            for strategy in guide {
                let (opponent_move, player_move) = strategy.split_once(" ").unwrap();
                let score = get_score_second_round(opponent_move, player_move);
                total_score += score;
            }
            println!("{}", total_score);
        }
    }
}

/// Rules
///
/// opponent:
/// rock = A
/// paper = B
/// scissors = C
///
/// player
/// rock = X
/// paper = Y
/// scissors = Z
///
/// points
/// rock = 1
/// paper = 2
/// scissors = 3
///
/// win = 6
/// draw = 3
/// lose = 0
///
/// opponent rock
/// A : X => draw + rock = 3 + 1 = 4
/// A : Y => win + paper = 6 + 2 = 8
/// A : Z => lose + scissors = 0 + 3 = 3
///
/// opponent paper
/// B : X => lose + rock =  0 + 1 = 1
/// B : Y => draw + paper = 3 + 2 = 5
/// B : Z => win + scissors = 6 + 3 = 9
///
/// opponent scissors
/// C : X => win + rock = 6 + 1 = 7
/// C : Y => lose + paper = 0 + 2 = 2
/// C : Z => draw + scissors = 3 + 3 = 6
fn get_score(opponent_move: &str, player_move: &str) -> i32 {
    if opponent_move == "A" && player_move == "X" {
        return 4;
    } else if opponent_move == "A" && player_move == "Y" {
        return 8;
    } else if opponent_move == "A" && player_move == "Z" {
        return 3;
    } else if opponent_move == "B" && player_move == "X" {
        return 1;
    } else if opponent_move == "B" && player_move == "Y" {
        return 5;
    } else if opponent_move == "B" && player_move == "Z" {
        return 9;
    } else if opponent_move == "C" && player_move == "X" {
        return 7;
    } else if opponent_move == "C" && player_move == "Y" {
        return 2;
    } else if opponent_move == "C" && player_move == "Z" {
        return 6;
    }
    return 0;
}

/// Rules changed:
/// rock = A
/// paper = B
/// scissors = C
///
/// second col:
/// lose = X
/// draw = Y
/// win = Z
///
/// points still the same:
/// rock = 1
/// paper = 2
/// scissors = 3
///
/// win = 6
/// draw = 3
/// lose = 0
fn get_score_second_round(opponent_move: &str, player_move: &str) -> i32 {
    match player_move {
        // lose
        "X" => match opponent_move {
            "A" => {
                // scissors
                return 0 + 3;
            }
            "B" => {
                // rock
                return 0 + 1;
            }
            "C" => {
                // paper
                return 0 + 2;
            }
            _ => {}
        },
        // draw
        "Y" => match opponent_move {
            "A" => {
                // rock
                return 3 + 1;
            }
            "B" => {
                // paper
                return 3 + 2;
            }
            "C" => {
                // scissors
                return 3 + 3;
            }
            _ => {}
        },
        // win
        "Z" => match opponent_move {
            "A" => {
                // paper
                return 6 + 2;
            }
            "B" => {
                // scissors
                return 6 + 3;
            }
            "C" => {
                // rock
                return 6 + 1;
            }
            _ => {}
        },
        _ => {}
    }
    return 0;
}
