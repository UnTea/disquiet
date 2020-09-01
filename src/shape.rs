pub mod plane;
pub mod triangle;
pub mod sphere;

// pub use self::plane::*;
pub use self::triangle::*;
pub use self::sphere::*;

use crate::math::*;
use crate::scene::MaterialId;

pub trait Shape : Send + Sync {
    fn hit(&self, ray: &Ray) -> Option<Hit>;
    fn normal_at(&self, point: Vector3) -> Vector3;
    fn material(&self) -> MaterialId;
    fn bounding_box(&self) -> Aabb;
    fn surface_area(&self) -> Float;
}

pub struct Hit<'shapes> {
    pub ray: Ray,
    pub t: Float,
    pub shape: &'shapes dyn Shape,
}

impl<'shapes> Hit<'shapes> {
    pub fn new(ray: &Ray, t: Float, shape: &'shapes dyn Shape) -> Hit<'shapes> {
        Hit {
            ray: ray.clone(),
            t,
            shape,
        }
    }

    pub fn point(&self) -> Vector3 {
        self.ray.point_at(self.t)
    }
}
