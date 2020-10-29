use crate::camera::*;
use crate::color::*;
use crate::image::*;
use crate::material::*;
use crate::shape::*;
use crate::math::Vector3;

pub struct Scene {
    materials: Vec<Box<dyn Material>>,
    shapes: Vec<Box<dyn Shape>>,
    pub world_color: Color3,
    pub sky: Option<Image>,
    pub camera: Camera,
}

pub type MaterialId = usize;

impl Scene {
    pub fn new() -> Self {
        Self {
            materials: Vec::new(),
            shapes: Vec::new(),
            world_color: Color3::new(0.0, 0.0, 0.0),
            sky: None,
            camera: Camera::look_at(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0), Vector3::Y, 70.0, 16.0 / 9.0),
        }
    }

    pub fn add_shape<S: Shape + 'static>(&mut self, s: S) {
        self.shapes.push(Box::new(s));
    }

    pub fn add_material<M: Material + 'static>(&mut self, m: M) -> MaterialId {
        self.materials.push(Box::new(m));
        self.materials.len() - 1
    }

    pub fn get_material(&self, material_id: MaterialId) -> &dyn Material {
        self.materials[material_id].as_ref()
    }

    pub fn shapes(&self) -> impl Iterator<Item = &dyn Shape> {
        self.shapes.iter().map(|o| o.as_ref())
    }
}
