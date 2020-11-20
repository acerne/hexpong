use crate::settings;
use crate::VisualComponent;
use ggez::*;
use rand::Rng;

pub struct Ball {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub r: f32,
}

impl Ball {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Ball {
            x: settings::BALL_SPAWN.0,
            y: settings::BALL_SPAWN.1,
            vx: rng.gen::<f32>() * 3.0 - 1.5,
            vy: -5.0,
            r: 3.0,
        }
    }
    pub fn bounce_away(&mut self, norm_vec: &nalgebra::Vector2<f32>) {
        let veloc_vec = nalgebra::Vector2::new(self.vx, self.vy);
        let bouce_vec = veloc_vec - 2.0 * veloc_vec.dot(&norm_vec) * norm_vec;
        self.vx = bouce_vec.x;
        self.vy = bouce_vec.y;
    }
}

impl VisualComponent for Ball {
    fn collision(&self, ball: &Ball) -> Option<nalgebra::Vector2<f32>> {
        let dist = ((self.x - ball.x).powf(2.0) + (self.y - ball.y).powf(2.0)).sqrt();
        if dist < self.r + ball.r {
            // hit - bounce off and destroy block
            return Some(nalgebra::Vector2::new(
                (self.x - ball.x) / dist,
                (self.y - ball.y) / dist,
            ));
        }
        None
    }
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.x += self.vx;
        self.y += self.vy;
        Ok(())
    }
    fn draw(&self, ctx: &mut Context) -> GameResult {
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            mint::Point2 {
                x: self.x,
                y: self.y,
            },
            self.r,
            0.1,
            graphics::WHITE,
        )?;
        graphics::draw(ctx, &circle, graphics::DrawParam::default())?;
        Ok(())
    }
}
