use crate::math::*;
use crate::shape::*;

pub struct Plane {
    pub point: Vector3,
    pub normal: Vector3,
}

impl Plane {
    pub fn new(point: Vector3, normal: Vector3) -> Plane {
        Plane {
            point,
            normal,
        }
    }

    pub fn intersect_ray(&self, ray: &Ray) -> Option<Float> {
        let d = self.point.dot(ray.direction);
        if d.abs() > EPSILON {
            let t = (self.point - ray.origin).dot(self.normal) / d;
            if t >= EPSILON {
                Some(t)
            } else {
                None
            }
        } else {
            None
        }
    }
}
