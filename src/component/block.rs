use crate::component::ball;
use crate::settings;
use crate::themes;
use crate::VisualComponent;
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

pub struct Hexagon {
    pub x: f32,
    pub y: f32,
    pub r: f32,
    pub phi: f32,
    pub block_type: BlockType,
}

impl Hexagon {
    pub fn get_vertices(&self) -> [mint::Point2<f32>; 6] {
        let mut vertices: [mint::Point2<f32>; 6] = [mint::Point2 { x: 0.0, y: 0.0 }; 6];
        for i in 0..6 {
            let angle = (self.phi + 30.0 + i as f32 * 60.0).to_radians();
            let xh = angle.cos() * self.r + self.x;
            let yh = angle.sin() * self.r + self.y;
            vertices[i] = mint::Point2 { x: xh, y: yh };
        }
        vertices
    }
    pub fn draw_trace(&self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let vertices = self.get_vertices();
        let trace = ggez::graphics::Mesh::new_polygon(
            ctx,
            ggez::graphics::DrawMode::stroke(3.0),
            &vertices,
            [0.8, 0.8, 0.8, 0.6].into(),
        )?;
        ggez::graphics::draw(
            ctx,
            &trace,
            ggez::graphics::DrawParam::from((ggez::mint::Point2 { x: 0.0, y: 0.0 },)),
        )?;
        Ok(())
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

impl VisualComponent for Hexagon {
    fn collision(&self, ball: &ball::Ball) -> Option<nalgebra::Vector2<f32>> {
        let dist = ((self.x - ball.x).powf(2.0) + (self.y - ball.y).powf(2.0)).sqrt();
        if dist < self.r + ball.r {
            return Some(nalgebra::Vector2::new(
                (self.x - ball.x) / dist,
                (self.y - ball.y) / dist,
            ));
        }
        None
    }
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&self, ctx: &mut Context, theme: &themes::Theme) -> GameResult {
        let vertices = self.get_vertices();
        let polygon = graphics::Mesh::new_polygon(
            ctx,
            graphics::DrawMode::fill(),
            &vertices,
            theme.get_block_color(&self.block_type),
        )?;
        graphics::draw(
            ctx,
            &polygon,
            ggez::graphics::DrawParam::from((ggez::mint::Point2 { x: 0.0, y: 0.0 },)),
        )?;
        Ok(())
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
}
