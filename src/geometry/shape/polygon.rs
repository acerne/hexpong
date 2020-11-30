use crate::geometry::base::{Point, Vector};

pub struct Polygon {
    pub vertices: Vec<Point>,
}

impl Polygon {
    pub fn new(vertices: &Vec<Point>) -> Self {
        let p = vertices.clone();
        Self { vertices: p }
    }
    pub fn from_vectors(start: Point, vectors: &Vec<Vector>) -> Self {
        let mut vertices = Vec::new();
        vertices.push(start);
        for vector in vectors.iter() {
            vertices.push(vertices.last().unwrap().clone() + vector.clone());
        }
        Self { vertices }
    }
}

impl std::fmt::Display for Polygon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let first = self.vertices.first();
        if let Some(first) = first {
            write!(f, "{}", first)?;
            for item in self.vertices.iter().skip(1) {
                write!(f, ", {}", item)?;
            }
        }
        Ok(())
    }
}
