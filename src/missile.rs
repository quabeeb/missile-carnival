use nalgebra::Vector2;
type Fec2 = Vector2<f32>;

use crate::enemy_group;

pub trait Missile {
    fn update(&mut self, enemies: &enemy_group::EnemyGroup);
    fn get_position(&self) -> Fec2;
    fn get_draw_rotation(&self) -> f32;
    fn get_spritebatch_index(&self) -> usize;
}