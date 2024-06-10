mod player;
mod cell;
mod board;

use std::io;
use crate::player::Player;
use crate::board::Board;

fn main() {
    println!("Enter the size of the tic tac toe board: ");
    let mut size_input = String::new();
    io::stdin().read_line(&mut size_input).expect("Failed to read line.");
    let size: usize = size_input.trim().parse().expect("Plese enter a valid number.");

    let mut board = Board::new(size);
    let mut current_player = Player::X;

    loop {
        board.display();
        println!("\nPlayer {}, enter your move as `row` and `column`, separated by a space and starting at position 0 0: ", current_player);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line.");
        let coords: Vec<usize> = input.trim().split_whitespace().filter_map(|s| s.parse().ok()).collect();

        if coords.len() != 2 {
            println!("\nInvalid input, please enter two numbers separated by space.");
            continue;
        }

        let (x, y) = (coords[0], coords[1]);

        if board.make_move(x, y, current_player) {
            if let Some(winner) = board.check_winner() {
                board.display();
                println!("\nPlayer {} wins!", winner);
                break;
            }
            if board.is_full() {
                board.display();
                println!("\nIt's a draw!");
                break;
            }
            current_player = match current_player {
                Player::X => Player::O,
                Player::O => Player::X,
            };
        } else {
            println!("\nInvalid move, try again.");
        }
    }
}
