use nalgebra::Vector2;
use ncollide2d::math::Point;
use ncollide2d::bounding_volume::aabb::AABB;

type Fec2 = Vector2<f32>;

const ENEMY_WIDTH: f32 = 10.0;
const ENEMY_HEIGHT: f32 = 10.0;

#[derive(Copy, Clone)]
pub struct Enemy {
    pub position: Fec2,
    pub targeting_position: Fec2,
    pub health: i32,
    pub dead: bool,
}

impl Enemy {
    pub fn new(starting_position: Fec2) -> Self {
        let targeting_position = Fec2::new(starting_position[0] + ENEMY_WIDTH/2.0, starting_position[1] + ENEMY_HEIGHT/2.0);

        Enemy {
            position: starting_position,
            targeting_position: targeting_position,
            health: 1,
            dead: false,
        }
    }

    pub fn get_bounding_volume(&self) -> AABB<f32> {
        let top_left_point = Point::new(self.position[0], self.position[1]);
        let bot_right_point = Point::new(self.position[0] + ENEMY_WIDTH, self.position[1] + ENEMY_HEIGHT); // SHOULD BE TOP LEFT POINT + POINT::NEW(MISSILE WIDTH, HEIGHT)
        
        AABB::new(top_left_point, bot_right_point)
    }

    pub fn decrement_health(&mut self, damage: i32) {
        self.health -= damage;

        if self.health <= 0 {
            self.dead = true;
        }
    }
}
