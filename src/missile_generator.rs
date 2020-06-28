use std::collections::HashSet;
use ggez::event::KeyCode;
use nalgebra::Vector2;
use ggez::{graphics, Context, GameResult};
use std::f64::consts::PI;

use crate::missile;
use crate::homing_missile;
use crate::straight_missile;
use crate::enemy_group;

type Fec2 = Vector2<f32>;

const MISSILE_GENERATOR_HEIGHT: f32 = 11.0;
const MISSILE_GENERATOR_WIDTH: f32 = 11.0;

const MINIMUM_GENERATOR_RADIUS: f32 = 15.0;
const MAXIMUM_GENERATOR_RADIUS: f32 = 30.0;

fn check_bounds(missile: &Box<dyn missile::Missile>) -> bool {
    missile.get_position()[1] > 0.0
    && missile.get_position()[1] < 1080.0
    && missile.get_position()[0] > 0.0
    && missile.get_position()[0] < 1920.0
    && missile.get_collided() == false
}

pub struct MissileGenerator {
    position: Fec2,
    pub missile_list: Vec<Box<dyn missile::Missile>>,
    missile_toggle: i32,
    iteration: i32,
    radius: f32,
    rotation_in_radians: f32,
    spritebatch_len: usize,
}

impl MissileGenerator {
    pub fn new(position: Fec2, radius: f32, rotation: f32, spritebatch_len: usize) -> Self {
        let missile_list: Vec<Box<dyn missile::Missile>> = Vec::new();

        MissileGenerator {
            position: position,
            missile_list: missile_list,
            missile_toggle: 0,
            iteration: 0,
            radius: radius,
            rotation_in_radians: rotation,
            spritebatch_len: spritebatch_len,
        }
    }

    pub fn add_missile(&mut self) {
        if self.missile_toggle % 2 == 0 {
            let temp_rotation = (3.0*PI/2.0) as f32;

            let missile_generator_offset_position = Fec2::new(self.position[0] + MISSILE_GENERATOR_WIDTH/2.0, self.position[1] + MISSILE_GENERATOR_HEIGHT/2.0 - 10.0);

            let iteration_mod = (self.iteration/2) as usize % self.spritebatch_len;
            let new_missile = homing_missile::HomingMissile::new(missile_generator_offset_position, 0.0, 5.0, temp_rotation, iteration_mod);

            self.missile_list.push(Box::new(new_missile));

            self.iteration += 1;
        }

        self.missile_toggle += 1;
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.draw_missile_generator(ctx)?;
        Ok(())
    }

    pub fn update(&mut self, player_position: Fec2, enemies: &mut enemy_group::EnemyGroup) {
        self.increment_orbit(player_position);
        self.update_missiles(enemies);
    }

    pub fn increment_orbit(&mut self, player_position: Fec2) {
        let new_x = player_position[0] + self.radius * self.rotation_in_radians.cos();
        let new_y = player_position[1] + self.radius * self.rotation_in_radians.sin();
        self.position = Fec2::new(new_x, new_y);

        self.rotation_in_radians += 0.04;

        if self.rotation_in_radians > 6.28 {
            self.rotation_in_radians = 0.0;
        }
    }

    pub fn handle_input(&mut self, pressed_keys: &HashSet<KeyCode>) {
        if pressed_keys.contains(&KeyCode::LShift) {
            self.radius = (self.radius - 1.0).max(MINIMUM_GENERATOR_RADIUS);
        } else {
            self.radius = (self.radius + 1.0).min(MAXIMUM_GENERATOR_RADIUS);
        }
    }

    fn update_missiles(&mut self, enemies: &mut enemy_group::EnemyGroup) {
        for m in self.missile_list.iter_mut() {
            m.update(enemies);
        }

        self.missile_list.retain(|missile| 
            check_bounds(missile)
        );
    }

    fn draw_missile_generator(&mut self, ctx: &mut Context) -> GameResult<()> {
        let missile_generator_color = [0.0, 0.0, 1.0, 0.3].into();

        let player = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                self.position[0],
                self.position[1],
                MISSILE_GENERATOR_HEIGHT,
                MISSILE_GENERATOR_WIDTH 
            ),
            missile_generator_color
        )?;

        graphics::draw(ctx, &player, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
        Ok(())
    }
}
