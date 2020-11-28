use crate::geometry::{Point, Vector};

pub struct Polygon {
    pub vertices: Vec<Point>,
}

impl Polygon {
    pub fn new(vertices: &Vec<Point>) -> Self {
        let p = vertices.clone();
        Self { vertices: p }
    }
    pub fn from_vectors(start: Point, vectors: &Vec<Vector>) -> Self {
        let mut points = Vec::new();
        points.push(start);
        for vector in vectors.iter() {
            points.push(points.last().unwrap().clone() + vector.clone());
        }
        Self { points }
    }
}
