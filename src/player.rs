use crate::position;

use ggez::{graphics, Context, GameResult};
use std::collections::HashSet;
use ggez::event::KeyCode;

pub const PLAYER_HEIGHT: i32 = 10;
pub const PLAYER_WIDTH: i32 = 10;

pub struct Player {
    pub position: position::Position
}

impl Player {
    pub fn handle_input(&mut self, pressed_keys: &HashSet<KeyCode>) {
        let mut incrementer = 2;

        if pressed_keys.contains(&KeyCode::LShift) {
            incrementer = 1;
        }

        if pressed_keys.contains(&KeyCode::Up) {
            self.position.y -= incrementer;
        }

        if pressed_keys.contains(&KeyCode::Down) {
            self.position.y += incrementer;
        }

        if pressed_keys.contains(&KeyCode::Left) {
            self.position.x -= incrementer;
        }

        if pressed_keys.contains(&KeyCode::Right) {
            self.position.x += incrementer;
        }
    }

    pub fn new(starting_position: position::Position) -> Player {
        Player{ position: starting_position }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()>{
        let player_color = [0.0, 0.0, 1.0, 1.0].into();
        let missile_spawner_color = [1.0, 0.0, 1.0, 1.0].into();

        let player = graphics::Mesh::new_rectangle(
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

        let missile_spawner_r = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new_i32(
                self.position.x + 20,
                self.position.y,
                PLAYER_HEIGHT,
                PLAYER_WIDTH
            ),
            missile_spawner_color
        )?;

        let missile_spawner_l = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new_i32(
                self.position.x - 20,
                self.position.y,
                PLAYER_HEIGHT,
                PLAYER_WIDTH
            ),
            missile_spawner_color
        )?;

        let missile_spawner_r_r = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new_i32(
                self.position.x + 40,
                self.position.y,
                PLAYER_HEIGHT,
                PLAYER_WIDTH
            ),
            missile_spawner_color
        )?;

        let missile_spawner_l_l = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new_i32(
                self.position.x - 40,
                self.position.y,
                PLAYER_HEIGHT,
                PLAYER_WIDTH
            ),
            missile_spawner_color
        )?;

        graphics::draw(ctx, &player, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
        graphics::draw(ctx, &missile_spawner_r, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
        graphics::draw(ctx, &missile_spawner_l, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
        graphics::draw(ctx, &missile_spawner_r_r, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
        graphics::draw(ctx, &missile_spawner_l_l, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;        

        Ok(())
    }
}