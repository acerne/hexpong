use crate::geometry::Scale;

pub struct Size {
    pub w: f32,
    pub h: f32,
}

impl Copy for Size {}

impl Clone for Size {
    fn clone(&self) -> Self {
        Self {
            w: self.w.clone(),
            h: self.h.clone(),
        }
    }
}

impl std::ops::Mul<Scale> for Size {
    type Output = Self;
    fn mul(self, other: Scale) -> Self {
        Self {
            w: self.w * other.sx,
            h: self.h * other.sy,
        }
    }
}

impl std::ops::Mul<f32> for Size {
    type Output = Self;
    fn mul(self, other: f32) -> Self {
        Self {
            w: self.w * other,
            h: self.h * other,
        }
    }
}
