use crate::component::ball;
use crate::settings;
use crate::InputState;
use crate::VisualComponent;
use ggez::*;

pub struct Player {
    pub barpos: f32,
    pub bars: Vec<Bar>,
    pub input: InputState,
}

impl Player {
    pub fn update(&mut self, ctx: &mut Context) -> GameResult {
        if self.input.left {
            self.barpos -= 0.03; // TODO as parameter
        } else if self.input.right {
            self.barpos += 0.03;
        }
        if self.barpos < (0.0 + settings::BAR_SIZE.0 / settings::HEXAGON_SIZE / 2.0) {
            self.barpos = 0.0 + settings::BAR_SIZE.0 / settings::HEXAGON_SIZE / 2.0;
        }
        if self.barpos > (1.0 - settings::BAR_SIZE.0 / settings::HEXAGON_SIZE / 2.0) {
            self.barpos = 1.0 - settings::BAR_SIZE.0 / settings::HEXAGON_SIZE / 2.0;
        }
        for bar in self.bars.iter_mut() {
            bar.pos = self.barpos;
            bar.update(ctx)?;
        }
        Ok(())
    }
    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        for bar in self.bars.iter() {
            bar.draw(ctx)?;
        }
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Bar {
    pub xc: f32,
    pub yc: f32,
    pub pos: f32,
    pub l1: f32, // w/2
    pub l2: f32, // h/2
    pub phi: f32,
    pub color: graphics::Color,
}

impl Bar {
    fn get_vertices(&self) -> [mint::Point2<f32>; 4] {
        let mut vertices: [mint::Point2<f32>; 4] = [mint::Point2 { x: 0.0, y: 0.0 }; 4];

        let phi = self.phi.to_radians();

        let long_cos = self.l1 * phi.cos();
        let long_sin = self.l1 * phi.sin();
        let short_cos = self.l2 * phi.cos();
        let short_sin = self.l2 * phi.sin();

        vertices[0].x = self.xc + long_cos + short_sin;
        vertices[0].y = self.yc - long_sin + short_cos;
        vertices[1].x = self.xc + long_cos - short_sin;
        vertices[1].y = self.yc - long_sin - short_cos;
        vertices[2].x = self.xc - long_cos - short_sin;
        vertices[2].y = self.yc + long_sin - short_cos;
        vertices[3].x = self.xc - long_cos + short_sin;
        vertices[3].y = self.yc + long_sin + short_cos;

        vertices
    }
}

impl VisualComponent for Bar {
    fn collision(&self, ball: &ball::Ball) -> Option<nalgebra::Vector2<f32>> {
        let phi = (-self.phi).to_radians();
        let tx = (ball.x - self.xc) * phi.cos() + (ball.y - self.yc) * phi.sin();
        let ty = -(ball.x - self.xc) * phi.sin() + (ball.y - self.yc) * phi.cos();
        if tx > -self.l1 && tx < self.l1 && ty > -self.l2 && ty < self.l2 {
            // hit - bounce off
            return Some(nalgebra::Vector2::new(
                (phi + std::f32::consts::PI / 2.0).cos(),
                (phi + std::f32::consts::PI / 2.0).sin(),
            ));
        }
        None
    }
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        let xc0 = settings::ORIGIN.0
            + if (self.phi % 120.0) == 0.0 {
                -settings::HEXAGON_SIZE / 2.0 + self.pos * settings::HEXAGON_SIZE
            } else {
                settings::HEXAGON_SIZE / 2.0 - self.pos * settings::HEXAGON_SIZE
            };
        let yc0 = settings::ORIGIN.1 + 3.0f32.sqrt() / 2.0 * settings::HEXAGON_SIZE;

        let phi = self.phi.to_radians();

        self.xc = (xc0 - settings::ORIGIN.0) * phi.cos()
            + (yc0 - settings::ORIGIN.1) * phi.sin()
            + settings::ORIGIN.0;
        self.yc = -(xc0 - settings::ORIGIN.0) * phi.sin()
            + (yc0 - settings::ORIGIN.1) * phi.cos()
            + settings::ORIGIN.1;

        Ok(())
    }
    fn draw(&self, ctx: &mut Context) -> GameResult {
        let vertices = self.get_vertices();
        let polygon =
            graphics::Mesh::new_polygon(ctx, graphics::DrawMode::fill(), &vertices, self.color)?;
        graphics::draw(
            ctx,
            &polygon,
            ggez::graphics::DrawParam::from((ggez::mint::Point2 { x: 0.0, y: 0.0 },)),
        )?;
        Ok(())
    }
}
