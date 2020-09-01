use crate::math::*;
use crate::scene::MaterialId;
use crate::shape::*;

pub struct Triangle {
    // Vertex positions
    pub a: Vector3,
    pub b: Vector3,
    pub c: Vector3,

    // Normals
    pub na: Vector3,
    pub nb: Vector3,
    pub nc: Vector3,

    pub material: MaterialId,
}

impl Shape for Triangle {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        // Möller–Trumbore intersection algorithm
        let edge1 = self.b - self.a;
        let edge2 = self.c - self.a;

        let pvec = ray.direction.cross(edge2);
        let det = edge1.dot(pvec);

        if det.abs() < EPSILON {
            return None;
        }

        let inv_det = 1.0 / det;
        let tvec = ray.origin - self.a;
        let u = tvec.dot(pvec) * inv_det;
        if u < 0.0 || u > 1.0 {
            return None;
        }

        let qvec = tvec.cross(edge1);
        let v = ray.direction.dot(qvec) * inv_det;
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = edge2.dot(qvec) * inv_det;
        if t < EPSILON {
            return None;
        }

        Some(Hit::new(ray, t, self))
    }

    fn normal_at(&self, point: Vector3) -> Vector3 {
        let p = point - self.a;
        let u = self.b - self.a;
        let v = self.c - self.a;

        // Orient normal towards the incoming ray direction
        if u.dot(p) > 0.0 {
            v.cross(u).normalize()
        } else {
            u.cross(v).normalize()
        }
    }

    fn material(&self) -> MaterialId {
        self.material
    }

    fn bounding_box(&self) -> Aabb {
        let min = self.a.min(self.b).min(self.c);
        let max = self.a.max(self.b).max(self.c);

        Aabb::new(min, max)
    }

    fn surface_area(&self) -> Float {
        let ab = self.b - self.a;
        let ac = self.c - self.a;
        ab.cross(ac).len() * 0.5
    }
}
