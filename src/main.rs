use ggez::*;

use ggez::event::{KeyCode, KeyMods};
use ggez::{event, graphics, Context, GameResult};

struct Bar {
    pos: f32,
    w: f32,
    h: f32,
    r: f32,
    phi: f32,
}

impl Bar {
    pub fn new(w: f32, r: f32, phi: f32) -> Self {
        Bar {
            pos: 0.5,
            w: w,
            h: 5.0,
            r: r,
            phi: phi,
        }
    }
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 1.0, 0.0, 1.0].into());
        let rectangle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            [
                300.0 + ((self.w / 2.0 + self.pos) * self.r),
                400.0,
                self.w * self.r,
                self.h,
            ]
            .into(),
            [1.0, 0.5, 0.0, 1.0].into(),
        )?;
        graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
        Ok(())
    }
}

// fn polarToCarthesian(r: f32, phi: f32) -> mint::Point2<f32> {
//     let x = r * phi.cos();
//     let y = r * phi.sin();
//     return mint::Point2 { x, y };
// }

struct GameState {
    bar: Bar,
}

impl GameState {
    /// Our new function will set up the initial state of our game.
    pub fn new() -> Self {
        GameState {
            bar: Bar::new(0.2, 200.0, 60.0),
        }
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.bar.draw(ctx)?;
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
            self.bar.pos -= 0.05;
            if self.bar.pos < 0.05 {
                self.bar.pos = 0.0;
            }
        } else if keycode == KeyCode::Right {
            self.bar.pos += 0.05;
            if self.bar.pos > 0.95 {
                self.bar.pos = 1.0;
            }
        }
    }
}

fn main() -> GameResult {
    let (ctx, events_loop) = &mut ggez::ContextBuilder::new("hexpong", "acerne")
        .window_setup(ggez::conf::WindowSetup::default().title("HexPong"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 600.0))
        .build()?;

    let state = &mut GameState::new();
    event::run(ctx, events_loop, state)
}
