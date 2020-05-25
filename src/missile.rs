use crate::position;

use ggez::{Context, GameResult};

#[derive(PartialEq, Eq, Hash)]
pub struct Missile {
    initial_position: position::Position,
    pub current_position: position::Position,
    timesteps: i32
}

impl Missile {
    pub fn new(pos: position::Position) -> Self {
        Missile {
            initial_position: pos,
            current_position: pos,
            timesteps: 0
        }
    }

    pub fn get_new_position(&mut self) -> position::Position {
        let ip = self.initial_position;

        self.timesteps += 1;

        let new_x = ip.x + self.timesteps;
        let new_y = 600 - self.timesteps.pow(2)/10;
        
        let new_position = position::Position::new(new_x, new_y);
        self.current_position = new_position;

        new_position
    }

    pub fn reset(&mut self) {
        self.timesteps = 0;
    }

    // fn update(&mut self) {

    // }

    // fn draw(&self, ctx: &mut Context) -> GameResult<()> {
    //     Ok(())
    // }
}
