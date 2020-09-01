use crate::accelerator::*;
use crate::color::Color3;
use crate::integrator::{Integrator, Output};
use crate::math::*;
use crate::random::*;
use crate::scene::*;

#[derive(Clone)]
pub struct PathTracer {
    bounces: usize,
    rng: RandomGenerator,
}

impl PathTracer {
    pub fn new(bounces: usize) -> Self {
        PathTracer {
            bounces,
            rng: RandomGenerator::new(),
        }
    }

    pub fn trace<A: Accelerator>(&mut self, scene: &Scene, ray: &Ray, accel: &A, bounce: usize) -> Output {
        if bounce >= self.bounces {
            return Output::new(Color3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0));
        }
        let hit = accel.trace(ray);
        let hit = if let Some(hit) = hit {
            hit
        } else {
            return if let Some(image) = &scene.sky {
                let phi = ray.direction.z.atan2(ray.direction.x);
                let theta = ray.direction.y.atan2((ray.direction.x * ray.direction.x + ray.direction.z * ray.direction.z).sqrt());
                let color = image.get_pixel_spherical(phi, theta);
                Output::new(color, Vector3::new(0.0, 0.0, 0.0))
            } else {
                Output::new(scene.world_color, Vector3::new(0.0, 0.0, 0.0))
            }
        };

        let point = hit.point();
        let normal = hit.shape.normal_at(point);
        let material = scene.get_material(hit.shape.material());

        let wo = ray.direction;
        let wi = material.next_ray_direction(normal, wo, &mut self.rng);

        let next_ray = Ray {
            origin: point,
            direction: wi,
        };

        let next_color = self.trace(scene, &next_ray, accel, bounce + 1).color;

        let color = material.brdf(normal, wi, wo) * wi.dot(normal);
        let pdf = material.pdf(normal, wi);

        let color = /*if pdf > 0.0 {*/
            material.emittance() + color / pdf * next_color;
        /*} else {
            material.emittance()
        };*/

        Output::new(color, normal)
    }
}

impl Integrator for PathTracer {
    fn integrate<A: Accelerator>(&mut self, scene: &Scene, ray: &Ray, accel: &A) -> Output {
        self.trace(scene, ray, accel, 0)
    }
}
