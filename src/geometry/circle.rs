use crate::geometry::Point;

pub struct Circle {
    center: Point,
    radius: f32,
}

impl Circle {
    fn new(center: Point, radius: f32) -> Self {
        Self { center, radius }
    }
}
