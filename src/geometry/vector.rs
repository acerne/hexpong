use crate::geometry::Point;
use float_eq::FloatEq;

pub struct Vector {
    pub dx: f32,
    pub dy: f32,
}

impl Vector {
    pub fn new(dx: f32, dy: f32) -> Self {
        Self { dx, dy }
    }
    pub fn zero() -> Self {
        Self::new(0.0, 0.0)
    }
    pub fn from_points(point_a: Point, point_b: Point) -> Self {
        let diff = point_b - point_a;
        Self {
            dx: diff.x,
            dy: diff.y,
        }
    }
    pub fn normalize(&mut self) {
        let len = self.get_length();
        self.dx = self.dx / len;
        self.dy = self.dy / len;
    }
    pub fn rotate(&mut self, phi: f32) {
        let x1 = self.dx;
        let y1 = self.dy;
        self.dx = x1 * phi.cos() - y1 * phi.sin();
        self.dy = x1 * phi.sin() + y1 * phi.cos();
    }
    pub fn dot(self, other: Self) -> f32 {
        self.dx * other.dx + self.dy * other.dy
    }
    pub fn cross(self, other: Self) -> f32 {
        self.dx * other.dy - self.dy * other.dx
    }
    pub fn get_length(&self) -> f32 {
        (self.dx.powf(2.0) + self.dy.powf(2.0)).sqrt()
    }
    pub fn get_orientation(&self) -> f32 {
        self.dy.atan2(self.dx)
    }
}

impl Copy for Vector {}

impl Clone for Vector {
    fn clone(&self) -> Self {
        Self {
            dx: self.dx.clone(),
            dy: self.dy.clone(),
        }
    }
}

impl std::ops::Add<Vector> for Vector {
    type Output = Self;
    fn add(self, other: Vector) -> Self {
        Self {
            dx: self.dx + other.dx,
            dy: self.dy + other.dy,
        }
    }
}

impl std::ops::Add<f32> for Vector {
    type Output = Self;
    fn add(self, other: f32) -> Self {
        Self {
            dx: self.dx + other,
            dy: self.dy + other,
        }
    }
}

impl std::ops::Sub<Vector> for Vector {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            dx: self.dx - other.dx,
            dy: self.dy - other.dy,
        }
    }
}

impl std::ops::Sub<f32> for Vector {
    type Output = Self;
    fn sub(self, other: f32) -> Self {
        Self {
            dx: self.dx - other,
            dy: self.dy - other,
        }
    }
}

impl std::ops::Mul<f32> for Vector {
    type Output = Self;
    fn mul(self, other: f32) -> Self {
        Self {
            dx: self.dx * other,
            dy: self.dy * other,
        }
    }
}

impl std::ops::Div<f32> for Vector {
    type Output = Self;
    fn div(self, other: f32) -> Self {
        Self {
            dx: self.dx / other,
            dy: self.dy / other,
        }
    }
}

impl std::ops::Neg for Vector {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            dx: -self.dx,
            dy: -self.dy,
        }
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.dx == other.dx && self.dy == other.dy
    }
}

impl FloatEq for Vector {
    type Epsilon = f32;

    fn eq_abs(&self, other: &Self, max_diff: &f32) -> bool {
        self.dx.eq_abs(&other.dx, max_diff) && self.dy.eq_abs(&other.dy, max_diff)
    }

    fn eq_rmax(&self, other: &Self, max_diff: &f32) -> bool {
        self.dx.eq_rmax(&other.dx, max_diff) && self.dy.eq_rmax(&other.dy, max_diff)
    }

    fn eq_rmin(&self, other: &Self, max_diff: &f32) -> bool {
        self.dx.eq_rmin(&other.dx, max_diff) && self.dy.eq_rmin(&other.dy, max_diff)
    }

    fn eq_r1st(&self, other: &Self, max_diff: &f32) -> bool {
        self.dx.eq_r1st(&other.dx, max_diff) && self.dy.eq_r1st(&other.dy, max_diff)
    }

    fn eq_r2nd(&self, other: &Self, max_diff: &f32) -> bool {
        self.dx.eq_r2nd(&other.dx, max_diff) && self.dy.eq_r2nd(&other.dy, max_diff)
    }

    fn eq_ulps(&self, other: &Self, max_diff: &float_eq::UlpsEpsilon<f32>) -> bool {
        self.dx.eq_ulps(&other.dx, max_diff) && self.dy.eq_ulps(&other.dy, max_diff)
    }
}

impl std::fmt::Debug for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Point")
            .field("dx", &self.dx)
            .field("dy", &self.dy)
            .finish()
    }
}

impl std::fmt::Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.dx, self.dy)
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::Vector;
    use float_eq::FloatEq;

    #[test]
    fn test_normalize() {
        let mut vector_a = Vector::new(5.0, -5.0);
        vector_a.normalize();
        let vector_b = Vector::new(1.0 / 2f32.sqrt(), -1.0 / 2f32.sqrt());
        assert!(
            vector_a.eq_abs(&vector_b, &10e-6),
            "{} == {}",
            vector_a,
            vector_b
        );
    }
    #[test]
    fn test_rotate() {
        let mut vector_a = Vector::new(1.0, 1.0);
        vector_a.rotate(90f32.to_radians());
        let vector_b = Vector::new(-1.0, 1.0);
        assert!(
            vector_a.eq_abs(&vector_b, &10e-6),
            "{} == {}",
            vector_a,
            vector_b
        );
    }
    #[test]
    fn test_dot() {
        let vector_a = Vector::new(5.0, 3.0);
        let vector_b = Vector::new(-1.0, 5.0);
        let result = vector_a.dot(vector_b);
        assert_eq!(result, 10f32);
    }
    #[test]
    fn test_cross() {
        let vector_a = Vector::new(5.0, 3.0);
        let vector_b = Vector::new(-1.0, 5.0);
        let result = vector_a.cross(vector_b);
        assert_eq!(result, 28f32);
    }
    #[test]
    fn test_length() {
        let vector_a = Vector::new(1.0, -1.0);
        let result = vector_a.get_length();
        assert_eq!(result, 2f32.sqrt());
    }
    #[test]
    fn test_orientation() {
        let vector_a = Vector::new(1.0, -1.0);
        let result = vector_a.get_orientation();
        assert_eq!(result, -45f32.to_radians());
    }
}
