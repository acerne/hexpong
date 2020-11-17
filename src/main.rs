use ggez::*;

use ggez::event::{KeyCode, KeyMods};
use ggez::{event, graphics, Context, GameResult};

use std::collections::HashMap;

const HEXAGON_SIZE: f32 = 300.0;
const SCREEN_SIZE: (f32, f32) = (800.0, 600.0);
const ORIGIN: (f32, f32) = (SCREEN_SIZE.0 / 2.0, SCREEN_SIZE.1 / 2.0);
const BAR_SIZE: (f32, f32) = (50.0, 5.0);

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
    vertices: [mint::Point2<f32>; 6],
}

impl Hexagon {
    pub fn new(x: f32, y: f32, r: f32) -> Self {
        Hexagon {
            x: x,
            y: y,
            r: r,
            phi: 0.0,
            vertices: [mint::Point2 { x: 0.0, y: 0.0 }; 6],
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
    tiles: HashMap<GridIndex, Hexagon>,
}

impl HexagonalGrid {
    pub fn new(grid_size: u16, tile_radius: f32) -> Self {
        let grid_radius = ((grid_size + 1) / 2) as i32;
        let mut tiles = HashMap::new();
        for q in (-grid_radius + 1)..grid_radius {
            for r in std::cmp::max(-grid_radius + 1, -q - grid_radius + 1)
                ..=std::cmp::min(grid_radius - 1, -q + grid_radius - 1)
            {
                println!("{0}, {1}", q, r);
                let s = -q - r;
                let index = GridIndex { q: q, r: r, s: s };
                let point = index.to_pixel(tile_radius);
                tiles.insert(index, Hexagon::new(point.x, point.y, tile_radius));
            }
        }
        HexagonalGrid { tiles: tiles }
    }
    fn draw(&self, ctx: &mut Context) -> GameResult {
        for (_, hexagon) in &self.tiles {
            hexagon.draw(ctx)?;
        }
        for (_, hexagon) in &self.tiles {
            hexagon.draw_trace(ctx)?;
        }
        Ok(())
    }
}

// !HEXMATH

struct Bar {
    pos: f32,
    w: f32,
    h: f32,
    phi: f32,
}

impl Bar {
    pub fn new(phi: f32) -> Self {
        Bar {
            pos: 0.5,
            w: BAR_SIZE.0,
            h: BAR_SIZE.1,
            phi: phi,
        }
    }
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&self, ctx: &mut Context) -> GameResult {
        let xc = ORIGIN.0
            + if (self.phi % (2.0 * std::f32::consts::PI / 3.0)) != 0.0 {
                -HEXAGON_SIZE / 2.0 + self.pos * HEXAGON_SIZE - self.w / 2.0
            } else {
                HEXAGON_SIZE / 2.0 - self.pos * HEXAGON_SIZE - self.w / 2.0
            };
        let yc = ORIGIN.1 + 3.0f32.sqrt() / 2.0 * HEXAGON_SIZE;

        let rect = graphics::Rect::new(xc, yc, self.w, self.h);
        let rectangle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            rect.into(),
            [1.0, 0.5, 0.0, 1.0].into(),
        )?;

        graphics::draw(
            ctx,
            &rectangle,
            ggez::graphics::DrawParam::from((
                ggez::mint::Point2 { x: 0.0, y: 0.0 },
                self.phi + std::f32::consts::PI,
                ggez::mint::Point2 {
                    x: ORIGIN.0,
                    y: ORIGIN.1,
                },
                [1.0, 0.5, 0.0, 1.0].into(),
            )),
        )?;
        Ok(())
    }
}

struct GameState {
    bars: Vec<Bar>,
    blocks: HexagonalGrid,
    barpos: f32,
    input: InputState,
}

impl GameState {
    pub fn new() -> Self {
        let mut bars = Vec::new();
        for ang in 0..6 {
            bars.push(Bar::new(ang as f32 * std::f32::consts::PI / 3.0));
        }
        GameState {
            bars: bars,
            blocks: HexagonalGrid::new(9, 20.0), // TODO as parameter
            barpos: 0.5,
            input: InputState::default(),
        }
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if self.input.left {
            self.barpos -= 0.03; // TODO as parameter + autoscale
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
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 0.2, 0.3, 1.0].into());
        self.blocks.draw(ctx)?;
        for bar in self.bars.iter() {
            bar.draw(ctx)?;
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
