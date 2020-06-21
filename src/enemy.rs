use nalgebra::Vector2;

type Fec2 = Vector2<f32>;

#[derive(Clone)]
pub struct Enemy {
    pub position: Fec2,
}

impl Enemy {
    pub fn new(starting_position: Fec2) -> Self {
        Enemy {
            position: starting_position,
        }
    }
}
