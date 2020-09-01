use crate::math::{Float, Vector3};
use rand_pcg::Pcg64Mcg;
use rand_distr::StandardNormal;
use rand::Rng;

#[derive(Clone)]
pub struct RandomGenerator {
    generator: Pcg64Mcg,
}

impl RandomGenerator {
    pub fn new() -> Self {
        Self {
            generator: Pcg64Mcg::new(0xCAFEF00DD15EA5E5),
        }
    }

    pub fn unit(&mut self) -> Float {
        self.generator.gen_range(0.0, 1.0)
    }

    pub fn range(&mut self, min: Float, max: Float) -> Float {
        let mut value = self.generator.gen_range(min, max);

        value
    }

    pub fn unit_sphere(&mut self) -> Vector3 {
        let mut vec = Vector3 {
            x: self.generator.gen_range(-1.0, 1.0),
            y: self.generator.gen_range(-1.0, 1.0),
            z: self.generator.gen_range(-1.0, 1.0),
        };

        while vec.len_squared() >= 1.0 {
            let x = self.generator.gen_range(-1.0, 1.0);
            let y = self.generator.gen_range(-1.0, 1.0);
            let z = self.generator.gen_range(-1.0, 1.0);

            vec = Vector3::new(x, y, z);
        }

        vec.normalize()
    }
}
