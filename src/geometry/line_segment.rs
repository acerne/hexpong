use crate::geometry::{Point, Vector};

pub struct LineSegment {
    pub a: Point,
    pub b: Point,
}

impl LineSegment {
    pub fn new(a: Point, b: Point) -> Self {
        Self { a, b }
    }
    pub fn from_vector(point: Point, vector: Vector) -> Self {
        Self {
            a: point,
            b: point + vector,
        }
    }
    pub fn is_on_segment(&self, point: Point) -> bool {
        point.distance_to(self.a) + point.distance_to(self.b) == self.a.distance_to(self.b)
    }
    fn intersection(&self, other: LineSegment) -> Option<Point> {
        let a1 = self.b.y - self.a.y;
        let b1 = self.a.x - self.b.x;
        let c1 = a1 * self.a.x + b1 * self.a.y;
        let a2 = other.b.y - other.a.y;
        let b2 = other.a.x - other.b.x;
        let c2 = a2 * other.a.x + b2 * other.a.y;
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

#[cfg(test)]
mod tests {
    use crate::geometry::LineSegment;
    use float_eq::FloatEq;

    #[test]
    fn test_distance_to() {
        let point_a = Point::new(5.0, 5.0);
        let point_b = Point::new(-5.0, 5.0);
        let distance = point_a.distance_to(point_b);
        assert_eq!(distance, 10f32);
    }
}
