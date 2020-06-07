use ggez::nalgebra::Vector2;

type Fec2 = Vector2<f32>;
const MAX_MISSILE_VELOCITY: f32 = -20.0;

pub struct Missile {
    pub rotation_vec: Fec2,
    pub position: Fec2,
    velocity: f32,
    acceleration: f32,
}

fn vec_from_rotation(rotation: f32) -> Vector2<f32> {
    let vx = rotation.sin();
    let vy = rotation.cos();
    Vector2::new(vx, vy)
}

impl Missile {
    pub fn new(position: Fec2, velocity: f32, acceleration: f32, rotation: f32) -> Self {
        let rotation_vec = vec_from_rotation(rotation);

        Missile {
            rotation_vec: rotation_vec,
            position: position,
            velocity: velocity,
            acceleration: acceleration,
        }
    }

    pub fn set_new_position(&mut self) {
        self.position += self.rotation_vec * self.velocity;
        self.velocity = MAX_MISSILE_VELOCITY.max(self.velocity + self.acceleration);
    }
}
