use crate::math::*;

pub struct Aabb {
    min: Vector3,
    max: Vector3,
}

impl Aabb {
    pub fn new(min: Vector3, max: Vector3) -> Aabb {
        assert!(min.x <= max.x && min.y <= max.y && min.z <= max.z);

        Aabb { min, max }
    }

    pub fn intersect(&self, rhs: &Aabb) -> bool {
        let x_intersection = self.max.x > rhs.min.x || rhs.max.x > self.min.x;
        let y_intersection = self.max.y > rhs.min.y || rhs.max.y > self.min.y;
        let z_intersection = self.max.z > rhs.min.z || rhs.max.z > self.min.z;
        x_intersection && y_intersection && z_intersection
    }

    pub fn intersect_ray(&self, ray: &Ray) -> bool {
        unimplemented!()
    }

    pub fn min(&self) -> Vector3 {
        self.min
    }

    pub fn max(&self) -> Vector3 {
        self.max
    }

    pub fn union(&self, rhs: Aabb) -> Aabb {
        let min = Vector3 {
            x: min(self.min.x, rhs.min.x),
            y: min(self.min.y, rhs.min.y),
            z: min(self.min.z, rhs.min.z),
        };

        let max = Vector3 {
            x: max(self.max.x, rhs.max.x),
            y: max(self.max.y, rhs.max.y),
            z: max(self.max.z, rhs.max.z),
        };

        Aabb::new(min, max)
    }

    pub fn center(&self) -> Vector3 {
        (self.min + self.max) / 2.0
    }
}
