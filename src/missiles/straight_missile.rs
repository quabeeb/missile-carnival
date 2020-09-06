use nalgebra::{Vector2};
use ncollide2d::math::Point;
use ncollide2d::bounding_volume::aabb::AABB;
use ncollide2d::math::Isometry;
use nalgebra::geometry::UnitComplex;
use std::f64::consts::PI;

use crate::missiles::missile::Missile;
use crate::enemies::enemy_group;

type Fec2 = Vector2<f32>;
const MAX_MISSILE_VELOCITY: f32 = 10.0;

pub struct StraightMissile {
    pub rotation_vec: Fec2,
    pub draw_rotation: f32,
    pub position: Fec2,
    velocity: f32,
    acceleration: f32,
    pub spritebatch_index: usize,
    pub collided: bool,
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
            draw_rotation: rotation + (PI/2.0) as f32,
            position: position,
            velocity: velocity,
            acceleration: acceleration,
            spritebatch_index: spritebatch_index,
            collided: false,
        }
    }

    pub fn set_new_position(&mut self) {
        self.position += self.rotation_vec * self.velocity;
        self.velocity = MAX_MISSILE_VELOCITY.min(self.velocity + self.acceleration);
    }
}

impl Missile for StraightMissile {
    fn update(&mut self, enemies: &mut enemy_group::EnemyGroup) {
        self.set_new_position();
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

    fn get_collided(&self) -> bool {
        self.collided
    }

    fn get_bounding_volume(&self) -> AABB<f32> {
        let top_left_point = Point::new(self.position[0], self.position[1]);

        let bot_right_point = Point::new(self.position[0] + 3.0, self.position[1] + 30.0); // SHOULD BE TOP LEFT POINT + POINT::NEW(MISSILE WIDTH, HEIGHT)
        
        let mut aabb = AABB::new(top_left_point, bot_right_point);

        let translation = Isometry::translation(-1.5, -15.0); // SHOULD BE .5*(MISSILE WIDTH, HEIGHT)
        aabb = aabb.transform_by(&translation);

        let rot = UnitComplex::new(self.draw_rotation);
        let rotation = Isometry::rotation_wrt_point(rot, aabb.center());
        aabb = aabb.transform_by(&rotation);

        aabb
    }
}