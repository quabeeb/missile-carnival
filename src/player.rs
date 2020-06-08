use ggez::event::KeyCode;
use ggez::{graphics, Context, GameResult};
use ggez::nalgebra::Vector2;
use ggez::nalgebra;

use crate::missile_generator;

use std::collections::HashSet;

type Fec2 = Vector2<f32>;

const PLAYER_HEIGHT: f32 = 10.0;
const PLAYER_WIDTH: f32 = 10.0;

const MISSILE_GAPS: [f32; 3] = [10.0, 10.0, 10.0];
const MISSILE_WIDTH: f32 = 10.0;

pub struct Player {
    pub position: Fec2,
    spritebatches: Vec<graphics::spritebatch::SpriteBatch>,
    missile_generator_list: Vec<missile_generator::MissileGenerator>
}

impl Player {
    pub fn handle_input(&mut self, pressed_keys: &HashSet<KeyCode>) {
        let mut incrementer = 3.0;

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
            for m in self.missile_generator_list.iter_mut() {
                m.add_missile();
            }
        }

        self.update();
    }

    pub fn new(starting_position: Fec2, spritebatches: Vec<graphics::spritebatch::SpriteBatch>) -> Player {
        let mut missile_generator_list: Vec<missile_generator::MissileGenerator> = Vec::new();

        let x = starting_position[0];
        let y = starting_position[1];

        let right_missile_generator = missile_generator::MissileGenerator::new(Fec2::new(x + PLAYER_WIDTH + MISSILE_GAPS[0] - 1.0, y));
        let left_missile_generator = missile_generator::MissileGenerator::new(Fec2::new(x - MISSILE_GAPS[0] - MISSILE_WIDTH, y));

        missile_generator_list.push(right_missile_generator);
        missile_generator_list.push(left_missile_generator);
        
        // let right_position = Fec2::new(x + PLAYER_WIDTH + MISSILE_GAPS[0] - 1.0, y); // -1.0 of origin
        // let left_position = Fec2::new(x - MISSILE_GAPS[0] - MISSILE_WIDTH, y);
        
        // let right_right_position = Fec2::new(x + PLAYER_WIDTH + MISSILE_GAPS[0] + MISSILE_GAPS[1] - 1.0, y); // -1.0 of origin
        // let left_left_position = Fec2::new(x - MISSILE_GAPS[0] - MISSILE_GAPS[1] - MISSILE_WIDTH, y);
        Player {
            position: starting_position,
            spritebatches: spritebatches,
            missile_generator_list: missile_generator_list,
        }
    }

    pub fn update(&mut self) { 
        let x = self.position[0];
        let y = self.position[1];

        self.missile_generator_list[0].update(Fec2::new(x + PLAYER_WIDTH + MISSILE_GAPS[0] - 1.0, y));
        self.missile_generator_list[1].update(Fec2::new(x - MISSILE_GAPS[0] - MISSILE_WIDTH, y));

        // for m in self.missile_generator_list.iter_mut() {
        //     m.update(Fec2::new(x + PLAYER_WIDTH + MISSILE_GAPS[0] - 1.0, y));
        // }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.draw_player(ctx)?;
        self.draw_missiles(ctx)?;

        for m in self.missile_generator_list.iter_mut() {
            m.draw(ctx)?;
        }
        
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

        for missile_generator in &self.missile_generator_list {
            for missile in &missile_generator.missile_list {
                let p = graphics::DrawParam::new()
                    .dest(nalgebra::Point2::new(missile.position[0] + 5.0, missile.position[1] + 5.0))
                    .rotation(0.785398)
                    .offset(nalgebra::Point2::new(0.5, 0.5));
                
                self.spritebatches[missile.spritebatch_index].add(p);
            }
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
