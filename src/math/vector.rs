use crate::math::*;
use crate::random::*;
use std::ops::{Add, Sub, Mul, Div, Index, IndexMut, Neg};

#[derive(Copy, Clone, Debug, Default)]
pub struct Vector3 {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

impl Vector3 {
    pub const ZERO: Vector3 = Vector3::new(0.0, 0.0, 0.0);
    pub const X: Vector3 = Vector3::new(1.0, 0.0, 0.0);
    pub const Y: Vector3 = Vector3::new(0.0, 1.0, 0.0);
    pub const Z: Vector3 = Vector3::new(0.0, 1.0, 1.0);

    pub const fn new(x: Float, y: Float, z: Float) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub fn dot(&self, rhs: Vector3) -> Float {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn normalize(&self) -> Vector3 {
        let len = self.len();
        *self / len
    }

    pub fn len_squared(&self) -> Float {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn len(&self) -> Float {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn min(&self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x.min(rhs.x),
            y: self.y.min(rhs.y),
            z: self.z.min(rhs.z),
        }
    }

    pub fn max(&self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x.max(rhs.x),
            y: self.y.max(rhs.y),
            z: self.z.max(rhs.z),
        }
    }

    pub fn clamp(&self, min: Vector3, max: Vector3) -> Vector3 {
        Vector3 {
            x: clamp(self.x, min.x, max.x),
            y: clamp(self.y, min.y, max.y),
            z: clamp(self.z, min.z, max.z),
        }
    }
}

impl Add for Vector3 {
    type Output = Vector3;
    fn add(self, rhs: Vector3) -> Vector3 {
        Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vector3 {
    type Output = Vector3;
    fn sub(self, rhs: Vector3) -> Vector3 {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<Float> for Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: Float) -> Vector3 {
        Vector3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Div<Float> for Vector3 {
    type Output = Vector3;
    fn div(self, rhs: Float) -> Vector3 {
        let reciprocal = 1.0 / rhs;
        Vector3::new(self.x * reciprocal, self.y * reciprocal, self.z * reciprocal)
    }
}

impl Neg for Vector3 {
    type Output = Vector3;
    fn neg(self) -> Vector3 {
        Vector3::new(-self.x, -self.y, -self.z)
    }
}

impl Index<Axis> for Vector3 {
    type Output = Float;
    fn index(&self, axis: Axis) -> &Float {
        match axis {
            Axis::X => &self.x,
            Axis::Y => &self.y,
            Axis::Z => &self.z,
        }
    }
}

impl IndexMut<Axis> for Vector3 {
    fn index_mut(&mut self, axis: Axis) -> &mut Float {
        match axis {
            Axis::X => &mut self.x,
            Axis::Y => &mut self.y,
            Axis::Z => &mut self.z,
        }
    }
}

pub struct Vector3x8 {
    pub x: Float8,
    pub y: Float8,
    pub z: Float8,
}

impl Vector3x8 {
    pub const ZERO: Vector3x8 = Vector3x8::new(Float8::ZERO, Float8::ZERO, Float8::ZERO);
    pub const X: Vector3x8 = Vector3x8::new(Float8::ONE, Float8::ZERO, Float8::ZERO);
    pub const Y: Vector3x8 = Vector3x8::new(Float8::ZERO, Float8::ONE, Float8::ZERO);
    pub const Z: Vector3x8 = Vector3x8::new(Float8::ZERO, Float8::ZERO, Float8::ONE);

    pub const fn new(x: Float8, y: Float8, z: Float8) -> Self {
        Self { x, y, z }
    }

    pub fn dot(&self, rhs: Vector3x8) -> Float8 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: Vector3x8) -> Vector3x8 {
        Vector3x8 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn normalize(self) -> Vector3x8 {
        let len = self.len();
        self / len
    }

    pub fn len_squared(&self) -> Float8 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn len(&self) -> Float8 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn min(&self, rhs: Vector3x8) -> Vector3x8 {
        Vector3x8 {
            x: self.x.min(rhs.x),
            y: self.y.min(rhs.y),
            z: self.z.min(rhs.z),
        }
    }

    pub fn max(&self, rhs: Self) -> Self {
        Vector3x8 {
            x: self.x.max(rhs.x),
            y: self.y.max(rhs.y),
            z: self.z.max(rhs.z),
        }
    }

    pub fn clamp(&self, min: Vector3x8, max: Vector3x8) -> Vector3x8 {
        Vector3x8 {
            x: self.x.clamp(min.x, max.x),
            y: self.y.clamp(min.y, max.y),
            z: self.z.clamp(min.z, max.z),
        }
    }
}

impl Add for Vector3x8 {
    type Output = Vector3x8;
    fn add(self, rhs: Vector3x8) -> Vector3x8 {
        Vector3x8::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vector3x8 {
    type Output = Vector3x8;
    fn sub(self, rhs: Vector3x8) -> Vector3x8 {
        Vector3x8::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<Float8> for Vector3x8 {
    type Output = Vector3x8;
    fn mul(self, rhs: Float8) -> Vector3x8 {
        Vector3x8::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Div<Float8> for Vector3x8 {
    type Output = Vector3x8;
    fn div(self, rhs: Float8) -> Vector3x8 {
        let reciprocal = rhs.recip();
        Vector3x8::new(self.x * reciprocal, self.y * reciprocal, self.z * reciprocal)
    }
}

impl Neg for Vector3x8 {
    type Output = Vector3x8;
    fn neg(self) -> Vector3x8 {
        Vector3x8::new(-self.x, -self.y, -self.z)
    }
}
