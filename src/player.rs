use crate::position;

use ggez::{input, graphics, Context, ContextBuilder, GameResult};

pub const PLAYER_HEIGHT: i32 = 10;
pub const PLAYER_WIDTH: i32 = 10;

pub struct Player {
    pub position: position::Position
}

impl Player {
    pub fn new(starting_position: position::Position) -> Player {
        Player{ position: starting_position }
    }

    pub fn shift(&mut self, x_inc: i32, y_inc:i32) {
        self.position.x += x_inc;
        self.position.y += y_inc;
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()>{
        let player_color = [0.0, 0.0, 1.0, 1.0].into();

        let missile_spawner = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new_i32(
                self.position.x,
                self.position.y,
                PLAYER_HEIGHT,
                PLAYER_WIDTH
            ),
            player_color
        )?;

        graphics::draw(ctx, &missile_spawner, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;

        Ok(())
    }
}