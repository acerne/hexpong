use ggez::*;
pub const SCREEN_SIZE: (f32, f32) = (800.0, 600.0);
pub const UNIT_SIZE: f32 = 1000.0;
pub const ORIGIN: (f32, f32) = (SCREEN_SIZE.0 / 2.0, SCREEN_SIZE.1 / 2.0);
pub const BALL_SPAWN: (f32, f32) = (0.0, 0.5 * UNIT_SIZE);
pub const NUMBER_PLAYERS: usize = 1;

pub fn get_origin() -> mint::Point2<f32> {
    mint::Point2 {
        x: ORIGIN.0,
        y: ORIGIN.1,
    }
}
pub fn get_scale_vector() -> mint::Vector2<f32> {
    let factor = get_scale_factor();
    mint::Vector2 {
        x: factor,
        y: factor,
    }
}
pub fn get_scale_factor() -> f32 {
    (SCREEN_SIZE.0).min(SCREEN_SIZE.1) / (UNIT_SIZE * 2.0)
}
pub fn unit_to_pixel(value: f32) -> f32 {
    value * get_scale_factor()
}
pub fn pixel_to_unit(value: f32) -> f32 {
    value / get_scale_factor()
}
pub fn norm_to_unit(value: f32) -> f32 {
    value * UNIT_SIZE
}
