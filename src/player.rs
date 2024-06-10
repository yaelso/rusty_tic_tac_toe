use std::fmt;

#[derive(Copy, Clone, PartialEq)]
pub enum Player {
    X,
    O,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Player::X => "X",
            Player::O => "O",
        })
    }
}
