use glam::{Vec2, Vec3};

pub type FaceTriangle = Vec<Vec3>;

pub struct Face {
    triangle: FaceTriangle,
    normal: Vec3,
}

/// For generics on Vec2/3/4
pub trait VectorOps {
    fn cross(&self, other: &Self) -> Self;
    fn dot(&self, other: &Self) -> f32;
}

impl VectorOps for Vec2 {
    fn cross(&self, other: &Self) -> Self {
        self.cross(other)
    }

    fn dot(&self, other: &Self) -> f32 {
        self.dot(other)
    }
}

impl VectorOps for Vec3 {
    fn cross(&self, other: &Self) -> Self {
        self.cross(other)
    }

    fn dot(&self, other: &Self) -> f32 {
        self.dot(other)
    }
}

/// Clamp macro
#[macro_export]
macro_rules! sclamp {
    ($x:expr,$y:expr) => {
        if $x - $y < 0.0 {
            $x = 0.0;
        } else {
            $x -= $y;
        }
    };
}

#[macro_export]
macro_rules! clamp {
    ($x:expr,$y:expr) => {
        if $x - $y < 0.0 {
            0.0
        } else {
            $x - $y
        }
    };
}
