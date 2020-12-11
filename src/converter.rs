use geometry::base::{Point, Vector};
use geometry::shape::Polygon;
use ggez::*;

#[allow(dead_code)]
pub fn convert_to_point(point: &Point) -> mint::Point2<f32> {
    mint::Point2 {
        x: point.x,
        y: point.y,
    }
}

#[allow(dead_code)]
pub fn convert_to_vector(vector: &Vector) -> nalgebra::Vector2<f32> {
    nalgebra::Vector2::new(vector.dx, vector.dy)
}

#[allow(dead_code)]
pub fn convert_to_points(polygon: &Polygon) -> Vec<mint::Point2<f32>> {
    let mut points = Vec::new();
    for vert in polygon.vertices.iter() {
        points.push(convert_to_point(vert));
    }
    points
}
