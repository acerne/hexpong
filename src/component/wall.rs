use crate::component::ball;
use crate::gamemode;
use crate::geometry::base::{Angle, Point, Size, Vector};
use crate::geometry::collision;
use crate::geometry::converter;
use crate::geometry::shape::{shape::Shape, Rectangle};
use crate::settings;
use crate::themes;
use crate::{AudibleComponent, VisualComponent};
use ggez::audio::*;
use ggez::*;

pub struct Wall {
    pub shape: Rectangle,
    pub side: gamemode::Side,
    mesh: Option<graphics::Mesh>,
}

impl Wall {
    pub fn new(side: &gamemode::Side) -> Self {
        let phi = side.to_ang() + 30.0;
        let phi_rad = phi.to_radians();
        Wall {
            shape: Rectangle::new(
                Point::new(
                    settings::UNIT_SIZE * phi_rad.cos() * 3.0f32.sqrt() / 2.0,
                    settings::UNIT_SIZE * phi_rad.sin() * 3.0f32.sqrt() / 2.0,
                ),
                Size::new(settings::UNIT_SIZE, settings::norm_to_unit(0.01)),
                Angle::new(phi as f64 + 90f64),
            ),
            side: side.clone(),
            mesh: None,
        }
    }
}

impl VisualComponent for Wall {
    fn collision(&self, ball: &ball::Ball) -> Option<Vector> {
        if collision::are_close(&self.shape, &ball.shape, 10.0) {
            let (dist, _, _) = collision::distance_closest_points(&self.shape, &ball.shape);
            if dist < 5.0 {
                return collision::ball_bounce(&ball.shape, ball.velocity, &self.shape);
            }
        }
        None
    }
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if self.mesh == None {
            self.mesh = self.create_mesh(ctx);
        }
        Ok(())
    }
    fn draw(&self, ctx: &mut Context, theme: &themes::Theme) -> GameResult {
        if let Some(polygon) = &self.mesh {
            graphics::draw(
                ctx,
                polygon,
                ggez::graphics::DrawParam::from((
                    mint::Point2 {
                        x: settings::ORIGIN.0 + settings::unit_to_pixel(self.shape.center.x),
                        y: settings::ORIGIN.1 + settings::unit_to_pixel(self.shape.center.y),
                    },
                    0.0,
                    mint::Point2 { x: 0.0, y: 0.0 },
                    settings::get_scale_vector(),
                    theme.wall,
                )),
            )?;
        }
        Ok(())
    }
    fn create_mesh(&mut self, ctx: &mut Context) -> Option<graphics::Mesh> {
        let mut shape = self.shape.clone();
        shape.center = Point::zero();
        let polygon = shape.to_polygon();
        let vertices = converter::convert_to_points(&polygon);
        Some(
            graphics::MeshBuilder::new()
                .polygon(graphics::DrawMode::fill(), &vertices, graphics::WHITE)
                .unwrap()
                .polygon(
                    ggez::graphics::DrawMode::stroke(settings::norm_to_unit(0.005)),
                    &vertices,
                    [0.8, 0.8, 0.8, 0.6].into(),
                )
                .unwrap()
                .build(ctx)
                .unwrap(),
        )
    }
}

impl AudibleComponent for Wall {
    fn play_sound(&self, ctx: &mut Context) {
        ggez::audio::Source::new(ctx, "/impactMetal_medium_003.ogg")
            .unwrap()
            .play_detached()
            .unwrap();
    }
}
