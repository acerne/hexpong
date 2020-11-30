use crate::geometry::base::{Point, Vector};
use crate::geometry::shape::Polygon;

pub trait Shape {
    fn translate(&mut self, vector: Vector);
    fn move_to(&mut self, point: Point);
    fn rotate(&mut self, theta: f32);
    fn rotate_to(&mut self, phi: f32);
    fn to_polygon(&self) -> Polygon;
    //fn to_bounding_box(&self) -> Box;
    //fn to_enclosing_circle(&self) -> Circle;
    //fn center_of_gravity(&self) -> Point
}
