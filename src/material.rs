use crate::color::Color3;
use crate::math::*;
use crate::random::RandomGenerator;

pub trait Material : Send + Sync {
    fn albedo(&self) -> Color3;
    fn next_ray_direction(&self, n: Vector3, wo: Vector3, rng: &mut RandomGenerator) -> Vector3;
    fn brdf(&self, n: Vector3, wi: Vector3, wo: Vector3) -> Color3;
    fn pdf(&self, n: Vector3, wi: Vector3) -> Float;
    fn emittance(&self) -> Color3;
}

pub struct Lambertian {
    pub color: Color3,
}

impl Material for Lambertian {
    fn albedo(&self) -> Color3 {
        self.color.clone()
    }

    fn next_ray_direction(&self, n: Vector3, wo: Vector3, rng: &mut RandomGenerator) -> Vector3 {
        /*let mut wi = rng.unit_sphere();
        if n.dot(wi) < 0.0 {
            wi = -wi;
        }
        wi*/

        let u = rng.range(0.0, 1.0);
        let v = rng.range(0.0, 1.0);

        let phi = 2.0 * PI * u;
        let cos_theta = 2.0 * v - 1.0;
        let f = (1.0 - cos_theta * cos_theta).sqrt();
        let sphere_point = Vector3::new(f * phi.cos(), f * phi.sin(), cos_theta);

        let result = (n + sphere_point).normalize();
        if result.x.is_nan() || result.y.is_nan() || result.z.is_nan() {
            println!("Nan");
        }

        result
    }

    fn brdf(&self, n: Vector3, wi: Vector3, wo: Vector3) -> Color3 {
        self.color.clone() / PI
    }

    fn pdf(&self, n: Vector3, wi: Vector3) -> Float {
        n.dot(wi) / PI
        // 1.0 / (2.0 * PI)
    }

    fn emittance(&self) -> Color3 {
        Color3::new(0.0, 0.0, 0.0)
    }
}

pub struct LightEmitter {
    pub color: Color3,
}

impl Material for LightEmitter {
    fn albedo(&self) -> Color3 {
        self.color
    }

    fn next_ray_direction(&self, n: Vector3, wo: Vector3, rng: &mut RandomGenerator) -> Vector3 {
        let mut wi = rng.unit_sphere();
        if n.dot(wi) < 0.0 {
            wi = -wi;
        }

        wi
    }

    fn brdf(&self, n: Vector3, wi: Vector3, wo: Vector3) -> Color3 {
        Color3::new(0.0, 0.0, 0.0)
    }

    fn pdf(&self, n: Vector3, wi: Vector3) -> Float {
        1.0
    }

    fn emittance(&self) -> Color3 {
        self.color
    }
}
