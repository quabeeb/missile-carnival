use std::collections::HashSet;
use ggez::event::KeyCode;
use nalgebra::Vector2;
use ggez::{graphics, Context, GameResult};
use std::f64::consts::PI;

use crate::homing_missile;
use crate::straight_missile;
use crate::enemy;
use crate::enemy_group;

type Fec2 = Vector2<f32>;

const MISSILE_GENERATOR_HEIGHT: i32 = 10;
const MISSILE_GENERATOR_WEIGHT: i32 = 10;

const MINIMUM_GENERATOR_RADIUS: f32 = 15.0;
const MAXIMUM_GENERATOR_RADIUS: f32 = 30.0;

pub struct MissileGenerator {
    position: Fec2,
    pub missile_list: Vec<homing_missile::Missile>,
    missile_toggle: i32,
    iteration: i32,
    radius: f32,
    rotation_in_radians: f32,
}

impl MissileGenerator {
    pub fn new(position: Fec2, radius: f32, rotation: f32) -> Self {
        let missile_list: Vec<homing_missile::Missile> = Vec::new();

        MissileGenerator {
            position: position,
            missile_list: missile_list,
            missile_toggle: 0,
            iteration: 0,
            radius: radius,
            rotation_in_radians: rotation,
        }
    }

    pub fn add_missile(&mut self) {
        if self.missile_toggle % 2 == 0 {
            let temp_rotation = (PI/2.0) as f32;
            // let temp_rotation = 4.71 - self.rotation_in_radians;

            let iteration_mod = (self.iteration/4 % 11) as usize;
            // let new_missile = straight_missile::Missile::new(self.position, 0.0, -5.0, temp_rotation, iteration_mod);
            let new_missile = homing_missile::Missile::new(self.position, 0.0, -5.0, temp_rotation, iteration_mod);

            self.missile_list.push(new_missile);

            self.iteration += 1;
        }

        self.missile_toggle += 1;
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.draw_missile_generator(ctx)?;
        Ok(())
    }

    pub fn update(&mut self, player_position: Fec2, enemies: &enemy_group::EnemyGroup) {
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

    fn update_missiles(&mut self, enemies: &enemy_group::EnemyGroup) {
        for m in self.missile_list.iter_mut() {
            m.update_homing_missile(enemies);
        }

        self.missile_list.retain(|missile| 
            missile.position[1] > 0.0
            && missile.position[1] < 600.0
            && missile.position[0] > 0.0
            && missile.position[0] < 800.0
        );
    }


    fn draw_missile_generator(&mut self, ctx: &mut Context) -> GameResult<()> {
        let missile_generator_color = [1.0, 0.0, 1.0, 1.0].into();

        let player = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new_i32(
                self.position[0] as i32,
                self.position[1] as i32,
                MISSILE_GENERATOR_HEIGHT,
                MISSILE_GENERATOR_WEIGHT 
            ),
            missile_generator_color
        )?;

        graphics::draw(ctx, &player, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
        Ok(())
    }
}
