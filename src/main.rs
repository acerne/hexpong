use ggez::*;

use crate::geometry::base::*;
use ggez::event::{KeyCode, KeyMods};
use ggez::{audio, graphics, Context, GameResult};

mod component;
mod gamemode;
mod geometry;
mod levels;
mod settings;
mod themes;

trait AudibleComponent {
    fn play_sound(&self, ctx: &mut Context);
}

trait VisualComponent {
    fn collision(&self, ball: &component::ball::Ball) -> Option<Vector>;
    fn update(&mut self, _ctx: &mut Context) -> GameResult;
    fn draw(&self, ctx: &mut Context, theme: &themes::Theme) -> GameResult;
    fn create_mesh(&mut self, ctx: &mut Context) -> Option<graphics::Mesh>;
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
    players: Vec<component::controller::Controller>,
    walls: Vec<component::wall::Wall>,
    level: levels::Level,
    balls: Vec<component::ball::Ball>,
    ball_speed: f32,
    theme: themes::Theme,
}

impl GameState {
    pub fn new() -> Self {
        let mode = gamemode::GameMode::new(
            "config/gamemodes/arcade-singleplayer.yaml",
            gamemode::Difficulty::Easy,
        );
        GameState {
            players: mode.players,
            walls: mode.walls,
            ball_speed: mode.ball_speed,
            level: levels::Level::new(String::from("config/levels/crowded.yaml")),
            balls: vec![component::ball::Ball::new(mode.ball_speed)],
            theme: themes::Theme::new(String::from("config/themes/base.yaml")),
        }
    }

    fn collision(&mut self, ctx: &mut Context) {
        let mut balls_lost = Vec::new();
        for (ball_index, ball) in self.balls.iter_mut().enumerate() {
            // ball going out of sight
            if ball.shape.center.x < -settings::UNIT_SIZE // TODO: better bounds, boundary ownership for multiplayer
                || ball.shape.center.y < -settings::UNIT_SIZE
                || ball.shape.center.x > settings::UNIT_SIZE
                || ball.shape.center.y > settings::UNIT_SIZE
            {
                // respanw ball
                balls_lost.push(ball_index);
                break;
            }

            // ball colliding with walls
            for wall in self.walls.iter() {
                let collision = wall.collision(&ball);
                if let Some(norm_vec) = collision {
                    wall.play_sound(ctx);
                    ball.bounce_away(norm_vec);
                    break;
                }
            }

            // ball colliding with bars
            for player in self.players.iter() {
                for bar in player.bars.iter() {
                    let collision = bar.collision(&ball);
                    if let Some(norm_vec) = collision {
                        bar.play_sound(ctx);
                        ball.bounce_away(norm_vec);
                        break;
                    }
                }
            }

            // ball colliding with blocks
            let mut block_hit = usize::MAX;
            for (hexagon_index, hexagon) in self.level.blocks.iter().enumerate() {
                let collision = hexagon.collision(&ball);
                if let Some(norm_vec) = collision {
                    hexagon.play_sound(ctx);
                    ball.bounce_away(norm_vec);
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
            self.balls.push(component::ball::Ball::new(self.ball_speed));
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
        for ball in self.balls.iter_mut() {
            ball.update(ctx)?;
        }
        for block in self.level.blocks.iter_mut() {
            block.update(ctx)?
        }
        for wall in self.walls.iter_mut() {
            wall.update(ctx)?
        }
        self.collision(ctx);
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, self.theme.background);
        self.level.draw(ctx, &self.theme)?;
        for wall in self.walls.iter() {
            wall.draw(ctx, &self.theme)?;
        }
        for player in self.players.iter() {
            player.draw(ctx, &self.theme)?;
        }
        for ball in self.balls.iter() {
            ball.draw(ctx, &self.theme)?;
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
    let resource_dir = if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = std::path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        std::path::PathBuf::from("./resources")
    };

    let (ctx, events_loop) = &mut ggez::ContextBuilder::new("hexpong", "acerne")
        .window_setup(ggez::conf::WindowSetup::default().title("HexPong"))
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(settings::SCREEN_SIZE.0, settings::SCREEN_SIZE.1),
        )
        .add_resource_path(resource_dir)
        .build()?;

    let state = &mut GameState::new();
    event::run(ctx, events_loop, state)
}
