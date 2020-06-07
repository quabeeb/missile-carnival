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
    player: player::Player
}

impl State {
    pub fn new(_ctx: &mut Context) -> State {
        let image_vec = State::load_missile_sprites(_ctx);
        let list: Vec<straight_missile::Missile> = Vec::new();
        let initial_position = Fec2::new(400.0, 600.0);
        
        let state = State {
            player: player::Player::new(initial_position, image_vec)
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

        while timer::check_update_time(ctx, DESIRED_FPS) {            
            self.player.update_missiles();
            // println!("{:0}", self.missiles.len());
            // println!("{:0}", ggez::timer::fps(ctx));
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);

        self.player.draw(ctx)?;

        graphics::present(ctx)?;
        
        Ok(())
    }
}
