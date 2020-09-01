use crate::math::*;
use crate::scene::MaterialId;
use crate::shape::*;

pub struct Sphere {
    pub center: Vector3,
    pub radius: Float,
    pub material: MaterialId,
}

impl Shape for Sphere {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        let oc = ray.origin - self.center;
        let b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let h = b * b - c;
        if h < 0.0 {
            return None;
        }

        let h = h.sqrt();

        let t = -b - h;
        if t > EPSILON {
            return Some(Hit::new(ray, t, self));
        }

        let t = -b + h;

        if t > EPSILON {
            return Some(Hit::new(ray, t, self));
        }

        None
    }

    fn normal_at(&self, point: Vector3) -> Vector3 {
        (point - self.center).normalize()
    }

    fn material(&self) -> MaterialId {
        self.material
    }

    fn bounding_box(&self) -> Aabb {
        let min = Vector3 {
            x: self.center.x - self.radius,
            y: self.center.y - self.radius,
            z: self.center.z - self.radius,
        };

        let max = Vector3 {
            x: self.center.x + self.radius,
            y: self.center.y + self.radius,
            z: self.center.z + self.radius,
        };

        Aabb::new(min, max)
    }

    fn surface_area(&self) -> Float {
        4.0 * PI * self.radius * self.radius
    }
}
