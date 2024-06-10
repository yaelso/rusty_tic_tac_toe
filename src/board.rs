use crate::player::Player;
use crate::cell::Cell;

pub struct Board {
    cells: [[Cell; 3]; 3],
}

impl Board {
    pub fn new() -> Board {
        Board {
            cells: [[Cell(None); 3]; 3],
        }
    }

    pub fn display(&self) {
        for row in &self.cells {
            for cell in row {
                match cell.0 {
                    Some(Player::X) => print!(" X "),
                    Some(Player::O) => print!(" O "),
                    None => print!(" . "),
                }
            }
            println!();
        }
    }

    pub fn make_move(&mut self, x: usize, y: usize, player: Player) -> bool {
        if x >= 3 || y >= 3 || self.cells[x][y].0.is_some() {
            return false;
        }
        self.cells[x][y] = Cell(Some(player));
        true
    }

    pub fn check_winner(&self) -> Option<Player> {
        // Check rows
        for row in &self.cells {
            if row[0].0.is_some() && row[0].0 == row[1].0 && row[1].0 == row[2].0 {
                return row[0].0;
            }
        }

        // Check columns
        for col in 0..3 {
            if self.cells[0][col].0.is_some() && self.cells[0][col].0 == self.cells[1][col].0 && self.cells[1][col].0 == self.cells[2][col].0 {
                return self.cells[0][col].0;
            }
        }

        // Check diagonals
        if self.cells[0][0].0.is_some() && self.cells[0][0].0 == self.cells[1][1].0 && self.cells[1][1].0 == self.cells[2][2].0 {
            return self.cells[0][0].0;
        }
        if self.cells[0][2].0.is_some() && self.cells[0][2].0 == self.cells[1][1].0 && self.cells[1][1].0 == self.cells[2][0].0 {
            return self.cells[0][2].0;
        }
        None
    }

    pub fn is_full(&self) -> bool {
        self.cells.iter().flatten().all(|cell| cell.0.is_some())
    }
}
