pub mod path_tracer;
pub mod primary_ray;

pub use path_tracer::*;
pub use primary_ray::*;

use crate::accelerator::Accelerator;
use crate::math::{Ray, Vector3};
use crate::scene::Scene;
use crate::shape::Shape;
use crate::color::Color3;

pub struct Output {
    pub color: Color3,
    pub normal: Vector3,
}

impl Output {
    pub fn new(color: Color3, normal: Vector3) -> Self {
        Self { color, normal }
    }
}

pub trait Integrator : Send + Sync + Clone {
    fn integrate<A: Accelerator>(&mut self, scene: &Scene, ray: &Ray, accel: &A) -> Output;
}
