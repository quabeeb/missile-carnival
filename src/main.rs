extern crate rand;

use std::path;
use ggez::{input, graphics, Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use ggez::timer;
use ggez::nalgebra::Vector2;
use ggez::filesystem;

mod enemy_group;
mod straight_missile;
mod player;
mod missile_generator;

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

fn load_missile_sprites(ctx: &mut Context) -> Vec<graphics::spritebatch::SpriteBatch> {
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

fn load_enemy_sprite(ctx: &mut Context) -> graphics::spritebatch::SpriteBatch {
    let image = graphics::Image::new(ctx, "/enemy1.png").unwrap();

    graphics::spritebatch::SpriteBatch::new(image)
}

fn load_enemy_missile_sprite(ctx: &mut Context) -> graphics::spritebatch::SpriteBatch {
    let image = graphics::Image::new(ctx, "/rainbow-missiles/00.png").unwrap();

    graphics::spritebatch::SpriteBatch::new(image)
}

struct State {
    player: player::Player,
    enemy_group: enemy_group::EnemyGroup,
}

impl State {
    pub fn new(_ctx: &mut Context) -> State {
        let image_vec = load_missile_sprites(_ctx);
        let enemy_sprite = load_enemy_sprite(_ctx);
        let enemy_missile_sprite = load_enemy_missile_sprite(_ctx);

        let initial_position = Fec2::new(400.0, 600.0);
        let enemy_initial_position = Fec2::new(400.0, 300.0);
        
        let state = State {
            player: player::Player::new(initial_position, image_vec),
            enemy_group: enemy_group::EnemyGroup::new(enemy_initial_position, enemy_sprite, enemy_missile_sprite),
        };

        state
    }
}

impl EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while timer::check_update_time(ctx, DESIRED_FPS) {        
            let pressed_keys = input::keyboard::pressed_keys(ctx);
            self.player.handle_input(pressed_keys);    
            self.player.update();

            // println!("{:0}", ggez::timer::fps(ctx));
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);

        self.enemy_group.draw(ctx)?;
        self.player.draw(ctx)?;

        graphics::present(ctx)?;
        
        Ok(())
    }
}
