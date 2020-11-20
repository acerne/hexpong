use crate::pawn;
use crate::settings;
use crate::VisualComponent;
use ggez::*;

pub enum BlockType {
    OneTouch,
    TwoTouch,
    ThreeTouch,
    Immortal,
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
    pub fn get_color(&self) -> [f32; 4] {
        match &self.block_type {
            BlockType::OneTouch => [0.5, 1.0, 0.5, 1.0],
            BlockType::TwoTouch => [1.0, 0.5, 0.5, 1.0],
            BlockType::ThreeTouch => [0.5, 0.5, 1.0, 1.0],
            BlockType::Immortal => [0.5, 0.5, 0.5, 1.0],
            _ => [1.0, 0.0, 0.0, 1.0],
        }
    }
    pub fn hit(&mut self) -> bool {
        match self.block_type {
            BlockType::OneTouch => true,
            BlockType::TwoTouch => {
                self.block_type = BlockType::OneTouch;
                false
            }
            BlockType::ThreeTouch => {
                self.block_type = BlockType::TwoTouch;
                false
            }
            BlockType::Immortal => false,
            _ => true,
        }
    }
}

impl VisualComponent for Hexagon {
    fn collision(&self, ball: &pawn::Ball) -> Option<nalgebra::Vector2<f32>> {
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
        Ok(())
    }
    fn draw(&self, ctx: &mut Context) -> GameResult {
        let vertices = self.get_vertices();
        let polygon = graphics::Mesh::new_polygon(
            ctx,
            graphics::DrawMode::fill(),
            &vertices,
            self.get_color().into(),
        )?;
        graphics::draw(
            ctx,
            &polygon,
            ggez::graphics::DrawParam::from((ggez::mint::Point2 { x: 0.0, y: 0.0 },)),
        )?;
        Ok(())
    }
}

struct GridIndex {
    q: i32,
    r: i32,
}

impl GridIndex {
    fn to_pixel(&self, tile_radius: f32) -> mint::Point2<f32> {
        let x = settings::ORIGIN.0
            + (self.q as f32 * 3.0f32.sqrt() + self.r as f32 * (3.0f32.sqrt() / 2.0)) * tile_radius;
        let y = settings::ORIGIN.1 + (3.0 / 2.0 * self.r as f32) * tile_radius;
        mint::Point2 { x: x, y: y }
    }
}

pub struct HexagonalGrid {
    pub tiles: Vec<Hexagon>,
}

impl HexagonalGrid {
    pub fn new(grid_size: u16, tile_radius: f32) -> Self {
        let grid_radius = ((grid_size + 1) / 2) as i32;
        let mut tiles = Vec::new();
        for q in (-grid_radius + 1)..grid_radius {
            for r in std::cmp::max(-grid_radius + 1, -q - grid_radius + 1)
                ..=std::cmp::min(grid_radius - 1, -q + grid_radius - 1)
            {
                let index = GridIndex { q: q, r: r };
                let point = index.to_pixel(tile_radius);
                tiles.push(Hexagon {
                    x: point.x,
                    y: point.y,
                    r: tile_radius,
                    phi: 0.0,
                    block_type: if q.abs() > 2 || r.abs() > 2 {
                        BlockType::OneTouch
                    } else if q.abs() == 2 || r.abs() == 2 {
                        BlockType::TwoTouch
                    } else if q.abs() == 1 || r.abs() == 1 {
                        BlockType::ThreeTouch
                    } else {
                        BlockType::Immortal
                    },
                });
            }
        }
        HexagonalGrid { tiles: tiles }
    }
    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        for hexagon in self.tiles.iter() {
            hexagon.draw(ctx)?;
        }
        for hexagon in self.tiles.iter() {
            hexagon.draw_trace(ctx)?;
        }
        Ok(())
    }
}
