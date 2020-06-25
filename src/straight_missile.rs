use nalgebra::Vector2;

use crate::missile;
use crate::enemy_group;

type Fec2 = Vector2<f32>;
const MAX_MISSILE_VELOCITY: f32 = 10.0;

pub struct StraightMissile {
    pub rotation_vec: Fec2,
    pub position: Fec2,
    velocity: f32,
    acceleration: f32,
    pub spritebatch_index: usize,
}

fn vec_from_rotation(rotation: f32) -> Vector2<f32> {
    let vx = rotation.sin();
    let vy = rotation.cos();
    Vector2::new(vx, vy)
}

impl StraightMissile {
    pub fn new(position: Fec2, velocity: f32, acceleration: f32, rotation: f32, spritebatch_index: usize) -> Self {
        let rotation_vec = vec_from_rotation(rotation);

        StraightMissile {
            rotation_vec: rotation_vec,
            position: position,
            velocity: velocity,
            acceleration: acceleration,
            spritebatch_index: spritebatch_index,
        }
    }

    pub fn set_new_position(&mut self) {
        self.position += self.rotation_vec * self.velocity;
        self.velocity = MAX_MISSILE_VELOCITY.min(self.velocity + self.acceleration);
    }
}

// impl missile::Missile for StraightMissile {
//     fn update(&mut self, enemies: &enemy_group::EnemyGroup) {
//         self.set_new_position();
//     }

//     fn get_position(&self){}

//     fn get_rotation(&self){}

//     fn get_draw_rotation(&self){}
// }