extern crate rand;

use ggez::{input, graphics, Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler, KeyCode};
use ggez::timer;

mod missile;
mod position;

const DESIRED_FPS: u32 = 60;


fn main() {    
    let (mut ctx, mut event_loop) = ContextBuilder::new("cuban-missie-crisis-test", "Andy")
        .window_setup(ggez::conf::WindowSetup::default().title("missile-crisis"))
		.build()
		.expect("Could not create ggez context!");


    let mut missile_carnival = State::new(&mut ctx);

    match event::run(&mut ctx, &mut event_loop, &mut missile_carnival) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}

struct State {
    missiles: Vec<missile::Missile>,
    missile_spawning_position: position::Position
}

impl State {
    pub fn new(_ctx: &mut Context) -> State {
        let list: Vec<missile::Missile> = Vec::new();
        let initial_position = position::Position::new(400, 600);

        let state = State {
            missiles: list,
            missile_spawning_position: initial_position,
        };

        state
    }
}

impl EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {

        let pressed_keys = input::keyboard::pressed_keys(ctx);
        let mut incrementer = 2;

        if pressed_keys.contains(&KeyCode::LShift) {
            incrementer = 1;
        }

        if pressed_keys.contains(&KeyCode::Up) {
            self.missile_spawning_position.y -= incrementer;     
        }

        if pressed_keys.contains(&KeyCode::Down) {
            self.missile_spawning_position.y += incrementer;
        }

        if pressed_keys.contains(&KeyCode::Left) {
            self.missile_spawning_position.x -= incrementer;
        }

        if pressed_keys.contains(&KeyCode::Right) {
            self.missile_spawning_position.x += incrementer;
        }

        if pressed_keys.contains(&KeyCode::Z) {
            let right_position = position::Position::new(self.missile_spawning_position.x + 15, self.missile_spawning_position.y);
            let left_position = position::Position::new(self.missile_spawning_position.x - 15, self.missile_spawning_position.y);
            let right_right_position = position::Position::new(self.missile_spawning_position.x + 30, self.missile_spawning_position.y);
            let left_left_position = position::Position::new(self.missile_spawning_position.x - 30, self.missile_spawning_position.y);

            let new_right_right_missile = missile::Missile::new(right_right_position);
            let new_left_left_missile = missile::Missile::new(left_left_position);
            let new_right_missile = missile::Missile::new(right_position);
            let new_left_missile = missile::Missile::new(left_position);

            self.missiles.push(new_right_missile);
            self.missiles.push(new_left_missile);
            self.missiles.push(new_right_right_missile);
            self.missiles.push(new_left_left_missile);
        }

        while timer::check_update_time(ctx, DESIRED_FPS) {            
            for m in self.missiles.iter_mut() {
                m.get_new_position();
            }

            self.missiles.retain(|x| x.current_position.y > -1);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // let missile_image = graphics::Image::new(ctx, "../static/graze-box-rg.png")?;
        // let missile_batch = graphics::spritebatch::SpriteBatch::new(missile_image);
    

        graphics::clear(ctx, graphics::WHITE);

        let player_color = [0.0, 0.0, 1.0, 1.0].into();
        let missile_color = [1.0, 0.0, 0.0, 1.0].into();

        let missile_spawner = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new_i32(
                self.missile_spawning_position.x,
                self.missile_spawning_position.y,
                10,
                10
            ),
            player_color
        )?;

        graphics::draw(ctx, &missile_spawner, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;

        for m in &self.missiles {
            let rectangle = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new_i32(
                    m.current_position.x,
                    m.current_position.y,
                    10,
                    10
                ),
                missile_color
            )?;

            graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
        }
        
        graphics::present(ctx)?;
        Ok(())
    }
}