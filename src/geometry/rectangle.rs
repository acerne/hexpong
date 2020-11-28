use crate::geometry::shape::Shape;
use crate::geometry::{Point, Polygon, Scale, Size, Vector};

pub struct Rectangle {
    origin: Point,
    size: Size,
}

impl Rectangle {
    pub fn new(origin: Point, size: Size) -> Self {
        Self { origin, size }
    }
    pub fn resize(&mut self, scale: Scale) {
        self.size = self.size * scale;
    }
    pub fn resize_to(&mut self, size: Size) {
        self.size = size;
    }
}

impl Shape for Rectangle {
    fn r#move(&mut self, vector: Vector) {
        self.origin = self.origin + vector;
    }
    fn move_to(&mut self, point: Point) {
        self.origin = point;
    }
    fn to_polygon(&self) -> Polygon {
        let mut vertices = Vec::new();
        vertices.push(self.origin);
        vertices.push(self.origin + Vector::new(self.size.w, 0.0));
        vertices.push(self.origin + Vector::new(self.size.w, self.size.h));
        vertices.push(self.origin + Vector::new(0.0, self.size.h));
        Polygon { vertices }
    }
}
