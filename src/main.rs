use ggez::*;

use ggez::event::{KeyCode, KeyMods};
use ggez::{event, graphics, Context, GameResult};

const SCREEN_SIZE: (f32, f32) = (800.0, 600.0);
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
        graphics::clear(ctx, [0.0, 1.0, 0.0, 1.0].into());
        let rectangle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            [100.0 + (self.pos * self.r), 400.0, self.w, self.h].into(),
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
    input: InputState,
}

impl GameState {
    /// Our new function will set up the initial state of our game.
    pub fn new() -> Self {
        GameState {
            bar: Bar::new(200.0, 60.0),
            input: InputState::default(),
        }
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if self.input.left {
            self.bar.pos -= 0.05;
        } else if self.input.right {
            self.bar.pos += 0.05;
        }
        if self.bar.pos < 0.0 {
            self.bar.pos = 0.0;
        }
        if self.bar.pos > 1.0 {
            self.bar.pos = 1.0;
        }
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
