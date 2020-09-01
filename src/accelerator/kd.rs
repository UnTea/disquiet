use crate::math::*;
use crate::shape::Shape;
/*
pub struct KdTreeNode<'a> {
    bounding_box: Aabb,
    split_plane: Plane,
    left: Option<Box<KdTreeNode<'a>>>,
    right: Option<Box<KdTreeNode<'a>>>,
    children: Vec<&'a dyn Shape>,
}

/// Find sum surface areas for shapes above and below
fn surface_areas(axis: Axis, split: Float, shapes: &Vec<&dyn Shape>) -> (Float, Float) {
    let mut left = 0.0;
    let mut right = 0.0;

    for shape in shapes {
        if shape.bounding_box().center()[axis] < split {
            left += shape.surface_area();
        } else {
            right += shape.surface_area();
        }
    }

    (left, right)
}

/// Pick split plane with the best (smallest) Surface Area Heuristic.
/// Splitting is done by discretizing the space into fixed amount of split planes, so the end result
/// might not be *that* optimal.
fn pick_split(bounding_box: &Aabb, shapes: &Vec<&dyn Shape>) -> (Float, Axis) {
    const STEPS: usize = 32;
    const T_TRAVERSAL: Float = 1.0;
    const T_INTERSECT: Float = 2.0;

    // Converts normalized split point to world space coordinates
    let to_world_space = |split, axis| {
        let min = bounding_box.min()[axis];
        let max = bounding_box.max()[axis];
        min + split * (max - min)
    };

    // Calculate Surface Area Heuristic for given normalized split point and axis
    let sah = |split: Float, axis| {
        let (left_area, right_area) = surface_areas(axis, to_world_space(split, axis), shapes);
        T_TRAVERSAL + T_INTERSECT * (split * left_area + (1.0 - split) * right_area)
    };

    let pick_plane = |&axis| (0..=STEPS)
        // Convert step into normalized split point (range [0; 1])
        .map(|step| {
            let split = step as Float / STEPS as Float;
            (sah(split, axis), split, axis)
        })
        // Choose a point with minimal SAH
        .min_by(|&(sah_a, _, _), &(sah_b, _, _)| cmp_float(&sah_a, &sah_b))
        // Convert split point into world-space coordinate, and add split axis
        .map(|(sah, split, axis)| (sah, to_world_space(split, axis), axis))
        // kd-trees with zero elements are prohibited, so we can unwrap safely
        .unwrap();

    [Axis::X, Axis::Y, Axis::Z].iter()
        // Pick plane on a given axis
        .map(pick_plane)
        // Choose one with minimal SAH
        .min_by(|&(sah_a, _, _), &(sah_b, _, _)| cmp_float(&sah_a, &sah_b))
        // Drop SAH value
        .map(|(_, point, axis)| (point, axis))
        .unwrap()
}

fn split_shapes(
    shapes: Vec<&dyn Shape>,
    split: Float,
    axis: Axis,
) -> (Vec<&dyn Shape>, Vec<&dyn Shape>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for shape in shapes {
        let bounding_box = shape.bounding_box();

        if bounding_box.max()[axis] < split {
            left.push(shape);
        } else if bounding_box.min()[axis] >= split {
            right.push(shape);
        } else {
            left.push(shape);
            right.push(shape);
        }
    }

    (left, right)
}

impl<'a> KdTreeNode<'a> {
    pub fn build(shapes: Vec<&'a dyn Shape>) -> Self {
        // FIXME: make a special case for tree with zero children?
        assert!(!shapes.is_empty());

        const MAX_CHILDREN: usize = 8;

        let bounding_box = Aabb::new(Vector3::ZERO, Vector3::ZERO);
        let bounding_box = shapes
            .iter()
            .fold(bounding_box, |result, object| result.union(object.bounding_box()));

        let (split, axis) = pick_split(&bounding_box, &shapes);
        let split_plane = Plane {
            point: bounding_box.min() + axis.to_unit_vector(),
            normal: axis.to_unit_vector(),
        };

        let total_shapes = shapes.len();

        let (children, left, right) = if total_shapes >= MAX_CHILDREN {
            let (left_shapes, right_shapes) = split_shapes(shapes, split, axis);
            let left = Box::new(KdTreeNode::build(left_shapes));
            let right = Box::new(KdTreeNode::build(right_shapes));
            (Vec::new(), Some(left), Some(right))
        } else {
            (shapes, None, None)
        };

        Self {
            bounding_box,
            split_plane,
            left,
            right,
            children,
        }
    }

    fn traverse(&mut self, t_near: Float, t_far: Float, ray: &Ray) -> Option<&dyn Shape> {
        let is_left = self.split_plane.normal.dot(ray.direction) > 0.0;

    }
}

pub trait Accelerator {
    fn trace(&self, ray: &Ray) -> Option<&dyn Shape>;
}

impl<'a> Accelerator for KdTreeNode<'a> {
    fn trace(&self, ray: &Ray) -> Option<&dyn Shape> {
        self.traverse(ray)
    }
}
*/
