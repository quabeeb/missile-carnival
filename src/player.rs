use ggez::{graphics, Context, GameResult};
use ggez::nalgebra::Vector2;

use std::collections::HashSet;
use ggez::event::KeyCode;

type Fec2 = Vector2<f32>;

pub const PLAYER_HEIGHT: f32 = 10.0;
pub const PLAYER_WIDTH: f32 = 10.0;

pub struct Player {
    pub position: Fec2
}

impl Player {
    pub fn handle_input(&mut self, pressed_keys: &HashSet<KeyCode>) {
        let mut incrementer = 2.0;

        if pressed_keys.contains(&KeyCode::LShift) {
            incrementer = 1.0;
        }

        if pressed_keys.contains(&KeyCode::Up) {
            self.position -= Fec2::new(0.0, incrementer);
        }

        if pressed_keys.contains(&KeyCode::Down) {
            self.position += Fec2::new(0.0, incrementer);
        }

        if pressed_keys.contains(&KeyCode::Left) {
            self.position -= Fec2::new(incrementer, 0.0);
        }

        if pressed_keys.contains(&KeyCode::Right) {
            self.position += Fec2::new(incrementer, 0.0);
        }
    }

    pub fn new(starting_position: Fec2) -> Player {
        Player{ position: starting_position }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()>{
        let player_color = [0.0, 0.0, 1.0, 1.0].into();

        let player = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new_i32(
                self.position[0] as i32,
                self.position[1] as i32,
                PLAYER_HEIGHT as i32,
                PLAYER_WIDTH as i32
            ),
            player_color
        )?;

        graphics::draw(ctx, &player, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
        Ok(())
    }
}