use crate::geometry::{Box, Circle, Point, Polygon, Vector};

pub trait Shape {
    fn r#move(&mut self, vector: Vector);
    fn move_to(&mut self, point: Point);
    fn to_polygon(&self) -> Polygon;
    //fn to_bounding_box(&self) -> Box;
    //fn to_enclosing_circle(&self) -> Circle;
    //fn center_of_gravity(&self) -> Point
}
