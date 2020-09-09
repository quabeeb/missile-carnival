use crate::enemies::enemy;
use crate::missiles::straight_missile;

use ggez::{graphics, Context, GameResult, nalgebra::Point2};
use nalgebra::Vector2;

type Fec2 = Vector2<f32>;

pub struct EnemyGroup {
    pub enemy_list: Vec<Box<dyn enemy::Enemy>>,
    enemy_spritebatch: graphics::spritebatch::SpriteBatch,
}

impl EnemyGroup {
    pub fn new(enemy_spritebatch: graphics::spritebatch::SpriteBatch) -> Self {
        let enemy_list: Vec<Box<dyn enemy::Enemy>> = Vec::new();
        let missile_list: Vec<straight_missile::StraightMissile> = Vec::new();

        EnemyGroup {
            enemy_list: enemy_list,
            enemy_spritebatch: enemy_spritebatch,
        }
    }

    pub fn add_enemy(&mut self, enemy: Box<dyn enemy::Enemy>) {
        self.enemy_list.push(enemy);
    }

    pub fn get_enemy(&mut self, index: usize) -> Option<&Box<dyn enemy::Enemy>> {
        self.enemy_list.get(index)
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.draw_enemy_group(ctx)?;
        Ok(())
    }

    pub fn update(&mut self) {
        self.enemy_list.retain(|enemy| 
            enemy.get_status() == false
        );  
    }

    pub fn update_enemy(&mut self, index: usize, damage: i32) {
        let enemy = self.enemy_list.get_mut(index);

        match enemy {
            Some(x) => {
                x.decrement_health(damage);
            },
            None => {
                println!("enemy not found?")
            }
        }        
    }

    fn draw_enemy_group(&mut self, ctx: &mut Context) -> GameResult<()> {
        let param = graphics::DrawParam::new();

        for enemy in &self.enemy_list {
            let enemy_position: Fec2 = enemy.get_position();

            let p = graphics::DrawParam::new()
                .dest(Point2::new(enemy_position[0], enemy_position[1]));
            
            self.enemy_spritebatch.add(p);            
        }

        graphics::draw(ctx, &self.enemy_spritebatch, param)?;

        self.enemy_spritebatch.clear();
        
        Ok(())
    }
}
