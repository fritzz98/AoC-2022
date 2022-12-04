use std::fs;
use std::str::Split;

fn main() {
    let file_contents = fs::read_to_string("input.txt").expect("error");

    let matches: Split<&str> = file_contents.split("\n");

    let mut sum: u128 = 0;

    for _match in matches {
        if _match.len() > 0 {
            let each_move = _match.as_bytes();
            let mapped_moves: (u8, u8) = map_input(each_move[0], each_move[2]);

            sum += calc_score(mapped_moves.0, mapped_moves.1);
        }
    }

    println!("{}", sum);
}

//0|1|2 vs 0|1|2
fn calc_score(opponent_move: u8, my_move: u8) -> u128 {
    let tuple = (opponent_move, my_move);
    let mut sum = 0;

    match tuple {
        (1, 1) => sum = 1 + 3, // R + D
        (2, 1) => sum = 1 + 0, // R + L
        (3, 1) => sum = 1 + 6, // R + W

        (1, 2) => sum = 2 + 6, // P + W
        (2, 2) => sum = 2 + 3, // P + D
        (3, 2) => sum = 2 + 0, // P + L

        (1, 3) => sum = 3 + 0, // S + L
        (2, 3) => sum = 3 + 6, // S + W
        (3, 3) => sum = 3 + 3, // S + D

        (_, _) => println!("wtf"),
    }

    sum
}

// 88 = X => 1 ROCK
// 89 = Y => 2 PAPER
// 90 = Z => 3 SCISSORS
// 65 = A => 1 ROCK
// 66 = B => 2 PAPER
// 67 = C => 3 SCISSORS
fn map_input(opponent_move: u8, my_move: u8) -> (u8, u8) {
    let mut res = (0, 0);

    match opponent_move {
        65 => res.0 = 1,
        66 => res.0 = 2,
        67 => res.0 = 3,
        _ => println!("Faulty data, {}", opponent_move.to_ascii_uppercase()),
    }

    match my_move {
        88 => res.1 = calc_my_move(res.0, 1),
        89 => res.1 = calc_my_move(res.0, 2),
        90 => res.1 = calc_my_move(res.0, 3),
        _ => println!("Faulty data"),
    }

    res
}

// 1 = L
// 2 = D
// 3 = W
fn calc_my_move(opponent_move: u8, outcome: u8) -> u8 {
    let mut my_move = 0;

    if outcome == 1 {
        // lose
        match opponent_move {
            1 => my_move = 3, // R
            2 => my_move = 1, // P
            3 => my_move = 2, // S
            _ => my_move = 0,
        }
    } else if outcome == 2 {
        // draw
        match opponent_move {
            1 => my_move = 1, // R
            2 => my_move = 2, // P
            3 => my_move = 3, // S
            _ => my_move = 0,
        }
    } else if outcome == 3{
        // win
        match opponent_move {
            1 => my_move = 2, // R
            2 => my_move = 3, // P
            3 => my_move = 1, // S
            _ => my_move = 0,
        }
    }

    my_move
}
