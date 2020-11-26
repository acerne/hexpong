use crate::geometry::point::Point;
use std::ops::{Add, Div, Mul, Neg, Sub};

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
    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }
    pub fn cross(self, other: Self) -> f32 {
        self.x * other.y - self.y * other.x
    }
    pub fn get_length(&self) -> f32 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }
    pub fn get_angle(&self) -> f32 {
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

impl Add<Vector> for Vector {
    type Output = Self;
    fn add(self, other: Vector) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<Point> for Vector {
    type Output = Self;
    fn add(self, other: Point) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<f32> for Vector {
    type Output = Self;
    fn add(self, other: f32) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
        }
    }
}

impl Sub<Vector> for Vector {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Sub<Point> for Vector {
    type Output = Self;
    fn sub(self, other: Point) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Sub<f32> for Vector {
    type Output = Self;
    fn sub(self, other: f32) -> Self {
        Self {
            x: self.x - other,
            y: self.y - other,
        }
    }
}

impl Mul<f32> for Vector {
    type Output = Self;
    fn mul(self, other: f32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Div<f32> for Vector {
    type Output = Self;
    fn div(self, other: f32) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl Neg for Vector {
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
