use ggez::*;

use ggez::event::{KeyCode, KeyMods};
use ggez::{event, graphics, Context, GameResult};

mod component;
mod levels;
mod settings;

trait VisualComponent {
    fn collision(&self, ball: &component::ball::Ball) -> Option<nalgebra::Vector2<f32>>;
    fn update(&mut self, _ctx: &mut Context) -> GameResult;
    fn draw(&self, ctx: &mut Context) -> GameResult;
}

pub struct InputState {
    pub left: bool,
    pub right: bool,
}

impl Default for InputState {
    fn default() -> Self {
        InputState {
            left: false,
            right: false,
        }
    }
}

struct GameState {
    players: Vec<component::player::Player>,
    level: levels::Level,
    balls: Vec<component::ball::Ball>,
}

impl GameState {
    pub fn new() -> Self {
        let mut players = Vec::new();
        for p in 0..settings::NUMBER_PLAYERS {
            let mut bars = Vec::new();
            for ang in (p..6).step_by(settings::NUMBER_PLAYERS) {
                bars.push(component::player::Bar {
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
            players.push(component::player::Player {
                barpos: 0.5,
                bars: bars,
                input: InputState::default(),
            });
        }
        GameState {
            players: players,
            level: levels::Level::new(String::from("levels/basic.yaml")), // TODO as parameter + autoscale
            balls: vec![component::ball::Ball::new()],
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
            for (hexagon_index, hexagon) in self.level.blocks.iter().enumerate() {
                let collision = hexagon.collision(&ball);
                if let Some(norm_vec) = collision {
                    ball.bounce_away(&norm_vec);
                    block_hit = hexagon_index;
                    break;
                }
            }
            if block_hit < usize::MAX {
                if self.level.blocks[block_hit].hit() {
                    self.level.blocks.remove(block_hit);
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
            self.balls.push(component::ball::Ball::new());
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
        self.level.draw(ctx)?;
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
