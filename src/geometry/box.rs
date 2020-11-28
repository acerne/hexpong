use crate::geometry::{Point, Scale, Size, Vector};

pub struct Box {
    center: Point,
    size: Size,
    orientation: f32,
}

impl Box {
    pub fn new(center: Point, size: Size, orientation: f32) -> Self {
        Self {
            center,
            size,
            orientation,
        }
    }
    pub fn r#move(&mut self, vector: Vector) {
        self.center = self.center + vector;
    }
    pub fn move_to(&mut self, point: Point) {
        self.center = point;
    }
    pub fn resize(&mut self, scale: Scale) {
        self.size = self.size * scale;
    }
    pub fn resize_to(&mut self, size: Size) {
        self.size = size;
    }
    pub fn rotate(&mut self, rotation: f32) {
        self.orientation += rotation;
    }
    pub fn rotate_to(&mut self, orientation: f32) {
        self.orientation = orientation;
    }
}
