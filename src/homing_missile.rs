use nalgebra::{Vector2, Vector3};
use nalgebra::base::Unit;

use crate::enemy;

const MAX_MISSILE_VELOCITY: f32 = -10.0;

type Fec2 = Vector2<f32>;
type Fec3 = Vector3<f32>;

pub struct Missile {
    pub rotation: f32,
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

impl Missile {
    pub fn new(position: Fec2, velocity: f32, acceleration: f32, rotation: f32, spritebatch_index: usize) -> Self {
        let rotation_vec = vec_from_rotation(rotation);

        Missile {
            rotation: rotation,
            rotation_vec: rotation_vec,
            position: position,
            velocity: velocity,
            acceleration: acceleration,
            spritebatch_index: spritebatch_index,
        }
    }

    pub fn set_new_position_empty(&mut self) {
        self.position += self.rotation_vec * self.velocity;
        self.velocity = MAX_MISSILE_VELOCITY.max(self.velocity + self.acceleration);
    }

    pub fn set_new_position(&mut self, target: &enemy::Enemy) {
        self.position += self.rotation_vec * self.velocity;
        self.velocity = MAX_MISSILE_VELOCITY.max(self.velocity + self.acceleration);

        let desired_direction = Unit::new_normalize(fec3ify(target.position - self.position));
        let current_direction = Unit::new_normalize(fec3ify(self.rotation_vec));
        let rotate_amount = desired_direction.cross(current_direction.as_ref())[2];

        self.rotation += rotate_amount/5.0;
        self.rotation_vec = vec_from_rotation(self.rotation);
    }
}
