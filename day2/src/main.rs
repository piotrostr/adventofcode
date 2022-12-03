use std::{env::args, io};

fn main() {
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

    let mut guide = input.split("\n").collect::<Vec<&str>>();

    // pop the two returns at the end
    guide.pop();
    guide.pop();

    match args().nth(1) {
        Some(pattern) => {
            if pattern == "--first" {
                let mut total_score: i32 = 0;
                for strategy in guide {
                    // this split is not optimal, would prefer to do the
                    // pythonic a, b = split(" ")
                    let (player_move, opponent_move) = strategy.split_at(1);
                    let score = get_score(player_move, &opponent_move[1..opponent_move.len()]);
                    total_score += score;
                }
                println!("{}", total_score);
            }
        }
        // second
        None => {
            println!("second");
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
fn get_score(player_move: &str, opponent_move: &str) -> i32 {
    if player_move == "A" && opponent_move == "X" {
        return 4;
    } else if player_move == "A" && opponent_move == "Y" {
        return 8;
    } else if player_move == "A" && opponent_move == "Z" {
        return 3;
    } else if player_move == "B" && opponent_move == "X" {
        return 1;
    } else if player_move == "B" && opponent_move == "Y" {
        return 5;
    } else if player_move == "B" && opponent_move == "Z" {
        return 9;
    } else if player_move == "C" && opponent_move == "X" {
        return 7;
    } else if player_move == "C" && opponent_move == "Y" {
        return 2;
    } else if player_move == "C" && opponent_move == "Z" {
        return 6;
    }
    return 0;
}
