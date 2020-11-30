use crate::geometry::base::{Point, Vector};

pub struct Polyline {
    pub points: Vec<Point>,
}

impl Polyline {
    pub fn new(points: &Vec<Point>) -> Self {
        let p = points.clone();
        Self { points: p }
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
