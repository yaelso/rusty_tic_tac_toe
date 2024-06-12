mod player;
mod cell;
mod board;

use std::io;
use crate::player::Player;
use crate::board::Board;

fn main() {
    loop {
        println!("Enter a single dimension between 3-10 to determine the size of the tic tac toe board (e.g. 3 for a 3x3 grid): ");
        let size = loop {
            let mut size_input = String::new();
            io::stdin().read_line(&mut size_input).expect("Failed to read line.");
            match size_input.trim().parse::<usize>() {
                Ok(size) if size >= 3 && size <= 10 => break size,
                _ => println!("Invalid input! Please enter a number between 3 and 10: "),
            }
        };

        let mut board = Board::new(size);
        let mut current_player = Player::X;

        loop {
            board.display();
            println!("\nPlayer {}, enter your move as `row` and `column`, separated by a space and starting at position 0 0: ", current_player);

            let (x, y) = loop {
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line.");
                let coords: Vec<usize> = input.trim().split_whitespace().filter_map(|s| s.parse().ok()).collect();
                if coords.len() == 2 && coords[0] < size && coords[1] < size {
                    break (coords[0], coords[1]);
                } else {
                    println!("Invalid input! Please enter two numbers between 0 and {}, separated by a space: ", size - 1);
                }
            };

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

        println!("Do you want to play again? (y/n): ");
        let mut replay_input = String::new();
        io::stdin().read_line(&mut replay_input).expect("Failed to read line.");
        if replay_input.trim().to_lowercase() != "y" {
            break;
        }
    }
}
