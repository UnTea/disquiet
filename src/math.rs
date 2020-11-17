pub mod aabb;
pub mod axis;
pub mod functions;
pub mod plane;
pub mod vector;

pub use self::aabb::*;
pub use self::axis::*;
pub use self::functions::*;
pub use self::plane::*;
pub use self::vector::*;
pub use std::f64::{EPSILON, MAX, consts::*};

pub type Float = f64;
pub type Float8 = wide::f32x8;

pub trait FloatExt : Sized {
    fn clamp(self, min: Self, max: Self) -> Self;
}

impl FloatExt for Float8 {
    fn clamp(self, min: Self, max: Self) -> Self {
        self.min(max).max(min)
    }
}

pub fn cmp_float(a: &Float, b: &Float) -> std::cmp::Ordering {
    if a < b {
        std::cmp::Ordering::Less
    } else if a > b {
        std::cmp::Ordering::Greater
    } else {
        std::cmp::Ordering::Equal
    }
}

#[derive(Clone, Debug)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn point_at(&self, t: Float) -> Vector3 {
        self.origin + self.direction * t
    }
}
