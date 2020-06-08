use crate::straight_missile;

use ggez::{graphics, Context, GameResult};
use ggez::nalgebra;
use ggez::nalgebra::Vector2;

type Fec2 = Vector2<f32>;

pub struct EnemyGroup {
    pub position: Fec2,
    enemy_spritebatch: graphics::spritebatch::SpriteBatch,
    enemy_missile_spritebatch: graphics::spritebatch::SpriteBatch,
    missile_list: Vec<straight_missile::Missile>,
}

impl EnemyGroup {
    pub fn new(starting_position: Fec2, enemy_spritebatch: graphics::spritebatch::SpriteBatch, enemy_missile_spritebatch: graphics::spritebatch::SpriteBatch) -> Self {
        let missile_list: Vec<straight_missile::Missile> = Vec::new();

        EnemyGroup {
            position: starting_position,
            enemy_spritebatch: enemy_spritebatch,
            enemy_missile_spritebatch: enemy_missile_spritebatch,
            missile_list: missile_list,
        }
    }

    pub fn update_missiles(&mut self) { 
        for m in self.missile_list.iter_mut() {
            m.set_new_position();
        }

        self.missile_list.retain(|missile| missile.position[1] > 620.0);
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.draw_enemy_group(ctx)?;
        self.draw_enemy_group_missiles(ctx)?;
        Ok(())
    }

    fn draw_enemy_group(&mut self, ctx: &mut Context) -> GameResult<()> {
        let param = graphics::DrawParam::new();

        let p = graphics::DrawParam::new()
            .dest(nalgebra::Point2::new(self.position[0], self.position[1]));
            
        self.enemy_spritebatch.add(p);

        graphics::draw(ctx, &self.enemy_spritebatch, param)?;

        self.enemy_spritebatch.clear();
        
        Ok(())
    }

    fn draw_enemy_group_missiles(&mut self, ctx: &mut Context) -> GameResult<()> {
        let param = graphics::DrawParam::new();

        for m in &self.missile_list {
            let p = graphics::DrawParam::new()
                .dest(nalgebra::Point2::new(m.position[0], m.position[1]));
            
            self.enemy_spritebatch.add(p);
        }

        graphics::draw(ctx, &self.enemy_spritebatch, param)?;

        self.enemy_spritebatch.clear();
        
        Ok(())
    }
}
