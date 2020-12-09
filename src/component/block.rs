use crate::component::ball;
use crate::geometry::base::{Angle, Point, Vector};
use crate::geometry::collision;
use crate::geometry::converter;
use crate::geometry::shape::{shape::Shape, Hexagon};
use crate::settings;
use crate::themes;
use crate::{AudibleComponent, VisualComponent};
use ggez::audio::*;
use ggez::*;

#[derive(PartialEq, Eq, Hash)]
pub enum BlockType {
    Basic,
    Basic2,
    Basic3,
    Immortal,
}

impl BlockType {
    pub fn from_str(input: &str) -> BlockType {
        match &input.to_lowercase()[..] {
            "basic" => BlockType::Basic,
            "basic2" => BlockType::Basic2,
            "basic3" => BlockType::Basic3,
            "immortal" => BlockType::Immortal,
            _ => panic!("Invalid block shape"),
        }
    }
}

pub struct Block {
    pub shape: Hexagon,
    pub block_type: BlockType,
    mesh: Option<graphics::Mesh>,
}

impl Block {
    pub fn new(x: f32, y: f32, r: f32, block_type: BlockType) -> Self {
        Block {
            shape: Hexagon::new(Point::new(x, y), r, Angle::new(90f64)),
            block_type: block_type,
            mesh: None,
        }
    }
    pub fn hit(&mut self) -> bool {
        match self.block_type {
            BlockType::Basic => true,
            BlockType::Basic2 => {
                self.block_type = BlockType::Basic;
                false
            }
            BlockType::Basic3 => {
                self.block_type = BlockType::Basic2;
                false
            }
            BlockType::Immortal => false,
            _ => true,
        }
    }
}

impl VisualComponent for Block {
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
        let location = self.shape.center;
        if let Some(polygon) = &self.mesh {
            graphics::draw(
                ctx,
                polygon,
                ggez::graphics::DrawParam::from((
                    mint::Point2 {
                        x: settings::ORIGIN.0 + settings::unit_to_pixel(location.x),
                        y: settings::ORIGIN.1 + settings::unit_to_pixel(location.y),
                    },
                    0.0,
                    mint::Point2 { x: 0.0, y: 0.0 },
                    settings::get_scale_vector(),
                    theme.get_block_color(&self.block_type),
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

impl AudibleComponent for Block {
    fn play_sound(&self, ctx: &mut Context) {
        ggez::audio::Source::new(ctx, "/impactGlass_medium_000.ogg")
            .unwrap()
            .play_detached()
            .unwrap();
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct GridIndex {
    pub q: i32,
    pub r: i32,
}

impl GridIndex {
    pub fn to_pixel(&self, tile_radius: f32) -> mint::Point2<f32> {
        let x = settings::ORIGIN.0
            + (self.q as f32 * 3.0f32.sqrt() + self.r as f32 * (3.0f32.sqrt() / 2.0)) * tile_radius;
        let y = settings::ORIGIN.1 + (3.0 / 2.0 * self.r as f32) * tile_radius;
        mint::Point2 { x: x, y: y }
    }
    pub fn to_unit(&self, tile_radius: f32) -> mint::Point2<f32> {
        let x =
            (self.q as f32 * 3.0f32.sqrt() + self.r as f32 * (3.0f32.sqrt() / 2.0)) * tile_radius;
        let y = (3.0 / 2.0 * self.r as f32) * tile_radius;
        mint::Point2 { x: x, y: y }
    }
}
