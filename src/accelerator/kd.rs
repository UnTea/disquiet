use crate::accelerator::Accelerator;
use crate::math::*;
use crate::scene::*;
use crate::shape::{Shape, Hit};

const LEAF_SIZE: usize = 8;

pub struct Leaf<'a> {
    shapes: Vec<&'a dyn Shape>,
}

pub enum Node<'a> {
    Leaf {
        bounding_box: Aabb,
        shapes: Vec<&'a dyn Shape>
    },
    Branch {
        bounding_box: Aabb,
        left: Box<Node<'a>>,
        right: Box<Node<'a>>,
    },
}

impl<'a> Node<'a> {
    pub fn new(shapes: Vec<&'a dyn Shape>) -> Self {
        let bounding_box = shapes.iter()
            .nth(0)
            .map(|shape| shape.bounding_box())
            .unwrap_or_default();
        let bounding_box = shapes.iter()
            .fold(bounding_box, |aabb, shape| shape.bounding_box().extend(aabb));

        for axis in &[Axis::X, Axis::Y, Axis::Z] {
            
        }

        unimplemented!()
    }
}

pub struct KdTree<'a> {
    root: Node<'a>,
}

impl<'a> KdTree<'a> {
    pub fn new(scene: &'a Scene) -> Self {
        Self {
            root: Node::new(scene.shapes().collect())
        }
    }
}

impl Accelerator for KdTree<'_> {
    fn trace(&self, ray: &Ray) -> Option<Hit> {
        unimplemented!()
    }
}
