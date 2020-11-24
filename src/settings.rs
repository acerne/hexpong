pub const HEXAGON_SIZE: f32 = 300.0; // TODO: scaling with screen
pub const SCREEN_SIZE: (f32, f32) = (800.0, 600.0);
pub const ORIGIN: (f32, f32) = (SCREEN_SIZE.0 / 2.0, SCREEN_SIZE.1 / 2.0);
pub const BALL_SPAWN: (f32, f32) = (ORIGIN.0, ORIGIN.1 + HEXAGON_SIZE - 50.0);
pub const NUMBER_PLAYERS: usize = 1;
