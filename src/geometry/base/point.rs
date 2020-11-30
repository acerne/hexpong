use crate::geometry::base::Vector;
use float_eq::FloatEq;

pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub fn zero() -> Self {
        Self::new(0.0, 0.0)
    }
    pub fn distance_to(&self, other: Point) -> f32 {
        ((other.x - self.x).powf(2.0) + (other.y - self.y).powf(2.0)).sqrt()
    }
}

impl Copy for Point {}

impl Clone for Point {
    fn clone(&self) -> Self {
        Self {
            x: self.x.clone(),
            y: self.y.clone(),
        }
    }
}

impl std::ops::Add<Point> for Point {
    type Output = Self;
    fn add(self, other: Point) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Add<Vector> for Point {
    type Output = Self;
    fn add(self, other: Vector) -> Self {
        Self {
            x: self.x + other.dx,
            y: self.y + other.dy,
        }
    }
}

impl std::ops::Add<f32> for Point {
    type Output = Self;
    fn add(self, other: f32) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
        }
    }
}

impl std::ops::Sub<Point> for Point {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Sub<Vector> for Point {
    type Output = Self;
    fn sub(self, other: Vector) -> Self {
        Self {
            x: self.x - other.dx,
            y: self.y - other.dy,
        }
    }
}

impl std::ops::Sub<f32> for Point {
    type Output = Self;
    fn sub(self, other: f32) -> Self {
        Self {
            x: self.x - other,
            y: self.y - other,
        }
    }
}

impl std::ops::Mul<f32> for Point {
    type Output = Self;
    fn mul(self, other: f32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl std::ops::Div<f32> for Point {
    type Output = Self;
    fn div(self, other: f32) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl FloatEq for Point {
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

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Point")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::base::Point;
    use float_eq::FloatEq;

    #[test]
    fn test_distance_to() {
        let point_a = Point::new(5.0, 5.0);
        let point_b = Point::new(-5.0, 5.0);
        let distance = point_a.distance_to(point_b);
        assert_eq!(distance, 10f32);
    }
}
