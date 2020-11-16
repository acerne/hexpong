use ggez::*;

use ggez::event::{KeyCode, KeyMods};
use ggez::{event, graphics, Context, GameResult};

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

struct Bar {
    pos: f32,
    w: f32,
    h: f32,
    r: f32,
    phi: f32,
}

impl Bar {
    pub fn new(r: f32, phi: f32) -> Self {
        Bar {
            pos: 0.5,
            w: BAR_SIZE.0,
            h: BAR_SIZE.1,
            r: r,
            phi: phi,
        }
    }
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&self, ctx: &mut Context) -> GameResult {
        let xc = ORIGIN.0 + self.pos * HEXAGON_SIZE + HEXAGON_SIZE * (self.phi).cos();
        let yc = ORIGIN.1 + HEXAGON_SIZE * (self.phi).sin();

        let rect = graphics::Rect::new(xc, yc, self.w, self.h);
        let rectangle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            rect.into(),
            [1.0, 1.0 * self.phi / (2.0 * std::f32::consts::PI), 0.0, 1.0].into(),
        )?;

        graphics::draw(
            ctx,
            &rectangle,
            ggez::graphics::DrawParam::from((
                ggez::mint::Point2 { x: 0.0, y: 0.0 },
                self.phi + std::f32::consts::PI / 2.0,
                ggez::mint::Point2 { x: xc, y: yc },
                [1.0, 1.0 * self.phi / (2.0 * std::f32::consts::PI), 0.0, 1.0].into(),
            )),
        )?;
        Ok(())
    }
}

// fn polarToCarthesian(r: f32, phi: f32) -> mint::Point2<f32> {
//     let x = r * phi.cos();
//     let y = r * phi.sin();
//     return mint::Point2 { x, y };
// }

struct GameState {
    bars: Vec<Bar>,
    barpos: f32,
    input: InputState,
}

impl GameState {
    pub fn new() -> Self {
        let mut bars = Vec::new();
        for ang in 0..6 {
            bars.push(Bar::new(
                HEXAGON_SIZE,
                ang as f32 * std::f32::consts::PI / 3.0,
            ));
        }
        GameState {
            bars: bars,
            barpos: 0.5,
            input: InputState::default(),
        }
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if self.input.left {
            self.barpos -= 0.05;
        } else if self.input.right {
            self.barpos += 0.05;
        }
        if self.barpos < 0.0 {
            self.barpos = 0.0;
        }
        if self.barpos > 1.0 {
            self.barpos = 1.0;
        }
        for bar in self.bars.iter_mut() {
            bar.pos = self.barpos;
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 0.2, 0.3, 1.0].into());
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
