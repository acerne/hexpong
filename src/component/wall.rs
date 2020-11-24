use crate::component::ball;
use crate::gamemode;
use crate::settings;
use crate::themes;
use crate::VisualComponent;
use ggez::*;

pub struct Wall {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
    pub thickness: f32,
    pub side: gamemode::Side,
}

impl Wall {
    pub fn new(side: &gamemode::Side) -> Self {
        let phi = side.to_ang();
        let phi_rad = phi.to_radians();
        let theta_rad = (phi + 60.0).to_radians();
        Wall {
            x1: settings::ORIGIN.0 + settings::HEXAGON_SIZE * phi_rad.cos(),
            y1: settings::ORIGIN.1 + settings::HEXAGON_SIZE * phi_rad.sin(),
            x2: settings::ORIGIN.0 + settings::HEXAGON_SIZE * theta_rad.cos(),
            y2: settings::ORIGIN.1 + settings::HEXAGON_SIZE * theta_rad.sin(),
            thickness: 5.0, // TODO: relative
            side: side.clone(),
        }
    }
}

impl VisualComponent for Wall {
    fn collision(&self, ball: &ball::Ball) -> Option<nalgebra::Vector2<f32>> {
        let d1 = ((ball.x - self.x1).powf(2.0) + (ball.y - self.y1).powf(2.0)).sqrt();
        let d2 = ((ball.x - self.x2).powf(2.0) + (ball.y - self.y2).powf(2.0)).sqrt();

        if (d1 + d2) - settings::HEXAGON_SIZE < 1.0 {
            // hit - bounce off
            let phi = (self.side.to_ang() - 150.0).to_radians();
            return Some(nalgebra::Vector2::new(phi.cos(), phi.sin()));
        }
        None
    }
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&self, ctx: &mut Context, theme: &themes::Theme) -> GameResult {
        let line = graphics::Mesh::new_line(
            ctx,
            &[
                mint::Point2 {
                    x: self.x1,
                    y: self.y1,
                },
                mint::Point2 {
                    x: self.x2,
                    y: self.y2,
                },
            ],
            self.thickness,
            theme.wall,
        )?;
        graphics::draw(
            ctx,
            &line,
            ggez::graphics::DrawParam::from((ggez::mint::Point2 { x: 0.0, y: 0.0 },)),
        )?;
        Ok(())
    }
}
