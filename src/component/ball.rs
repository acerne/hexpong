use crate::settings;
use crate::themes;
use crate::VisualComponent;
use ggez::*;
use rand::Rng;

pub struct Ball {
    pub x: f32,
    pub y: f32,
    pub r: f32,
    pub velocity: f32,
    pub direction: f32,
    mesh: Option<graphics::Mesh>,
}

impl Ball {
    pub fn new(ball_speed: f32) -> Self {
        let mut rng = rand::thread_rng();
        let var = (rng.gen::<f32>() - 0.5) * 20.0;

        Ball {
            x: settings::BALL_SPAWN.0,
            y: settings::BALL_SPAWN.1,
            velocity: settings::norm_to_unit(ball_speed),
            direction: (270.0 + var).to_radians(),
            r: settings::norm_to_unit(0.01),
            mesh: None,
        }
    }
    pub fn bounce_away(&mut self, norm_vec: &nalgebra::Vector2<f32>) {
        let veloc_vec = nalgebra::Vector2::new(
            self.velocity * self.direction.cos(),
            self.velocity * self.direction.sin(),
        );
        let bouce_vec = veloc_vec - 2.0 * veloc_vec.dot(&norm_vec) * norm_vec;
        self.velocity = (bouce_vec.x.powf(2.0) + bouce_vec.y.powf(2.0)).sqrt();
        self.direction = bouce_vec.y.atan2(bouce_vec.x);
    }
    fn get_position(&self) -> mint::Point2<f32> {
        mint::Point2 {
            x: self.x,
            y: self.y,
        }
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
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if self.mesh == None {
            self.mesh = self.create_mesh(ctx);
        }
        self.x += self.velocity * self.direction.cos();
        self.y += self.velocity * self.direction.sin();
        Ok(())
    }
    fn draw(&self, ctx: &mut Context, theme: &themes::Theme) -> GameResult {
        if let Some(circle) = &self.mesh {
            graphics::draw(
                ctx,
                circle,
                ggez::graphics::DrawParam::from((
                    mint::Point2 {
                        x: settings::ORIGIN.0 + settings::unit_to_pixel(self.x),
                        y: settings::ORIGIN.1 + settings::unit_to_pixel(self.y),
                    },
                    0.0,
                    mint::Point2 { x: 0.0, y: 0.0 },
                    settings::get_scale_vector(),
                    graphics::WHITE,
                )),
            )?;
            let text = ggez::graphics::Text::new(format!(
                "vel: {}\ndir: {}",
                self.velocity,
                self.direction.to_degrees()
            ));
            graphics::draw(
                ctx,
                &text,
                ggez::graphics::DrawParam::from((
                    mint::Point2 {
                        x: settings::ORIGIN.0 + settings::unit_to_pixel(self.x + 10.0),
                        y: settings::ORIGIN.1 + settings::unit_to_pixel(self.y),
                    },
                    0.0,
                    mint::Point2 { x: 0.0, y: 0.0 },
                    mint::Vector2 { x: 1.0, y: 1.0 },
                    [1.0, 0.0, 0.0, 1.0].into(),
                )),
            )?;
        }
        Ok(())
    }
    fn create_mesh(&mut self, ctx: &mut Context) -> Option<graphics::Mesh> {
        Some(
            graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                mint::Point2 { x: 0.0, y: 0.0 },
                self.r,
                0.1,
                graphics::WHITE,
            )
            .unwrap(),
        )
    }
}
