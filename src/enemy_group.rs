use crate::enemy;
use crate::straight_missile;

use ggez::{graphics, Context, GameResult, nalgebra::Point2};
use nalgebra::Vector2;

type Fec2 = Vector2<f32>;

pub struct EnemyGroup {
    pub enemy_list: Vec<enemy::Enemy>,
    enemy_spritebatch: graphics::spritebatch::SpriteBatch,
}

impl EnemyGroup {
    pub fn new(enemy_spritebatch: graphics::spritebatch::SpriteBatch) -> Self {
        let enemy_list: Vec<enemy::Enemy> = Vec::new();
        let missile_list: Vec<straight_missile::Missile> = Vec::new();

        EnemyGroup {
            enemy_list: enemy_list,
            enemy_spritebatch: enemy_spritebatch,
        }
    }

    pub fn add_enemy(&mut self, enemy: enemy::Enemy) {
        self.enemy_list.push(enemy);
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.draw_enemy_group(ctx)?;
        Ok(())
    }

    fn draw_enemy_group(&mut self, ctx: &mut Context) -> GameResult<()> {
        let param = graphics::DrawParam::new();

        for enemy in &self.enemy_list {
            let p = graphics::DrawParam::new()
                .dest(Point2::new(enemy.position[0], enemy.position[1]));
            
            self.enemy_spritebatch.add(p);            
        }

        graphics::draw(ctx, &self.enemy_spritebatch, param)?;

        self.enemy_spritebatch.clear();
        
        Ok(())
    }
}
