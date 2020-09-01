use crate::math::*;
use std::ops::{Add, Sub, Mul, Div};

const GAMMA: Float = 2.2;

pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Rgba {
    pub fn rgb(r: u8, g: u8, b: u8) -> Rgba {
        Rgba { r, g, b, a: 255 }
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Rgba {
        Rgba { r, g, b, a }
    }
}


pub trait Color {
    type Component;
    const COMPONENT_COUNT: usize;
    fn load(data: &[Self::Component]) -> Self;
    fn store(&self, data: &mut [Self::Component]);
}

#[derive(Clone, Copy)]
pub struct Color3 {
    pub r: Float,
    pub g: Float,
    pub b: Float,
}

impl Color3 {
    pub fn new(r: Float, g: Float, b: Float) -> Self {
        Color3 { r, g, b }
    }

    pub fn clamp(&self, min: Color3, max: Color3) -> Color3 {
        Color3 {
            r: clamp(self.r, min.r, max.r),
            g: clamp(self.g, min.g, max.g),
            b: clamp(self.b, min.b, max.b),
        }
    }
}

fn srgb_to_linear(component: Float) -> Float {
    component.powf(2.2)
}

fn linear_to_srgb(component: Float) -> Float {
    component.powf(1.0 / 2.2)
}

fn aces(x: Color3) -> Color3 {
    let a = 2.51;
    let b = Color3::new(0.03, 0.03, 0.03);
    let c = 2.43;
    let d = Color3::new(0.59, 0.59, 0.59);
    let e = Color3::new(0.14, 0.14, 0.14);
    ((x*(x*a+b))/(x*(x*c+d)+e)).clamp(Color3::new(0.0, 0.0, 0.0), Color3::new(1.0, 1.0, 1.0))
}

impl From<Vector3> for Color3 {
    fn from(n: Vector3) -> Color3 {
        Color3 {
            r: n.x * 0.5 + 0.5,
            g: n.y * 0.5 + 0.5,
            b: n.z * 0.5 + 0.5,
        }
    }
}

impl From<Rgba> for Color3 {
    fn from(c: Rgba) -> Color3 {
        Color3 {
            r: srgb_to_linear(c.r as Float / 255.0),
            g: srgb_to_linear(c.g as Float / 255.0),
            b: srgb_to_linear(c.b as Float / 255.0),
        }
    }
}

impl Into<Rgba> for Color3 {
    fn into(self) -> Rgba {
        let c = aces(self);
        Rgba {
            r: (linear_to_srgb(c.r) * 255.0) as u8,
            g: (linear_to_srgb(c.g) * 255.0) as u8,
            b: (linear_to_srgb(c.b) * 255.0) as u8,
            a: 255,
        }
    }
}

impl Add for Color3 {
    type Output = Self;
    fn add(self, rhs: Color3) -> Color3 {
        Color3 {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl Sub for Color3 {
    type Output = Self;
    fn sub(self, rhs: Color3) -> Color3 {
        Color3 {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        }
    }
}

impl Mul for Color3 {
    type Output = Self;
    fn mul(self, rhs: Color3) -> Color3 {
        Color3 {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl Div for Color3 {
    type Output = Self;
    fn div(self, rhs: Color3) -> Color3 {
        Color3 {
            r: self.r / rhs.r,
            g: self.g / rhs.g,
            b: self.b / rhs.b,
        }
    }
}

impl Mul<Float> for Color3 {
    type Output = Self;
    fn mul(self, rhs: Float) -> Color3 {
        Color3 {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl Div<Float> for Color3 {
    type Output = Self;
    fn div(self, rhs: Float) -> Color3 {
        let reciprocal = 1.0 / rhs;
        Color3 {
            r: self.r * reciprocal,
            g: self.g * reciprocal,
            b: self.b * reciprocal,
        }
    }
}
