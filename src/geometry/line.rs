use crate::geometry::{Point, Vector};

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
            // TODO:: float_eq
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
