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
