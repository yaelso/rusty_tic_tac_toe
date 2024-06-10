use crate::player::Player;
use crate::cell::Cell;

pub struct Board {
    pub size: usize,
    pub cells: Vec<Vec<Cell>>,
}

impl Board {
    pub fn new(size: usize) -> Board {
        Board {
            size,
            cells: vec![vec![Cell(None); size]; size],
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
        if x >= self.size || y >= self.size || self.cells[x][y].0.is_some() {
            return false;
        }
        self.cells[x][y] = Cell(Some(player));
        true
    }

    pub fn check_winner(&self) -> Option<Player> {
        // Check rows
        for row in &self.cells {
            if row[0].0.is_some() && row.iter().all(|cell| cell.0 == row[0].0) {
                return row[0].0;
            }
        }

        // Check columns
        for col in 0..self.size {
            if self.cells[0][col].0.is_some() && (0..self.size).all(|row| self.cells[row][col].0 == self.cells[0][col].0) {
                return self.cells[0][col].0;
            }
        }

        // Check diagonals
        if self.cells[0][0].0.is_some() && (0..self.size).all(|i| self.cells[i][i].0 == self.cells[0][0].0) {
            return self.cells[0][0].0;
        }
        if self.cells[0][2].0.is_some() && (0..self.size).all(|i| self.cells[i][self.size - 1 - 1].0 == self.cells[0][self.size - 1].0) {
            return self.cells[0][self.size - 1].0;
        }
        None
    }

    pub fn is_full(&self) -> bool {
        self.cells.iter().flatten().all(|cell| cell.0.is_some())
    }
}
