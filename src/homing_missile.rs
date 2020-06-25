use nalgebra::{Vector2, Vector3};
use nalgebra::base::Unit;
use std::f64::consts::PI;

use crate::missile;
use crate::enemy_group;
use crate::enemy;

const MAX_MISSILE_VELOCITY: f32 = 15.0;

type Fec2 = Vector2<f32>;
type Fec3 = Vector3<f32>;

pub struct HomingMissile {
    pub rotation: f32,
    pub draw_rotation: f32,
    pub rotation_vec: Fec2,
    pub position: Fec2,
    velocity: f32,
    acceleration: f32,
    pub spritebatch_index: usize,
}

fn vec_from_rotation(rotation: f32) -> Fec2 {
    let vx = rotation.cos();
    let vy = rotation.sin();
    Fec2::new(vx, vy)
}

fn fec3ify(vec2: Fec2) -> Fec3 {
    Fec3::new(vec2[0], vec2[1], 0.0)
}

impl HomingMissile {
    pub fn new(position: Fec2, velocity: f32, acceleration: f32, rotation: f32, spritebatch_index: usize) -> Self {
        let rotation_vec = vec_from_rotation(rotation);

        HomingMissile {
            rotation: rotation,
            draw_rotation: rotation + (PI/2.0) as f32,
            rotation_vec: rotation_vec,
            position: position,
            velocity: velocity,
            acceleration: acceleration,
            spritebatch_index: spritebatch_index,
        }
    }

    fn get_closest_enemy(&self, enemy_group: &enemy_group::EnemyGroup) -> Option<enemy::Enemy> {
        let mut min_distance: f32 = -1.0;
        let mut closest_enemy: Option<enemy::Enemy> = None;

        for enemy in &enemy_group.enemy_list {
            let distance_vec = self.position - enemy.targeting_position;
            let distance: f32 = distance_vec[0].powf(2.0) + distance_vec[1].powf(2.0);

            if min_distance == -1.0 {
                min_distance = distance;
            }
            
            if distance <= min_distance {
                min_distance = distance;
                closest_enemy = Some(enemy::Enemy::new(Fec2::new(enemy.targeting_position[0], enemy.targeting_position[1])));
            }
        }

        closest_enemy
    }

    pub fn update_homing_missile(&mut self, enemies: &enemy_group::EnemyGroup) {
        let target = self.get_closest_enemy(enemies);

        match target {
            Some(x) => {
                self.set_new_position(&x);
            },
            None => {
                self.set_new_position_empty();                  
            }
        } 
    }

    pub fn set_new_position_empty(&mut self) {
        self.position += self.rotation_vec * self.velocity;
        self.velocity = MAX_MISSILE_VELOCITY.min(self.velocity + self.acceleration);
    }

    pub fn set_new_position(&mut self, target: &enemy::Enemy) {
        self.position += self.rotation_vec * self.velocity;
        self.velocity = MAX_MISSILE_VELOCITY.min(self.velocity + self.acceleration);

        let desired_direction = Unit::new_normalize(fec3ify(target.position - self.position));
        let current_direction = Unit::new_normalize(fec3ify(self.rotation_vec));
        let rotate_amount = desired_direction.cross(current_direction.as_ref())[2];

        self.rotation -= rotate_amount/2.0;
        self.draw_rotation = self.rotation + (PI/2.0) as f32;

        self.rotation_vec = vec_from_rotation(self.rotation);
    }
}

impl missile::Missile for HomingMissile {
    fn update(&mut self, enemies: &enemy_group::EnemyGroup) {
        self.update_homing_missile(enemies);
    }

    fn get_position(&self) -> Fec2 {
        self.position
    }

    fn get_spritebatch_index(&self) -> usize {
        self.spritebatch_index
    }

    fn get_draw_rotation(&self) -> f32 {
        self.draw_rotation
    }
}
