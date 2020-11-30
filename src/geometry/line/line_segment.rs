use crate::geometry::base::{Point, Vector};

pub struct LineSegment {
    pub origin: Point,
    pub end: Point,
}

impl LineSegment {
    pub fn new(origin: Point, end: Point) -> Self {
        Self { origin, end }
    }
    pub fn from_vector(point: Point, vector: Vector) -> Self {
        Self {
            origin: point,
            end: point + vector,
        }
    }
    pub fn to_vector(&self) -> Vector {
        let diff = self.origin - self.end;
        Vector {
            dx: diff.x,
            dy: diff.y,
        }
    }
    pub fn is_on_segment(&self, point: Point) -> bool {
        point.distance_to(self.origin) + point.distance_to(self.end)
            == self.origin.distance_to(self.end)
    }
    fn intersection(&self, other: LineSegment) -> Option<Point> {
        let a1 = self.end.y - self.origin.y;
        let b1 = self.origin.x - self.end.x;
        let c1 = a1 * self.origin.x + b1 * self.origin.y;
        let a2 = other.end.y - other.origin.y;
        let b2 = other.origin.x - other.end.x;
        let c2 = a2 * other.origin.x + b2 * other.origin.y;
        let delta = a1 * b2 - a2 * b1;
        if delta == 0.0 {
            return None;
        }
        let intersection = Point {
            x: (b2 * c1 - b1 * c2) / delta,
            y: (a1 * c2 - a2 * c1) / delta,
        };
        if !self.is_on_segment(intersection) {
            return None;
        }
        Some(intersection)
    }
}
impl Copy for LineSegment {}

impl Clone for LineSegment {
    fn clone(&self) -> Self {
        Self {
            origin: self.origin.clone(),
            end: self.end.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::base::{Point, Vector};
    use crate::geometry::line::LineSegment;
    use float_eq::FloatEq;

    #[test]
    fn test_from_vector() {
        let point_a = Point::new(1.0, 1.0);
        let point_b = Point::new(-1.0, -1.0);
        let vector = Vector::from_points(point_a, point_b);
        let line = LineSegment::from_vector(point_a, vector);
        assert!(
            line.origin.eq_abs(&point_a, &10e-6),
            "{} == {}",
            line.origin,
            point_a
        );
        assert!(
            line.end.eq_abs(&point_b, &10e-6),
            "{} == {}",
            line.origin,
            point_b
        );
    }
    #[test]
    fn test_is_on_segment() {
        let point_a = Point::new(1.0, 1.0);
        let point_b = Point::new(-1.0, -1.0);
        let line = LineSegment::new(point_a, point_b);
        // test point on line segment
        let test_point = Point::zero();
        assert!(line.is_on_segment(test_point));
        // test point not on line segment
        let test_point = Point::new(1.0, -1.0);
        assert!(!line.is_on_segment(test_point));
        // test point on line, but not on line segment
        let test_point = Point::new(2.0, 2.0);
        assert!(!line.is_on_segment(test_point));
    }
    #[test]
    fn test_intersection() {
        // test intersecting line segments
        let line_a = LineSegment::new(Point::new(1.0, 1.0), Point::new(-1.0, -1.0));
        let line_b = LineSegment::new(Point::new(1.0, -1.0), Point::new(-1.0, 1.0));
        let intersection = line_a.intersection(line_b);
        assert!(!intersection.is_none());
        assert_eq!(intersection.unwrap(), Point::zero());

        // test parallel line segments
        let line_a = LineSegment::new(Point::new(1.0, 1.0), Point::new(1.0, -1.0));
        let line_b = LineSegment::new(Point::new(-1.0, 1.0), Point::new(-1.0, -1.0));
        let intersection = line_a.intersection(line_b);
        assert!(intersection.is_none());

        // test intersecting lines, but not line segments
        let line_a = LineSegment::new(Point::new(1.0, 1.0), Point::new(2.0, -1.0));
        let line_b = LineSegment::new(Point::new(-1.0, 1.0), Point::new(-2.0, -1.0));
        let intersection = line_a.intersection(line_b);
        assert!(intersection.is_none());
    }
}
