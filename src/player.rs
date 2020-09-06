use std::collections::HashSet;
use ggez::event::KeyCode;
use ggez::{graphics, Context, GameResult, nalgebra::Point2};
use nalgebra::Vector2;

use std::f64::consts::PI;

use crate::enemies::enemy_group;
use crate::missiles::missile_generator;

type Fec2 = Vector2<f32>;

const PLAYER_HEIGHT: f32 = 10.0;
const PLAYER_WIDTH: f32 = 10.0;

pub struct Player {
    pub position: Fec2,
    spritebatches: Vec<graphics::spritebatch::SpriteBatch>,
    missile_generator_list: Vec<missile_generator::MissileGenerator>
}

impl Player {
    pub fn new(starting_position: Fec2, spritebatches: Vec<graphics::spritebatch::SpriteBatch>) -> Player {
        let mut missile_generator_list: Vec<missile_generator::MissileGenerator> = Vec::new();

        let radius = 30.0;
        let spritebatches_len = spritebatches.len();

        let right_missile_generator = missile_generator::MissileGenerator::new(starting_position, radius, 0.0, spritebatches_len);
        let bottom_missile_generator = missile_generator::MissileGenerator::new(starting_position, radius, (PI/2.0) as f32, spritebatches_len);
        let left_missile_generator = missile_generator::MissileGenerator::new(starting_position, radius, PI as f32, spritebatches_len);
        let top_missile_generator = missile_generator::MissileGenerator::new(starting_position, radius, (PI*3.0/2.0) as f32, spritebatches_len);

        missile_generator_list.push(right_missile_generator);
        missile_generator_list.push(bottom_missile_generator);
        missile_generator_list.push(left_missile_generator);
        missile_generator_list.push(top_missile_generator);        
        
        Player {
            position: starting_position,
            spritebatches: spritebatches,
            missile_generator_list: missile_generator_list,
        }
    }

    pub fn update(&mut self, enemies: &mut enemy_group::EnemyGroup) {
        for m in self.missile_generator_list.iter_mut() {
            m.update(self.position, enemies);
        }
    }

    pub fn handle_input(&mut self, pressed_keys: &HashSet<KeyCode>) {
        let mut incrementer = 5.0;

        if pressed_keys.contains(&KeyCode::LShift) {
            incrementer = 2.0;
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

        for m in self.missile_generator_list.iter_mut() {
            m.handle_input(pressed_keys);
        }        
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
        let player_color = [1.0, 0.4, 0.7, 1.0].into();

        let player = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                self.position[0],
                self.position[1],
                PLAYER_HEIGHT,
                PLAYER_WIDTH
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
                let missile_position = missile.get_position();
                let missile_draw_rotation = missile.get_draw_rotation();
                let missile_spritebatch_index = missile.get_spritebatch_index();

                let p = graphics::DrawParam::new()
                    .dest(Point2::new(missile_position[0], missile_position[1]))
                    .rotation(missile_draw_rotation)
                    .offset(Point2::new(0.5, 0.5));

                self.spritebatches[missile_spritebatch_index].add(p);
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
