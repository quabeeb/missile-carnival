use nalgebra::Vector2;

type Fec2 = Vector2<f32>;

const ENEMY_WIDTH: f32 = 10.0;
const ENEMY_HEIGHT: f32 = 10.0;

#[derive(Clone)]
pub struct Enemy {
    pub position: Fec2,
    pub targeting_position: Fec2,
}

impl Enemy {
    pub fn new(starting_position: Fec2) -> Self {
        let targeting_position = Fec2::new(starting_position[0] + ENEMY_WIDTH/2.0, starting_position[1] + ENEMY_HEIGHT/2.0);

        Enemy {
            position: starting_position,
            targeting_position: targeting_position,
        }
    }
}
