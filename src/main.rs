extern crate rand;

use std::path;
use ggez::{input, graphics, Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler, KeyCode};
use ggez::timer;
use ggez::nalgebra;

mod missile;
mod position;
mod player;

const DESIRED_FPS: u32 = 60;

const MISSILE_GAPS: [i32; 3] = [10, 20, 30];
const MISSILE_WIDTH: i32 = 8;

fn main() {
    let resource_dir = path::PathBuf::from("./static");

    let (mut ctx, mut event_loop) = ContextBuilder::new("cuban-missie-crisis-test", "Andy")
        .window_setup(ggez::conf::WindowSetup::default().title("missile-crisis"))
        .add_resource_path(resource_dir)
		.build()
		.expect("Could not create ggez context!");


    let mut missile_carnival = State::new(&mut ctx);

    match event::run(&mut ctx, &mut event_loop, &mut missile_carnival) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}

struct State {
    player: player::Player,
    missiles: Vec<missile::Missile>,
    spritebatch: graphics::spritebatch::SpriteBatch,
}

impl State {
    pub fn new(_ctx: &mut Context) -> State {
        let list: Vec<missile::Missile> = Vec::new();
        let initial_position = position::Position::new(400, 600);

        let image = graphics::Image::new(_ctx, "/colorful-missile.png").unwrap();
        let batch = graphics::spritebatch::SpriteBatch::new(image);

        let state = State {
            player: player::Player::new(initial_position),
            missiles: list,
            spritebatch: batch,
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
            self.player.shift(0, incrementer * -1);
        }

        if pressed_keys.contains(&KeyCode::Down) {
            self.player.shift(0, incrementer * 1);
        }

        if pressed_keys.contains(&KeyCode::Left) {
            self.player.shift(incrementer * -1, 0);
        }

        if pressed_keys.contains(&KeyCode::Right) {
            self.player.shift(incrementer * 1, 0);
        }

        if pressed_keys.contains(&KeyCode::Z) {
            let x = self.player.position.x;
            let y = self.player.position.y;

            let right_position = position::Position::new(x + player::PLAYER_WIDTH + MISSILE_GAPS[0], y);
            let left_position = position::Position::new(x - MISSILE_GAPS[0] - MISSILE_WIDTH, y);
            let right_right_position = position::Position::new(x + player::PLAYER_WIDTH + MISSILE_GAPS[1], y);
            let left_left_position = position::Position::new(x - MISSILE_GAPS[1] - MISSILE_WIDTH, y);

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
        graphics::clear(ctx, graphics::WHITE);

        for m in &self.missiles {
            let p = graphics::DrawParam::new()
                .dest(nalgebra::Point2::new(m.current_position.x as f32, m.current_position.y as f32));

            self.spritebatch.add(p);
        }

        self.player.draw(ctx)?;

        let param = graphics::DrawParam::new();

        graphics::draw(ctx, &self.spritebatch, param)?;
        self.spritebatch.clear();

        graphics::present(ctx)?;

        println!("{:0}", ggez::timer::fps(ctx));
        Ok(())
    }
}