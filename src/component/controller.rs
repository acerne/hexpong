use crate::component::ball;
use crate::gamemode;
use crate::settings;
use crate::themes;
use crate::InputState;
use crate::{AudibleComponent, VisualComponent};
use geometry::base::*;
use geometry::collision;
use geometry::shape::*;
use ggez::audio::*;
use ggez::*;

pub enum Player {
    Player1,
    Player2,
    Player3,
}

pub struct Controller {
    pub barpos: f32,
    pub bars: Vec<Bar>,
    pub bar_size: f32,
    pub input: InputState,
    pub player: Player,
}

impl Controller {
    pub fn new(bar_size: f32, player: Player) -> Self {
        Controller {
            barpos: 0.5,
            bar_size: bar_size,
            bars: Vec::new(),
            input: InputState::default(),
            player: player,
        }
    }
    pub fn update(&mut self, ctx: &mut Context) -> GameResult {
        if self.input.left {
            self.barpos -= 0.03; // TODO as parameter
        } else if self.input.right {
            self.barpos += 0.03;
        }
        if self.barpos < (0.0 + self.bar_size / 2.0) {
            self.barpos = 0.0 + self.bar_size / 2.0;
        }
        if self.barpos > (1.0 - self.bar_size / 2.0) {
            self.barpos = 1.0 - self.bar_size / 2.0;
        }
        for bar in self.bars.iter_mut() {
            bar.pos = self.barpos;
            bar.update(ctx)?;
        }
        Ok(())
    }
    pub fn draw(&self, ctx: &mut Context, theme: &themes::Theme) -> GameResult {
        for bar in self.bars.iter() {
            bar.draw(ctx, theme)?;
        }
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Bar {
    pub shape: Rectangle,
    pub side: gamemode::Side,
    pos: f32,
    mesh: Option<graphics::Mesh>,
    owner: Player,
    reversed: bool,
}

impl Bar {
    pub fn new(side: &gamemode::Side, bar_size: f32, owner: Player, reversed: bool) -> Self {
        Bar {
            shape: Rectangle::new(
                Point::zero(),
                Size::new(
                    settings::norm_to_unit(bar_size),
                    settings::norm_to_unit(0.02),
                ),
                Angle::new(side.to_ang() as f64 - 60f64),
            ),
            pos: 0.5,
            side: side.clone(),
            mesh: None,
            owner: owner,
            reversed: reversed,
        }
    }
    fn get_vertices(&self) -> [mint::Point2<f32>; 4] {
        let mut vertices: [mint::Point2<f32>; 4] = [mint::Point2 { x: 0.0, y: 0.0 }; 4];
        let phi = (self.side.to_ang() - 60.0).to_radians();

        let long_cos = self.shape.size.w / 2.0 * phi.cos();
        let long_sin = self.shape.size.w / 2.0 * phi.sin();
        let short_cos = self.shape.size.h / 2.0 * phi.cos();
        let short_sin = self.shape.size.h / 2.0 * phi.sin();

        vertices[0].x = long_cos + short_sin;
        vertices[0].y = -long_sin + short_cos;
        vertices[1].x = long_cos - short_sin;
        vertices[1].y = -long_sin - short_cos;
        vertices[2].x = -long_cos - short_sin;
        vertices[2].y = long_sin - short_cos;
        vertices[3].x = -long_cos + short_sin;
        vertices[3].y = long_sin + short_cos;

        vertices
    }
}

impl VisualComponent for Bar {
    fn collision(&self, ball: &ball::Ball) -> Option<Vector> {
        if collision::detection::are_close(&self.shape, &ball.shape, 10.0) {
            let (dist, _, _) =
                collision::detection::distance_closest_points(&self.shape, &ball.shape);
            if dist < 5.0 {
                return collision::detection::ball_bounce(&ball.shape, ball.velocity, &self.shape);
            }
        }
        None
    }
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if self.mesh == None {
            self.mesh = self.create_mesh(ctx);
        }

        let mut xc0 = -settings::UNIT_SIZE / 2.0 + self.pos * settings::UNIT_SIZE;
        let yc0 = 3.0f32.sqrt() / 2.0 * settings::UNIT_SIZE;

        if self.reversed {
            xc0 = -xc0;
        }

        let phi = self.shape.phi.to_rad32();

        self.shape.center = Point::new(
            xc0 * phi.cos() + yc0 * phi.sin(),
            -xc0 * phi.sin() + yc0 * phi.cos(),
        );
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
                    match self.owner {
                        Player::Player1 => theme.player1,
                        Player::Player2 => theme.player2,
                        Player::Player3 => theme.player3,
                    },
                )),
            )?;
        }
        Ok(())
    }
    fn create_mesh(&mut self, ctx: &mut Context) -> Option<graphics::Mesh> {
        let vertices = self.get_vertices();
        Some(
            graphics::Mesh::new_polygon(
                ctx,
                graphics::DrawMode::fill(),
                &vertices,
                graphics::WHITE,
            )
            .unwrap(),
        )
    }
}

impl AudibleComponent for Bar {
    fn play_sound(&self, ctx: &mut Context) {
        ggez::audio::Source::new(ctx, "/back_003.ogg")
            .unwrap()
            .play_detached()
            .unwrap();
    }
}
