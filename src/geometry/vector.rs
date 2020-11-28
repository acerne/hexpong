use crate::geometry::point::Point;

pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub fn zero() -> Self {
        Self::new(0.0, 0.0)
    }
    pub fn normalize(&mut self) {
        let len = self.get_length();
        self.x = self.x / len;
        self.y = self.y / len;
    }
    pub fn rotate(&mut self, phi: f32) {
        let x1 = self.x;
        let y1 = self.y;
        self.x = x1 * phi.cos() - y1 * phi.sin();
        self.y = x1 * phi.sin() + y1 * phi.cos();
    }
    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }
    pub fn cross(self, other: Self) -> f32 {
        self.x * other.y - self.y * other.x
    }
    pub fn get_length(&self) -> f32 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }
    pub fn get_orientation(&self) -> f32 {
        self.y.atan2(self.x)
    }
}

impl Copy for Vector {}

impl Clone for Vector {
    fn clone(&self) -> Self {
        Self {
            x: self.x.clone(),
            y: self.y.clone(),
        }
    }
}

impl std::ops::Add<Vector> for Vector {
    type Output = Self;
    fn add(self, other: Vector) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Add<f32> for Vector {
    type Output = Self;
    fn add(self, other: f32) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
        }
    }
}

impl std::ops::Sub<Vector> for Vector {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Sub<f32> for Vector {
    type Output = Self;
    fn sub(self, other: f32) -> Self {
        Self {
            x: self.x - other,
            y: self.y - other,
        }
    }
}

impl std::ops::Mul<f32> for Vector {
    type Output = Self;
    fn mul(self, other: f32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl std::ops::Div<f32> for Vector {
    type Output = Self;
    fn div(self, other: f32) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl std::ops::Neg for Vector {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl float_eq::FloatEq for Vector {
    type Epsilon = f32;

    fn eq_abs(&self, other: &Self, max_diff: &f32) -> bool {
        self.x.eq_abs(&other.x, max_diff) && self.y.eq_abs(&other.y, max_diff)
    }

    fn eq_rmax(&self, other: &Self, max_diff: &f32) -> bool {
        self.x.eq_rmax(&other.x, max_diff) && self.y.eq_rmax(&other.y, max_diff)
    }

    fn eq_rmin(&self, other: &Self, max_diff: &f32) -> bool {
        self.x.eq_rmin(&other.x, max_diff) && self.y.eq_rmin(&other.y, max_diff)
    }

    fn eq_r1st(&self, other: &Self, max_diff: &f32) -> bool {
        self.x.eq_r1st(&other.x, max_diff) && self.y.eq_r1st(&other.y, max_diff)
    }

    fn eq_r2nd(&self, other: &Self, max_diff: &f32) -> bool {
        self.x.eq_r2nd(&other.x, max_diff) && self.y.eq_r2nd(&other.y, max_diff)
    }

    fn eq_ulps(&self, other: &Self, max_diff: &float_eq::UlpsEpsilon<f32>) -> bool {
        self.x.eq_ulps(&other.x, max_diff) && self.y.eq_ulps(&other.y, max_diff)
    }
}

impl std::fmt::Debug for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Point")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

impl std::fmt::Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::vector::Vector;
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
