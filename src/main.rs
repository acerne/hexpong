use ggez::*;

use ggez::event::{KeyCode, KeyMods};
use ggez::{event, graphics, Context, GameResult};

use rand::Rng;

mod block;
mod settings;

trait VisualComponent {
    fn collision(&self, ball: &pawn::Ball) -> Option<nalgebra::Vector2<f32>>;
    fn update(&mut self, _ctx: &mut Context) -> GameResult;
    fn draw(&self, ctx: &mut Context) -> GameResult;
}

mod pawn {
    use ggez::*;
    pub struct Ball {
        pub x: f32,
        pub y: f32,
        pub vx: f32,
        pub vy: f32,
        pub r: f32,
    }

    impl Ball {
        pub fn bounce_away(&mut self, norm_vec: &nalgebra::Vector2<f32>) {
            let veloc_vec = nalgebra::Vector2::new(self.vx, self.vy);
            let bouce_vec = veloc_vec - 2.0 * veloc_vec.dot(&norm_vec) * norm_vec;
            self.vx = bouce_vec.x;
            self.vy = bouce_vec.y;
        }
    }
}

impl pawn::Ball {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        pawn::Ball {
            x: settings::BALL_SPAWN.0,
            y: settings::BALL_SPAWN.1,
            vx: rng.gen::<f32>() * 3.0 - 1.5,
            vy: -5.0,
            r: 3.0,
        }
    }
}

impl VisualComponent for pawn::Ball {
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
        self.x += self.vx;
        self.y += self.vy;
        Ok(())
    }
    fn draw(&self, ctx: &mut Context) -> GameResult {
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            mint::Point2 {
                x: self.x,
                y: self.y,
            },
            self.r,
            0.1,
            graphics::WHITE,
        )?;
        graphics::draw(ctx, &circle, graphics::DrawParam::default())?;
        Ok(())
    }
}

struct Bar {
    xc: f32,
    yc: f32,
    pos: f32,
    l1: f32, // w/2
    l2: f32, // h/2
    phi: f32,
    color: graphics::Color,
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
    fn collision(&self, ball: &pawn::Ball) -> Option<nalgebra::Vector2<f32>> {
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

struct InputState {
    left: bool,
    right: bool,
}

impl Default for InputState {
    fn default() -> Self {
        InputState {
            left: false,
            right: false,
        }
    }
}

struct Player {
    barpos: f32,
    bars: Vec<Bar>,
    input: InputState,
}

impl Player {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
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
    fn draw(&self, ctx: &mut Context) -> GameResult {
        for bar in self.bars.iter() {
            bar.draw(ctx)?;
        }
        Ok(())
    }
}

struct GameState {
    players: Vec<Player>,
    blocks: block::HexagonalGrid,
    balls: Vec<pawn::Ball>,
}

impl GameState {
    pub fn new() -> Self {
        let mut players = Vec::new();
        for p in 0..settings::NUMBER_PLAYERS {
            let mut bars = Vec::new();
            for ang in (p..6).step_by(settings::NUMBER_PLAYERS) {
                bars.push(Bar {
                    pos: 0.5,
                    xc: 0.0,
                    yc: 0.0,
                    l1: settings::BAR_SIZE.0 / 2.0,
                    l2: settings::BAR_SIZE.1 / 2.0,
                    phi: ang as f32 * 60.0,
                    color: graphics::Color::new(
                        if p == 0 { 1.0 } else { 0.0 },
                        if p == 1 { 1.0 } else { 0.0 },
                        if p == 2 { 1.0 } else { 0.0 },
                        1.0,
                    ),
                });
            }
            players.push(Player {
                barpos: 0.5,
                bars: bars,
                input: InputState::default(),
            });
        }
        GameState {
            players: players,
            blocks: block::HexagonalGrid::new(9, 20.0), // TODO as parameter + autoscale
            balls: vec![pawn::Ball::new()],
        }
    }

    fn collision(&mut self) {
        let mut balls_lost = Vec::new();
        for (ball_index, ball) in self.balls.iter_mut().enumerate() {
            // ball colliding with walls
            if ball.x < 0.0
                || ball.y < 0.0
                || ball.x > settings::SCREEN_SIZE.0
                || ball.y > settings::SCREEN_SIZE.1
            {
                // hit - respanw ball
                balls_lost.push(ball_index);
                break;
            }

            // ball colliding with bars
            for player in self.players.iter() {
                for bar in player.bars.iter() {
                    let collision = bar.collision(&ball);
                    if let Some(norm_vec) = collision {
                        ball.bounce_away(&norm_vec);
                        break;
                    }
                }
            }

            // ball colliding with blocks
            let mut block_hit = usize::MAX;
            for (hexagon_index, hexagon) in self.blocks.tiles.iter().enumerate() {
                let collision = hexagon.collision(&ball);
                if let Some(norm_vec) = collision {
                    ball.bounce_away(&norm_vec);
                    block_hit = hexagon_index;
                    break;
                }
            }
            if block_hit < usize::MAX {
                if self.blocks.tiles[block_hit].hit() {
                    self.blocks.tiles.remove(block_hit);
                }
                break;
            }
        }
        if balls_lost.len() > 0 {
            for &ball_index in balls_lost.iter() {
                self.balls.remove(ball_index);
            }
        }
        if self.balls.is_empty() {
            self.balls.push(pawn::Ball::new());
        }
    }

    fn update_input(&mut self, keycode: KeyCode, key_pressed: bool) {
        match settings::NUMBER_PLAYERS {
            1 => match keycode {
                KeyCode::Left => self.players[0].input.left = key_pressed,
                KeyCode::Right => self.players[0].input.right = key_pressed,
                _ => (),
            },
            2 => match keycode {
                KeyCode::Left => self.players[0].input.left = key_pressed,
                KeyCode::Right => self.players[0].input.right = key_pressed,
                KeyCode::A => self.players[1].input.left = key_pressed,
                KeyCode::D => self.players[1].input.right = key_pressed,
                _ => (),
            },
            3 => match keycode {
                KeyCode::Left => self.players[0].input.left = key_pressed,
                KeyCode::Right => self.players[0].input.right = key_pressed,
                KeyCode::D => self.players[1].input.left = key_pressed,
                KeyCode::A => self.players[1].input.right = key_pressed,
                KeyCode::L => self.players[2].input.left = key_pressed,
                KeyCode::J => self.players[2].input.right = key_pressed,
                _ => (),
            },
            _ => panic!("Exceeded the number of players!"),
        }
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        for player in self.players.iter_mut() {
            player.update(ctx)?;
        }
        self.collision();
        for ball in self.balls.iter_mut() {
            ball.update(ctx)?;
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 0.2, 0.3, 1.0].into());
        self.blocks.draw(ctx)?;
        for player in self.players.iter() {
            player.draw(ctx)?;
        }
        for ball in self.balls.iter() {
            ball.draw(ctx)?;
        }
        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        self.update_input(keycode, true);
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        self.update_input(keycode, false);
    }
}

fn main() -> GameResult {
    let (ctx, events_loop) = &mut ggez::ContextBuilder::new("hexpong", "acerne")
        .window_setup(ggez::conf::WindowSetup::default().title("HexPong"))
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(settings::SCREEN_SIZE.0, settings::SCREEN_SIZE.1),
        )
        .build()?;

    let state = &mut GameState::new();
    event::run(ctx, events_loop, state)
}
