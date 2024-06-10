use crate::player::Player;

#[derive(Copy, Clone)]
pub struct Cell(pub Option<Player>);
