use crate::position;

use ggez::{Context, GameResult};

#[derive(PartialEq, Eq, Hash)]
pub struct Missile {
    initial_position: position::Position,
    pub current_position: position::Position,
    timesteps: i32,
    incrementer: i32
}

impl Missile {
    pub fn new(pos: position::Position, increment: i32) -> Self {
        Missile {
            initial_position: pos,
            current_position: pos,
            timesteps: 0,
            incrementer: increment
        }
    }

    pub fn get_new_position(&mut self) -> position::Position {
        let ip = self.initial_position;

        self.timesteps += self.incrementer;

        let new_x = ip.x + self.timesteps;
        let new_y = ip.y - self.timesteps.pow(2)/5;
        
        let new_position = position::Position::new(new_x, new_y);
        self.current_position = new_position;

        new_position
    }
}
