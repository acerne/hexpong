use ggez::*;

use ggez::event::{KeyCode, KeyMods};
use ggez::{event, graphics, Context, GameResult};

use rand::Rng;

const HEXAGON_SIZE: f32 = 300.0;
const SCREEN_SIZE: (f32, f32) = (800.0, 600.0);
const ORIGIN: (f32, f32) = (SCREEN_SIZE.0 / 2.0, SCREEN_SIZE.1 / 2.0);
const BAR_SIZE: (f32, f32) = (100.0, 12.0);

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

// HEXMATH
struct Hexagon {
    x: f32,
    y: f32,
    r: f32,
    phi: f32,
}

impl Hexagon {
    pub fn new(x: f32, y: f32, r: f32) -> Self {
        Hexagon {
            x: x,
            y: y,
            r: r,
            phi: 0.0,
        }
    }
    fn get_vertices(&self) -> [mint::Point2<f32>; 6] {
        let mut vertices: [mint::Point2<f32>; 6] = [mint::Point2 { x: 0.0, y: 0.0 }; 6];
        for i in 0..6 {
            let angle = (self.phi + 30.0 + i as f32 * 60.0).to_radians();
            let xh = angle.cos() * self.r + self.x;
            let yh = angle.sin() * self.r + self.y;
            vertices[i] = mint::Point2 { x: xh, y: yh };
        }
        vertices
    }
    fn draw(&self, ctx: &mut Context) -> GameResult {
        let vertices = self.get_vertices();
        let polygon = graphics::Mesh::new_polygon(
            ctx,
            graphics::DrawMode::fill(),
            &vertices,
            [0.5, 1.0, 0.5, 1.0].into(),
        )?;
        graphics::draw(
            ctx,
            &polygon,
            ggez::graphics::DrawParam::from((ggez::mint::Point2 { x: 0.0, y: 0.0 },)),
        )?;
        Ok(())
    }
    fn draw_trace(&self, ctx: &mut Context) -> GameResult {
        let vertices = self.get_vertices();
        let trace = graphics::Mesh::new_polygon(
            ctx,
            graphics::DrawMode::stroke(3.0),
            &vertices,
            [0.8, 0.8, 0.8, 0.6].into(),
        )?;
        graphics::draw(
            ctx,
            &trace,
            ggez::graphics::DrawParam::from((ggez::mint::Point2 { x: 0.0, y: 0.0 },)),
        )?;
        Ok(())
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct GridIndex {
    q: i32,
    r: i32,
    s: i32,
}

impl GridIndex {
    fn to_pixel(&self, tile_radius: f32) -> mint::Point2<f32> {
        let x = ORIGIN.0
            + (self.q as f32 * 3.0f32.sqrt() + self.r as f32 * (3.0f32.sqrt() / 2.0)) * tile_radius;
        let y = ORIGIN.1 + (3.0 / 2.0 * self.r as f32) * tile_radius;
        mint::Point2 { x: x, y: y }
    }
}

struct HexagonalGrid {
    tiles: Vec<Hexagon>,
}

impl HexagonalGrid {
    pub fn new(grid_size: u16, tile_radius: f32) -> Self {
        let grid_radius = ((grid_size + 1) / 2) as i32;
        let mut tiles = Vec::new();
        for q in (-grid_radius + 1)..grid_radius {
            for r in std::cmp::max(-grid_radius + 1, -q - grid_radius + 1)
                ..=std::cmp::min(grid_radius - 1, -q + grid_radius - 1)
            {
                let s = -q - r;
                let index = GridIndex { q: q, r: r, s: s };
                let point = index.to_pixel(tile_radius);
                tiles.push(Hexagon::new(point.x, point.y, tile_radius));
            }
        }
        HexagonalGrid { tiles: tiles }
    }
    fn draw(&self, ctx: &mut Context) -> GameResult {
        for hexagon in self.tiles.iter() {
            hexagon.draw(ctx)?;
        }
        for hexagon in self.tiles.iter() {
            hexagon.draw_trace(ctx)?;
        }
        Ok(())
    }
}

// !HEXMATH

struct Ball {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    r: f32,
}

impl Ball {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        Ball {
            x: ORIGIN.0,
            y: ORIGIN.1 + HEXAGON_SIZE - 50.0,
            vx: rng.gen::<f32>() * 3.0 - 1.5,
            vy: -5.0,
            r: 3.0,
        }
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
    pos: f32,
    xc: f32,
    yc: f32,
    l1: f32, // w/2
    l2: f32, // h/2
    phi: f32,
}

impl Bar {
    pub fn new(phi: f32) -> Self {
        Bar {
            pos: 0.5,
            xc: 0.0,
            yc: 0.0,
            l1: BAR_SIZE.0 / 2.0,
            l2: BAR_SIZE.1 / 2.0,
            phi: phi,
        }
    }
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        let xc0 = ORIGIN.0
            + if (self.phi % 120.0) == 0.0 {
                -HEXAGON_SIZE / 2.0 + self.pos * HEXAGON_SIZE //- self.w / 2.0
            } else {
                HEXAGON_SIZE / 2.0 - self.pos * HEXAGON_SIZE // - self.w / 2.0
            };
        let yc0 = ORIGIN.1 + 3.0f32.sqrt() / 2.0 * HEXAGON_SIZE;

        let phi = self.phi.to_radians();

        self.xc = (xc0 - ORIGIN.0) * phi.cos() + (yc0 - ORIGIN.1) * phi.sin() + ORIGIN.0;
        self.yc = -(xc0 - ORIGIN.0) * phi.sin() + (yc0 - ORIGIN.1) * phi.cos() + ORIGIN.1;

        Ok(())
    }
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
    fn draw(&self, ctx: &mut Context) -> GameResult {
        let vertices = self.get_vertices();
        let polygon = graphics::Mesh::new_polygon(
            ctx,
            graphics::DrawMode::fill(),
            &vertices,
            [1.0, 0.5, 0.0, 1.0].into(),
        )?;
        graphics::draw(
            ctx,
            &polygon,
            ggez::graphics::DrawParam::from((ggez::mint::Point2 { x: 0.0, y: 0.0 },)),
        )?;
        Ok(())
    }
}

struct GameState {
    bars: Vec<Bar>,
    blocks: HexagonalGrid,
    ball: Ball,
    barpos: f32,
    input: InputState,
}

impl GameState {
    pub fn new() -> Self {
        let mut bars = Vec::new();
        for ang in 0..6 {
            bars.push(Bar::new(ang as f32 * 60.0));
        }
        GameState {
            bars: bars,
            blocks: HexagonalGrid::new(9, 20.0), // TODO as parameter + autoscale
            ball: Ball::new(),
            barpos: 0.5,
            input: InputState::default(),
        }
    }

    fn collision(&mut self) {
        // ball colliding with walls
        if self.ball.x < 0.0
            || self.ball.y < 0.0
            || self.ball.x > SCREEN_SIZE.0
            || self.ball.y > SCREEN_SIZE.1
        {
            // hit - respanw ball
            self.ball = Ball::new();
            return;
        }

        // ball colliding with bars
        for bar in self.bars.iter() {
            let phi = (-bar.phi).to_radians();
            let tx = (self.ball.x - bar.xc) * phi.cos() + (self.ball.y - bar.yc) * phi.sin();
            let ty = -(self.ball.x - bar.xc) * phi.sin() + (self.ball.y - bar.yc) * phi.cos();
            if tx > -bar.l1 && tx < bar.l1 && ty > -bar.l2 && ty < bar.l2 {
                // hit - bounce off
                let norm_vec = nalgebra::Vector2::new(
                    (phi + std::f32::consts::PI / 2.0).cos(),
                    (phi + std::f32::consts::PI / 2.0).sin(),
                );
                let veloc_vec = nalgebra::Vector2::new(self.ball.vx, self.ball.vy);
                let bouce_vec = veloc_vec - 2.0 * veloc_vec.dot(&norm_vec) * norm_vec;
                self.ball.vx = bouce_vec.x;
                self.ball.vy = bouce_vec.y;
                return;
            }
        }

        // ball colliding with blocks
        let mut to_destroy = usize::MAX;
        for (pos, hexagon) in self.blocks.tiles.iter().enumerate() {
            let dist =
                ((hexagon.x - self.ball.x).powf(2.0) + (hexagon.y - self.ball.y).powf(2.0)).sqrt();
            if dist < hexagon.r + self.ball.r {
                // hit - bounce off and destroy block
                let norm_vec = nalgebra::Vector2::new(
                    (hexagon.x - self.ball.x) / dist,
                    (hexagon.y - self.ball.y) / dist,
                );
                let veloc_vec = nalgebra::Vector2::new(self.ball.vx, self.ball.vy);
                let bouce_vec = veloc_vec - 2.0 * veloc_vec.dot(&norm_vec) * norm_vec;
                self.ball.vx = bouce_vec.x;
                self.ball.vy = bouce_vec.y;

                to_destroy = pos;
                break;
            }
        }
        if to_destroy < usize::MAX {
            self.blocks.tiles.remove(to_destroy);
        }
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if self.input.left {
            self.barpos -= 0.03; // TODO as parameter
        } else if self.input.right {
            self.barpos += 0.03;
        }
        if self.barpos < (0.0 + BAR_SIZE.0 / HEXAGON_SIZE / 2.0) {
            self.barpos = 0.0 + BAR_SIZE.0 / HEXAGON_SIZE / 2.0;
        }
        if self.barpos > (1.0 - BAR_SIZE.0 / HEXAGON_SIZE / 2.0) {
            self.barpos = 1.0 - BAR_SIZE.0 / HEXAGON_SIZE / 2.0;
        }
        for bar in self.bars.iter_mut() {
            bar.pos = self.barpos;
            bar.update(ctx)?;
        }
        self.collision();
        self.ball.update(ctx)?;
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 0.2, 0.3, 1.0].into());
        self.blocks.draw(ctx)?;
        for bar in self.bars.iter() {
            bar.draw(ctx)?;
        }
        self.ball.draw(ctx)?;
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
        if keycode == KeyCode::Left {
            self.input.left = true;
        } else if keycode == KeyCode::Right {
            self.input.right = true;
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        if keycode == KeyCode::Left {
            self.input.left = false;
        } else if keycode == KeyCode::Right {
            self.input.right = false;
        }
    }
}

fn main() -> GameResult {
    let (ctx, events_loop) = &mut ggez::ContextBuilder::new("hexpong", "acerne")
        .window_setup(ggez::conf::WindowSetup::default().title("HexPong"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()?;

    let state = &mut GameState::new();
    event::run(ctx, events_loop, state)
}
