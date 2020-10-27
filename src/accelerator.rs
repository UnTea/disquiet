use crate::math::Ray;
use crate::shape::{Shape, Hit};

pub mod kd;
pub mod linear;

pub use self::linear::ShapeVec;

pub trait Accelerator : Send + Sync {
    fn trace(&self, ray: &Ray) -> Option<Hit>;
}
