use nalgebra::Vector2;
use ncollide2d::bounding_volume::aabb::AABB;
type Fec2 = Vector2<f32>;

pub trait Enemy {
    fn get_status(&self) -> bool;
    fn decrement_health(&mut self, damage: i32);
    fn get_bounding_volume(&self) -> AABB<f32>;
    fn get_position(&self) -> Fec2;
    fn get_targeting_position(&self) -> Fec2;
}
