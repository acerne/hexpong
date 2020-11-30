use crate::geometry::shape::Shape;
use crate::geometry::{Point, Polygon, Vector};
use float_eq::FloatEq;

pub struct Circle {
    center: Point,
    radius: f32,
}

impl Circle {
    fn new(center: Point, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl Shape for Circle {
    fn translate(&mut self, vector: Vector) {
        self.center = self.center + vector;
    }
    fn move_to(&mut self, point: Point) {
        self.center = point;
    }
    fn rotate(&mut self, _theta: f32) {
        // does nothing
    }
    fn rotate_to(&mut self, _phi: f32) {
        // does nothing
    }
    fn to_polygon(&self) -> Polygon {
        // determine number of polygon vertices from radius
        let n_vertices = 4 + (4.0 * self.radius.sqrt().floor()) as usize;
        let mut vertices = Vec::new();
        vertices.reserve(n_vertices);
        let angle_step = (360.0 / n_vertices as f32).to_radians();
        for i in 0..n_vertices {
            vertices.push(
                self.center
                    + Vector::new(
                        self.radius * ((i as f32) * angle_step).cos(),
                        self.radius * ((i as f32) * angle_step).sin(),
                    ),
            )
        }
        Polygon { vertices }
    }
}

impl PartialEq for Circle {
    fn eq(&self, other: &Self) -> bool {
        self.center.eq_abs(&other.center, &10e-6)
            && float_eq::float_eq!(self.radius, other.radius, abs <= 10e-6)
    }
}

impl std::fmt::Debug for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Circle")
            .field("center", &self.center)
            .field("radius", &self.radius)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::shape::Shape;
    use crate::geometry::{Circle, Point, Vector};
    use float_eq::FloatEq;
    #[test]
    fn test_translate() {
        let mut circle = Circle::new(Point::new(10.0, -5.0), 10.0);
        circle.translate(Vector::new(-2.0, 1.0));
        let expected = Circle::new(Point::new(8.0, -4.0), 10.0);
        assert_eq!(circle, expected);
    }
    #[test]
    fn test_move_to() {
        let mut circle = Circle::new(Point::new(10.0, -5.0), 10.0);
        circle.move_to(Point::new(-2.0, 1.0));
        let expected = Circle::new(Point::new(-2.0, 1.0), 10.0);
        assert_eq!(circle, expected);
    }
    #[test]
    fn test_to_polygon() {
        let circle = Circle::new(Point::new(10.0, -5.0), 10.0);
        let poly = circle.to_polygon();
        let length = poly.vertices.len();
        assert!(length % 4 == 0);
        let vert_rightmost = Point::new(20.0, -5.0);
        let vert_bottommost = Point::new(10.0, 5.0);
        let vert_leftmost = Point::new(0.0, -5.0);
        let vert_topmost = Point::new(10.0, -15.0);
        assert!(
            poly.vertices[0].eq_abs(&vert_rightmost, &10e-6)
                && poly.vertices[length / 4].eq_abs(&vert_bottommost, &10e-6)
                && poly.vertices[2 * length / 4].eq_abs(&vert_leftmost, &10e-6)
                && poly.vertices[3 * length / 4].eq_abs(&vert_topmost, &10e-6),
            "{}, {}, {}, {} == {}, {}, {}, {}",
            poly.vertices[0],
            poly.vertices[length / 4],
            poly.vertices[2 * length / 4],
            poly.vertices[3 * length / 4],
            vert_rightmost,
            vert_bottommost,
            vert_leftmost,
            vert_topmost
        );
    }
}
