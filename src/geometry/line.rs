use crate::geometry::point::Point;


struct Line {
    r: f32,
    phi: f32
}

impl Line {
    fn new(r: f32, phi: f32) -> Self {
    Self {r, phi}
    }
    fn from_slope_intercept(k: f32, n: f32)-> Self {

    }   
    fn to_slope_intercept(&self) -> (f32, f32) {

    } 
}

struct LineSegment {
    a: Point,
    b: Point,
}

impl LineSegment {
    fn new(a: Point, b: Point) -> Self {
        Self { a, b }
    }
    fn is_on_segment(&self, point: Point) -> bool {
        point.x <= self.a.x.max(self.b.x)
            && point.x >= self.a.x.min(self.b.x)
            && point.y <= self.a.y.max(self.b.y)
            && point.y >= self.a.y.min(self.b.y)
    }
    fn intersection(&self, other: LineSegment) -> Option<Point> {
        if self.a.x.max(self.b.x) >= other.a.x.min(other.b.x) {
            let k1 = (self.a.y - self.b.y) / (self.a.x - self.b.x);
            let k2 = (other.a.y - other.b.y) / (other.a.x - other.b.x);
            let 
        }
        None
    }
}
