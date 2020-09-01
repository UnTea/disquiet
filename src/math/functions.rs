use crate::math::*;

pub fn min(a: Float, b: Float) -> Float {
    if a < b {
        a
    } else {
        b
    }
}

pub fn max(a: Float, b: Float) -> Float {
    if a > b {
        a
    } else {
        b
    }
}

pub fn clamp(value: Float, min: Float, max: Float) -> Float {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

pub fn modulo(n: Float, m: Float) -> Float {
    ((n % m) + m) % m
}