extern crate rand;

use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use ggez::timer;
use rand::Rng;


mod missile;
mod position;

fn main() {    
    let (mut ctx, mut event_loop) = ContextBuilder::new("cuban-missie-crisis-test", "Andy")
        .window_setup(ggez::conf::WindowSetup::default().title("O shit you started WW3"))
		.build()
		.expect("Could not create ggez context!");


    let mut missile_carnival = State::new(&mut ctx);

    match event::run(&mut ctx, &mut event_loop, &mut missile_carnival) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}

struct State {
    timesteps: i32,
    missiles: Vec<missile::Missile>
}

impl State {
    pub fn new(_ctx: &mut Context) -> State {
        let list: Vec<missile::Missile> = Vec::new();

        let state = State {
            timesteps: 300,
            missiles: list
        };

        state
    }
}

impl EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {

            let missile_spawning_position = position::Position::new(self.timesteps, 0);
            let new_missile = missile::Missile::new(missile_spawning_position);

            self.missiles.push(new_missile);

            for m in self.missiles.iter_mut() {
                m.get_new_position();
            }

            self.timesteps -= 1;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {

        graphics::clear(ctx, graphics::WHITE);

        let mut rng = rand::thread_rng();
        let color = [rng.gen(), rng.gen(), rng.gen(), 1.0].into();

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
                color
            )?;

            graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
        }
        
        graphics::present(ctx)?;
        Ok(())
    }
}