use crate::math::*;
use crate::scene::MaterialId;
use crate::shape::*;

pub struct Plane {
    pub point: Vector3,
    pub normal: Vector3,
    pub material: MaterialId,
}

impl Shape for Plane {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        let d = self.normal.dot(ray.direction);
        if d.abs() > EPSILON {
            let t = (self.point - ray.origin).dot(self.normal) / d;
            if t > EPSILON {
                Some(Hit::new(ray, t, self))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn normal_at(&self, point: Vector3) -> Vector3 {
        self.normal
    }

    fn material(&self) -> MaterialId {
        self.material
    }

    fn bounding_box(&self) -> Aabb {
        unimplemented!()
    }

    fn surface_area(&self) -> Float {
        unimplemented!()
    }
}
