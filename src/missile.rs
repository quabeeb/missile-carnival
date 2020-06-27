use nalgebra::Vector2;
use ncollide2d::bounding_volume::AABB;

type Fec2 = Vector2<f32>;

use crate::enemy_group;

pub trait Missile {
    fn update(&mut self, enemies: &enemy_group::EnemyGroup);
    fn get_position(&self) -> Fec2;
    fn get_draw_rotation(&self) -> f32;
    fn get_spritebatch_index(&self) -> usize;
    fn get_bounding_volume(&self) -> AABB<f32>;
    fn get_collided(&self) -> bool;
}