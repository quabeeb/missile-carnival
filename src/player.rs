use crate::straight_missile;

use ggez::event::KeyCode;
use ggez::{graphics, Context, GameResult};
use ggez::nalgebra;
use ggez::nalgebra::Vector2;

use std::collections::HashSet;

type Fec2 = Vector2<f32>;

const PLAYER_HEIGHT: f32 = 10.0;
const PLAYER_WIDTH: f32 = 10.0;

const MISSILE_GAPS: [f32; 3] = [10.0, 10.0, 10.0];
const MISSILE_WIDTH: f32 = 10.0;

pub struct Player {
    pub position: Fec2,
    spritebatches: Vec<graphics::spritebatch::SpriteBatch>,
    missile_list: Vec<straight_missile::Missile>,
    iteration: i32,
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

        if pressed_keys.contains(&KeyCode::Z) {
            let iteration_mod = (self.iteration % 11) as usize;

            let x = self.position[0];
            let y = self.position[1];

            let right_position = Fec2::new(x + PLAYER_WIDTH + MISSILE_GAPS[0], y);
            let left_position = Fec2::new(x - MISSILE_GAPS[0] - MISSILE_WIDTH, y);

            let new_right_missile = straight_missile::Missile::new(right_position, 0.0, -0.5, 0.0, iteration_mod);
            let new_left_missile = straight_missile::Missile::new(left_position, 0.0, -0.5, 0.0, iteration_mod);

            self.missile_list.push(new_right_missile);
            self.missile_list.push(new_left_missile);

            self.iteration += 1;
        }
    }

    pub fn new(starting_position: Fec2, spritebatches: Vec<graphics::spritebatch::SpriteBatch>) -> Player {
        let missile_list: Vec<straight_missile::Missile> = Vec::new();

        Player {
            position: starting_position,
            spritebatches: spritebatches,
            missile_list: missile_list,
            iteration: 0,
        }
    }

    pub fn update_missiles(&mut self) { 
        for m in self.missile_list.iter_mut() {
            m.set_new_position();
        }

        self.missile_list.retain(|x| x.position[1] > -20.0);
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.draw_player(ctx)?;
        self.draw_missiles(ctx)?;
        Ok(())
    }

    fn draw_player(&mut self, ctx: &mut Context) -> GameResult<()> {
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

    fn draw_missiles(&mut self, ctx: &mut Context) -> GameResult<()> {
        let param = graphics::DrawParam::new();

        for m in &self.missile_list {
            let p = graphics::DrawParam::new()
                .dest(nalgebra::Point2::new(m.position[0], m.position[1]));
            
            self.spritebatches[m.spritebatch_index].add(p);
        }

        for spritebatch in &self.spritebatches {
            graphics::draw(ctx, spritebatch, param)?;
        }

        for index in 0..self.spritebatches.len() {
            self.spritebatches[index].clear();
        }
        
        Ok(())
    }
}
