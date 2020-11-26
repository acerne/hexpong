use crate::geometry::point::Point;

struct Circle {
    center: Point,
    radius: f32,
}

impl Circle {
    fn new(center: Point, radius: f32) -> Self {
        Self { center, radius }
    }
}
