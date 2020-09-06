extern crate rand;
extern crate ncollide2d;

use rand::Rng;
use std::path;
use ggez::event::KeyCode;
use ggez::{input, graphics, Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use ggez::timer;
use nalgebra::Vector2;
use ggez::filesystem;

mod enemies;
mod missiles;
mod player;

type Fec2 = Vector2<f32>;

const DESIRED_FPS: u32 = 60;
const WINDOW_WIDTH: f32 = 1920.0;
const WINDOW_HEIGHT: f32 = 1080.0;

fn main() {
    let resource_dir = path::PathBuf::from("./resources");

    let (mut ctx, mut event_loop) = ContextBuilder::new("cuban-missie-crisis-test", "Andy")
        .window_setup(ggez::conf::WindowSetup::default().title("missile-crisis"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
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

struct State {
    player: player::Player,
    enemy_group: enemies::enemy_group::EnemyGroup,
}

impl State {
    pub fn new(_ctx: &mut Context) -> State {
        let image_vec = load_missile_sprites(_ctx);
        let enemy_sprite = load_enemy_sprite(_ctx);

        let initial_position = Fec2::new(WINDOW_WIDTH/2.0, WINDOW_HEIGHT - 50.0);

        let state = State {
            player: player::Player::new(initial_position, image_vec),
            enemy_group: enemies::enemy_group::EnemyGroup::new(enemy_sprite),
        };

        state
    }
}

impl EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while timer::check_update_time(ctx, DESIRED_FPS) {        
            let pressed_keys = input::keyboard::pressed_keys(ctx);

            if pressed_keys.contains(&KeyCode::Q) {
                let mut rng = rand::thread_rng();

                let rng_enemy: enemies::enemy::Enemy = enemies::enemy::Enemy::new(
                    Fec2::new(rng.gen_range(0.0, 1920.0), rng.gen_range(0.0, 1080.0))
                );
                
                self.enemy_group.add_enemy(rng_enemy);
            }

            self.player.handle_input(pressed_keys);    
            self.player.update(&mut self.enemy_group);
            self.enemy_group.update();

            println!("{:0}", ggez::timer::fps(ctx));
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        
        self.enemy_group.draw(ctx)?;
        self.player.draw(ctx)?;

        graphics::present(ctx)?;
        
        Ok(())
    }
}
