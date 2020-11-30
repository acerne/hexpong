use crate::geometry::shape::Shape;
use crate::geometry::{Point, Polygon, Scale, Size, Vector};
use float_eq::FloatEq;

pub struct Rectangle {
    center: Point,
    size: Size,
    phi: f32,
}

impl Rectangle {
    pub fn new(center: Point, size: Size, phi: f32) -> Self {
        Self { center, size, phi }
    }
    pub fn resize(&mut self, scale: Scale) {
        self.size = self.size * scale;
    }
    pub fn resize_to(&mut self, size: Size) {
        self.size = size;
    }
}

impl Shape for Rectangle {
    fn translate(&mut self, vector: Vector) {
        self.center = self.center + vector;
    }
    fn move_to(&mut self, point: Point) {
        self.center = point;
    }
    fn rotate(&mut self, theta: f32) {
        self.phi += theta;
    }
    fn rotate_to(&mut self, phi: f32) {
        self.phi = phi;
    }
    fn to_polygon(&self) -> Polygon {
        let mut vertices = Vec::new();
        vertices.reserve(4);
        let half_size = self.size / 2.0;
        let w_cos = half_size.w * self.phi.cos();
        let w_sin = half_size.w * self.phi.sin();
        let h_cos = half_size.h * self.phi.cos();
        let h_sin = half_size.h * self.phi.sin();
        vertices.push(self.center + Vector::new(-w_cos + h_sin, -w_sin - h_cos)); // (-x, -y)
        vertices.push(self.center + Vector::new(w_cos + h_sin, w_sin - h_cos)); // (x, -y)
        vertices.push(self.center + Vector::new(w_cos - h_sin, w_sin + h_cos)); // (x, y)
        vertices.push(self.center + Vector::new(-w_cos - h_sin, -w_sin + h_cos)); // (-x, y)
        Polygon { vertices }
    }
}

impl PartialEq for Rectangle {
    fn eq(&self, other: &Self) -> bool {
        self.center.eq_abs(&other.center, &10e-6)
            && self.size.eq_abs(&other.size, &10e-6)
            && float_eq::float_eq!(self.phi, other.phi, abs <= 10e-6)
    }
}

impl std::fmt::Debug for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Rectangle")
            .field("center", &self.center)
            .field("size", &self.size)
            .field("phi", &self.phi)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::shape::Shape;
    use crate::geometry::{Point, Rectangle, Size, Vector};
    use float_eq::FloatEq;
    #[test]
    fn test_translate() {
        // test axis aligned rectangle
        let mut rect = Rectangle::new(
            Point::new(10.0, -5.0),
            Size::new(2.0, 1.0),
            45f32.to_radians(),
        );
        rect.translate(Vector::new(-2.0, 1.0));
        let expected = Rectangle::new(
            Point::new(8.0, -4.0),
            Size::new(2.0, 1.0),
            45f32.to_radians(),
        );
        assert_eq!(rect, expected);
    }
    #[test]
    fn test_move_to() {
        // test axis aligned rectangle
        let mut rect = Rectangle::new(
            Point::new(10.0, -5.0),
            Size::new(2.0, 1.0),
            45f32.to_radians(),
        );
        rect.move_to(Point::new(-2.0, 1.0));
        let expected = Rectangle::new(
            Point::new(-2.0, 1.0),
            Size::new(2.0, 1.0),
            45f32.to_radians(),
        );
        assert_eq!(rect, expected);
    }
    #[test]
    fn test_rotate() {
        let mut rect = Rectangle::new(
            Point::new(10.0, -5.0),
            Size::new(2.0, 1.0),
            0f32.to_radians(),
        );
        rect.rotate(45f32.to_radians());
        let expected = Rectangle::new(
            Point::new(10.0, -5.0),
            Size::new(2.0, 1.0),
            45f32.to_radians(),
        );
        assert_eq!(rect, expected);
    }
    #[test]
    fn test_to_polygon_axis_aligned() {
        // test axis aligned rectangle
        let rect = Rectangle::new(Point::new(10.0, -5.0), Size::new(4.0, 2.0), 0.0);
        let poly = rect.to_polygon();
        let vert_a = Point::new(8.0, -6.0);
        let vert_b = Point::new(12.0, -6.0);
        let vert_c = Point::new(12.0, -4.0);
        let vert_d = Point::new(8.0, -4.0);
        assert!(
            poly.vertices[0].eq_abs(&vert_a, &10e-6)
                && poly.vertices[1].eq_abs(&vert_b, &10e-6)
                && poly.vertices[2].eq_abs(&vert_c, &10e-6)
                && poly.vertices[3].eq_abs(&vert_d, &10e-6),
            "{} == {}, {}, {}, {}",
            poly,
            vert_a,
            vert_b,
            vert_c,
            vert_d
        );
    }
    #[test]
    fn test_to_polygon_axis_aligned_90() {
        // test axis aligned rectangle, rotated 90 degrees
        let rect = Rectangle::new(
            Point::new(10.0, -5.0),
            Size::new(4.0, 2.0),
            90f32.to_radians(),
        );
        let poly = rect.to_polygon();
        let vert_a = Point::new(11.0, -7.0);
        let vert_b = Point::new(11.0, -3.0);
        let vert_c = Point::new(9.0, -3.0);
        let vert_d = Point::new(9.0, -7.0);
        assert!(
            poly.vertices[0].eq_abs(&vert_a, &10e-6)
                && poly.vertices[1].eq_abs(&vert_b, &10e-6)
                && poly.vertices[2].eq_abs(&vert_c, &10e-6)
                && poly.vertices[3].eq_abs(&vert_d, &10e-6),
            "{} == {}, {}, {}, {}",
            poly,
            vert_a,
            vert_b,
            vert_c,
            vert_d
        );
    }
    #[test]
    fn test_to_polygon_oriented() {
        // test oriented rectangle
        let rect = Rectangle::new(
            Point::new(10.0, -5.0),
            Size::new(2.0 / 2f32.sqrt(), 2.0 / 2f32.sqrt()),
            45f32.to_radians(),
        );
        let poly = rect.to_polygon();
        let vert_a = Point::new(10.0, -6.0);
        let vert_b = Point::new(11.0, -5.0);
        let vert_c = Point::new(10.0, -4.0);
        let vert_d = Point::new(9.0, -5.0);
        assert!(
            poly.vertices[0].eq_abs(&vert_a, &10e-6)
                && poly.vertices[1].eq_abs(&vert_b, &10e-6)
                && poly.vertices[2].eq_abs(&vert_c, &10e-6)
                && poly.vertices[3].eq_abs(&vert_d, &10e-6),
            "{} == {}, {}, {}, {}",
            poly,
            vert_a,
            vert_b,
            vert_c,
            vert_d
        );
    }
}
