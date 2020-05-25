use crate::position;

use ggez::{Context, GameResult};

#[derive(PartialEq, Eq, Hash)]
pub struct Missile {
    pub current_position: position::Position
}

impl Missile {
    pub fn new(pos: position::Position) -> Self {
        Missile {
            current_position: pos,
        }
    }

    pub fn get_new_position(&mut self) -> position::Position {
        self.current_position.y -= 10;
        self.current_position
    }
}
