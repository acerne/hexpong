use crate::geometry::{Point, Scale, Size, Vector};

pub struct Rectangle {
    origin: Point,
    size: Size,
}

impl Rectangle {
    pub fn new(origin: Point, size: Size) -> Self {
        Self { origin, size }
    }
    pub fn r#move(&mut self, vector: Vector) {
        self.origin = self.origin + vector;
    }
    pub fn move_to(&mut self, point: Point) {
        self.origin = point;
    }
    pub fn resize(&mut self, scale: Scale) {
        self.size = self.size * scale;
    }
    pub fn resize_to(&mut self, size: Size) {
        self.size = size;
    }
}
