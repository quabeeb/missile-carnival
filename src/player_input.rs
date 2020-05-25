#[derive(Clone, Copy)]
pub struct PlayerInput {
    pub up : bool,
    pub down : bool,
    pub left : bool,
    pub right : bool,    
}

impl PlayerInput {
    pub fn new() -> PlayerInput {
        PlayerInput {
            up : false,
            down : false,
            left : false,
            right : false
        }
    }
}