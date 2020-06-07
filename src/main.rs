extern crate rand;

use std::path;
use ggez::{input, graphics, Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler, KeyCode};
use ggez::timer;
use ggez::nalgebra::Vector2;
use ggez::filesystem;
use ggez::nalgebra;

mod straight_missile;
mod player;

type Fec2 = Vector2<f32>;

const DESIRED_FPS: u32 = 60;

const MISSILE_GAPS: [f32; 3] = [10.0, 10.0, 10.0];
const MISSILE_WIDTH: f32 = 10.0;

fn main() {
    let resource_dir = path::PathBuf::from("./resources");

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
    missiles: Vec<straight_missile::Missile>,
    spritebatches: Vec<graphics::spritebatch::SpriteBatch>,
    iteration: i32
}

impl State {
    pub fn new(_ctx: &mut Context) -> State {
        let image_vec = State::load_missile_sprites(_ctx);
        let list: Vec<straight_missile::Missile> = Vec::new();
        let initial_position = Fec2::new(400.0, 600.0);
        
        let state = State {
            player: player::Player::new(initial_position),
            missiles: list,
            spritebatches: image_vec,
            iteration: 0,
        };

        state
    }

    pub fn load_missile_sprites(ctx: &mut Context) -> Vec<graphics::spritebatch::SpriteBatch> {
        let mut image_vec: Vec<graphics::spritebatch::SpriteBatch> = Vec::new();
    
        let mut rainbow_missiles_dir: Vec<path::PathBuf> = filesystem::read_dir(ctx, "/rainbow-missiles").unwrap().collect();

        rainbow_missiles_dir.sort();
    
        for missile_sprite in rainbow_missiles_dir {
            let image = graphics::Image::new(ctx, missile_sprite).unwrap();
            let batch = graphics::spritebatch::SpriteBatch::new(image);
            image_vec.push(batch);
        }
        
        image_vec
    }
}

impl EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let pressed_keys = input::keyboard::pressed_keys(ctx);

        self.player.handle_input(pressed_keys);
        
        if pressed_keys.contains(&KeyCode::Z) {
            let x = self.player.position[0];
            let y = self.player.position[1];

            let right_position = Fec2::new(x + player::PLAYER_WIDTH + MISSILE_GAPS[0], y);
            let left_position = Fec2::new(x - MISSILE_GAPS[0] - MISSILE_WIDTH, y);

            let new_right_missile = straight_missile::Missile::new(right_position, -1.0, -0.5, 0.0);
            let new_left_missile = straight_missile::Missile::new(left_position, -1.0, -0.5, 0.0);

            self.missiles.push(new_right_missile);
            self.missiles.push(new_left_missile);
        }

        while timer::check_update_time(ctx, DESIRED_FPS) {
            for m in self.missiles.iter_mut() {
                m.set_new_position();
            }

            self.missiles.retain(|x| x.position[1] > -20.0);
            self.iteration += 1;

            // println!("{:0}", self.missiles.len());
            // println!("{:0}", ggez::timer::fps(ctx));
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let iteration_mod = (self.iteration % 11) as usize;

        graphics::clear(ctx, graphics::WHITE);

        for m in &self.missiles {
            let p = graphics::DrawParam::new()
                .dest(nalgebra::Point2::new(m.position[0], m.position[1]));

            self.spritebatches[iteration_mod].add(p);
        }

        self.player.draw(ctx)?;

        let param = graphics::DrawParam::new();

        graphics::draw(ctx, &self.spritebatches[iteration_mod], param)?;
        self.spritebatches[iteration_mod].clear();

        graphics::present(ctx)?;
        
        Ok(())
    }
}
