use crate::geometry::point::Point;
use crate::geometry::vector::Vector;

pub struct Line {
    pub r: f32,
    pub phi: f32,
}

impl Line {
    pub fn new(r: f32, phi: f32) -> Self {
        Self { r, phi }
    }
    pub fn from_vector(vector: Vector) -> Self {
        let r = vector.get_length();
        let phi = vector.get_orientation() + std::f32::consts::PI / 2.0;
        Self { r, phi }
    }
    pub fn from_points(a: Point, b: Point) -> Self {
        let r = a.distance_to(b);
        let phi = (b.y - a.y).atan2(b.x - a.x) + std::f32::consts::PI / 2.0;
        Self { r, phi }
    }
    pub fn from_slope_intercept_form(k: f32, n: f32) -> Self {
        // y = kx + n
        let phi = k.atan();
        let r = n / phi.sin();
        Self { r, phi }
    }
    // pub fn from_standard_form(a: f32, b: f32, c: f32) -> Self {
    //     // ax + by = c
    // }
    pub fn to_slope_intercept_form(&self) -> (f32, f32) {
        // y = kx + n
        match self.phi.to_degrees() {
            0.0 => (self.r, std::f32::INFINITY),
            90.0 => (std::f32::INFINITY, self.r),
            180.0 => (-self.r, std::f32::INFINITY),
            270.0 => (std::f32::INFINITY, -self.r),
            _ => {
                let n = self.r * self.phi.sin();
                let k = n / (self.r * self.phi.cos());
                (k, n)
            }
        }
    }
    // pub fn to_standard_form(&self) -> (f32, f32, f32) {
    //     // ax + by = c
    // }
    // fn intersection(&self, other: Line) -> Option<Point> {
    //     None
    // }
}

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
