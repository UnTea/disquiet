use crate::accelerator::Accelerator;
use crate::math::*;
use crate::scene::*;
use crate::shape::{Shape, Hit};

pub struct LinearAccelerator<'a> {
    shapes: Vec<&'a dyn Shape>,
}

impl<'a> LinearAccelerator<'a> {
    pub fn new(scene: &'a Scene) -> Self {
        Self {
            shapes: scene.shapes().collect(),
        }
    }
}

impl Accelerator for LinearAccelerator<'_> {
    fn trace(&self, ray: &Ray) -> Option<Hit> {
        self.shapes.iter()
            .fold((None, MAX), |(closest_hit, t), shape| {
                if let Some(hit) = shape.hit(ray) {
                    if hit.t < t {
                        let new_t = hit.t;
                        return (Some(hit), new_t);
                    }
                }
                (closest_hit, t)
            }).0
    }
}
