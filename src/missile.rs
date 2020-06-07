use crate::position;

#[derive(PartialEq, Eq, Hash)]
pub struct Missile {
    x_increment: i8,
    initial_position: position::Position,
    pub current_position: position::Position
}

impl Missile {
    pub fn new(direction: i8, pos: position::Position) -> Self {
        Missile {
            x_increment: direction,
            initial_position: pos,
            current_position: pos,
        }
    }

    pub fn get_new_position(&mut self) -> position::Position {
        self.current_position.x += self.x_increment as i32;
        self.current_position.y = (self.current_position.y) - ((self.current_position.x - self.initial_position.x).pow(3)/(self.current_position.x - self.initial_position.x).pow(2)).abs();
        self.current_position
    }
}
