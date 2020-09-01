use crate::accelerator::Accelerator;
use crate::color::Color3;
use crate::integrator::{Integrator, Output};
use crate::math::*;
use crate::scene::Scene;
use crate::shape::Shape;

#[derive(Clone)]
pub struct PrimaryRayIntegrator;

impl PrimaryRayIntegrator {
    pub fn new() -> Self {
        Self
    }
}

impl Integrator for PrimaryRayIntegrator {
    fn integrate<A: Accelerator>(&mut self, scene: &Scene, ray: &Ray, accel: &A) -> Output {
        let hit = accel.trace(ray);
        let hit = if let Some(hit) = hit {
            hit
        } else {
            return Output::new(scene.world_color, Vector3::new(-1.0, -1.0, -1.0));
        };

        let albedo = scene.get_material(hit.shape.material()).albedo();
        let normal = hit.shape.normal_at(hit.point());

        Output::new(albedo, normal)
    }
}
