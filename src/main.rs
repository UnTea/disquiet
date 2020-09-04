#![allow(unused_imports)]
#![allow(dead_code)]

mod accelerator;
mod camera;
mod color;
mod error;
mod image;
mod integrator;
mod io;
mod material;
mod math;
mod random;
mod scene;
mod shape;

use std::fs::File;

use crate::accelerator::*;
use crate::camera::*;
use crate::color::*;
use crate::image::*;
use crate::integrator::*;
use crate::material::*;
use crate::math::*;
use crate::random::*;
use crate::scene::*;
use crate::shape::*;
use crossbeam_deque::{Injector, Steal};

fn build_scene() -> Scene {
    let origin = Vector3::new(-1.5, 9.0, 0.1);
    let at = Vector3::new(-1.5, 0.0, 0.0);
    let up = Vector3::new(0.0, 1.0, 0.0);
    let fov = 90.0;
    let aspect_ratio = 2.0 / 1.0;
    let camera = Camera::look_at(origin, at, up, fov, aspect_ratio);

    let mut scene = Scene::new(camera);

    // scene.sky = Some(io::load("spiaggia_di_mondello_4k.hdr").unwrap());
    // scene.sky = Some(io::load("blaubeuren_night_4k.hdr").unwrap());
    // scene.sky = Some(io::load("blaubeuren_night_4k.hdr").unwrap());

    let red = scene.add_material(Lambertian {
        color: Color3::new(1.0, 0.1, 0.1),
    });

    let green = scene.add_material(Lambertian {
        color: Color3::new(0.1, 0.9, 0.1),
    });

    let white = scene.add_material(Lambertian {
        color: Color3::new(1.0, 1.0, 1.0),
    });

    let yellow = scene.add_material(Lambertian {
        color: Color3::new(0.9, 0.7, 0.1),
    });

    let light = scene.add_material(LightEmitter {
        color: Color3::new(10.0, 10.0, 10.0),
    });

    scene.add_shape(Sphere {
        center: Vector3::new(0.5, -0.2, -0.5),
        radius: 0.3,
        material: green,
    });
    scene.add_shape(Sphere {
        center: Vector3::new(0.0, 0.0, 3.0),
        radius: 0.5,
        material: red,
    });
    scene.add_shape(Sphere {
        center: Vector3::new(-1.5, 0.0, 0.0),
        radius: 0.5,
        material: white,
    });

    scene.add_shape(crate::shape::plane::Plane {
        point: Vector3::new(0.0, -0.5, 0.0),
        normal: Vector3::new(0.0, 1.0, 0.0),
        material: white,
    });

    scene.add_shape(Sphere {
        center: Vector3::new(-3.0, 2.0, -3.0),
        radius: 0.5,
        material: light,
    });
    scene.add_shape(Sphere {
        center: Vector3::new(3.0, 2.0, -3.0),
        radius: 0.5,
        material: light,
    });
    scene.add_shape(Sphere {
        center: Vector3::new(3.0, 2.0, 3.0),
        radius: 0.5,
        material: light,
    });
    scene.add_shape(Sphere {
        center: Vector3::new(-3.0, 2.0, 3.0),
        radius: 0.5,
        material: light,
    });
/*
    let mut triangle = Triangle {
        a: Vector3::new(2.4, 2.0, -0.1),
        b: Vector3::new(2.4, 1.5, 0.4),
        c: Vector3::new(2.0, 2.2, 0.4),
        na: Vector3::new(0.0, 1.0, 0.0),
        nb: Vector3::new(0.0, 1.0, 0.0),
        nc: Vector3::new(0.0, 1.0, 0.0),
        material: yellow,
    };
    // triangle.recalculate_normals();
    scene.add_shape(triangle);*/

    scene
}

pub struct RendererInput<'a, A: Accelerator> {
    scene: &'a Scene,
    accel: &'a A,
    sample_count: usize,
    width: usize,
    height: usize,
    tile_size: usize,
    thread_count: usize,
}

pub struct TileInfo {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

#[derive(Clone)]
pub struct Renderer<I: Integrator> {
    integrator: I,
    rng: RandomGenerator,
}

impl<I: Integrator> Renderer<I> {
    fn new(integrator: I) -> Self {
        Self {
            integrator,
            rng: RandomGenerator::new(),
        }
    }

    unsafe fn render_tile<A: Accelerator>(&mut self, input: &RendererInput<A>, image: &RgbaImage, tile: TileInfo) {
        for y in tile.y..tile.y+tile.height {
            for x in tile.x..tile.x+tile.width {
                let width = image.width() as Float;
                let height = image.height() as Float;

                let mut color = Color3::new(0.0, 0.0, 0.0);

                for _ in 0..input.sample_count {
                    let jitter_u = self.rng.unit();
                    let jitter_v = self.rng.unit();

                    let u = (x as Float + jitter_u) / width;
                    let v = (y as Float + jitter_v) / height;

                    let ray = input.scene.camera.get_ray(u, v, &mut self.rng);
                    color = color + self.integrator.integrate(input.scene, &ray, input.accel).color;
                }

                image.set_pixel_unsafe(x, y, (color / input.sample_count as f64).into());
            }
        }
    }
}

fn render<I: Integrator + Clone, A: Accelerator>(integrator: I, input: RendererInput<A>) -> RgbaImage {
    let image = RgbaImage::new(input.width, input.height);

    let queue = Injector::new();

    let tiles_x = input.width / input.tile_size + if input.width % input.tile_size == 0 { 0 } else { 1 };
    let tiles_y = input.height / input.tile_size + if input.height % input.tile_size == 0 { 0 } else { 1 };

    for tile_y in 0..tiles_y {
        for tile_x in 0..tiles_x {
            let x = tile_x * input.tile_size;
            let y = tile_y * input.tile_size;
            let width = input.tile_size.min(input.width - x);
            let height = input.tile_size.min(input.height - y);

            queue.push(TileInfo { x, y, width, height });
        }
    }

    let mut renderers = std::iter::repeat(Renderer::new(integrator))
        .take(input.thread_count)
        .collect::<Vec<_>>();

    crossbeam_utils::thread::scope(|scope| for _ in 0..input.thread_count {
        let ref queue = queue;
        let ref image = image;
        let ref input = input;
        let mut renderer = renderers.pop().unwrap();
        scope.spawn(move |_| {
            while let Steal::Success(tile) = queue.steal() {
                unsafe {
                    renderer.render_tile(&input, &image, tile);
                }
            }
            renderer
        });
    }).unwrap();

    image
}

fn main() {
    let path = if let Some(path) = std::env::args().nth(1) {
        path
    } else {
        eprintln!("usage: disquiet <output.png>");
        return;
    };

    println!("output path: {}", path);

    let scene = build_scene();
    let accel = LinearAccelerator::new(&scene);

    let integrator = PathTracer::new(4);
    let input = RendererInput {
        scene: &scene,
        accel: &accel,
        sample_count: 512,
        width: 1024,
        height: 512,
        tile_size: 64,
        thread_count: 16,
    };

    let image = render(integrator, input);

    println!("saving");
    image.save("test.png").unwrap();
}
