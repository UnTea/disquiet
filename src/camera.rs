use crate::math::*;
use crate::random::RandomGenerator;

pub struct Camera {
    origin: Vector3,
    top_left: Vector3,
    vertical: Vector3,
    horizontal: Vector3,
}

impl Camera {
    pub fn look_at(origin: Vector3, at: Vector3, up: Vector3, fov: Float, aspect_ratio: Float) -> Self {
        // Create orthonormal basis
        let forward = (at - origin).normalize();
        let right = up.cross(forward).normalize();
        let up = forward.cross(right).normalize();

        // Calculate film size
        let fov = fov.to_radians();
        let vertical_size = (fov / 2.0).tan();
        let vertical = -up * vertical_size;
        let horizontal = right * vertical_size * aspect_ratio;

        let top_left = origin + forward - vertical - horizontal;

        Self {
            origin,
            top_left,
            vertical: vertical * 2.0,
            horizontal: horizontal * 2.0,
        }
    }

    pub fn get_ray(&self, u: Float, v: Float, rng: &mut RandomGenerator) -> Ray {
        let direction = self.top_left + self.horizontal * u + self.vertical * v - self.origin;
        Ray {
            origin: self.origin,
            direction: direction.normalize(),
        }
    }
}
