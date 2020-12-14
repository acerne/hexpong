use crate::converter;
use crate::settings;
use crate::themes;
use crate::VisualComponent;
use geometry::base::{Angle, Point, Vector};
use geometry::collision::*;
use geometry::shape::*;
use ggez::*;
use rand::Rng;

pub struct Ball {
    pub shape: Circle,
    // pub velocity: f32,
    // pub direction: f32,
    pub velocity: Vector,
    mesh: Option<graphics::Mesh>,
}

impl Ball {
    pub fn new(ball_speed: f32) -> Self {
        let mut rng = rand::thread_rng();
        let var = (rng.gen::<f64>() - 0.5) * 20.0;

        Ball {
            shape: Circle::new(
                Point::new(settings::BALL_SPAWN.0, settings::BALL_SPAWN.1),
                settings::norm_to_unit(0.01),
            ),
            velocity: Vector::from_magnitude(
                settings::norm_to_unit(ball_speed),
                Angle::new(270f64 + var),
            ),
            mesh: None,
        }
    }
    pub fn bounce_away(&mut self, norm_vec: Vector) {
        self.velocity = self.velocity - norm_vec * self.velocity.dot(norm_vec) * 2.0;
    }
    fn get_position(&self) -> mint::Point2<f32> {
        converter::convert_to_point(&self.shape.center())
    }
}

impl VisualComponent for Ball {
    fn collision(&self, ball: &Ball) -> Option<Vector> {
        // TODO - Ball-ball collision
        None
    }
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if self.mesh == None {
            self.mesh = self.create_mesh(ctx);
        }
        self.shape.translate(self.velocity);
        Ok(())
    }
    fn draw(&self, ctx: &mut Context, theme: &themes::Theme) -> GameResult {
        if let Some(circle) = &self.mesh {
            graphics::draw(
                ctx,
                circle,
                ggez::graphics::DrawParam::from((
                    mint::Point2 {
                        x: settings::ORIGIN.0 + settings::unit_to_pixel(self.shape.center().x),
                        y: settings::ORIGIN.1 + settings::unit_to_pixel(self.shape.center().y),
                    },
                    0.0,
                    mint::Point2 { x: 0.0, y: 0.0 },
                    settings::get_scale_vector(),
                    graphics::WHITE,
                )),
            )?;
            let text = ggez::graphics::Text::new(format!(
                "vel: {:.2}\ndir: {:.2}°",
                self.velocity.magnitude(),
                self.velocity.orientation().deg
            ));
            graphics::draw(
                ctx,
                &text,
                ggez::graphics::DrawParam::from((
                    mint::Point2 {
                        x: settings::ORIGIN.0
                            + settings::unit_to_pixel(self.shape.center().x + 10.0),
                        y: settings::ORIGIN.1 + settings::unit_to_pixel(self.shape.center().y),
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
                self.shape.radius(),
                0.1,
                graphics::WHITE,
            )
            .unwrap(),
        )
    }
}
