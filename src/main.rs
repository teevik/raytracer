mod camera;
mod data;
mod extensions;
mod materials;
mod shapes;
mod world;

use camera::Camera;
use data::Shape;
use indicatif::ParallelProgressIterator;
use materials::{dialectric::Dialectric, lambertian::Lambertian, metal::Metal};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use shapes::sphere::Sphere;
use std::{f32::consts::PI, fs, sync::Arc};
use vek::{
    geom::repr_simd::Ray,
    vec::repr_simd::{Vec2, Vec3},
};
use world::World;

fn ray_color(ray: Ray<f32>, depth_left: u32, world: &World) -> Vec3<f32> {
    if depth_left == 0 {
        return Vec3::zero();
    }

    // World hit
    let ray_hit = world.hit(ray, 0.001..f32::INFINITY);

    if let Some(ray_hit) = ray_hit {
        if let Some(scatter_result) = ray_hit.material.clone().scatter(ray, ray_hit) {
            return scatter_result.attenuation
                * ray_color(scatter_result.scattered, depth_left - 1, world);
        } else {
            return Vec3::zero();
        }
    }

    // Background gradient
    let unit_direction = ray.direction.normalized();
    let a = (unit_direction.y + 1.) / 2.;

    Vec3::broadcast(1. - a) + (a * Vec3::new(0.5, 0.7, 1.0))
}

fn main() {
    let image_size = Vec2::new(800, 400);

    // World
    let scene_1: World = {
        let material_ground = Arc::new(Lambertian {
            albedo: Vec3::new(0.8, 0.8, 0.),
        });

        let material_center = Arc::new(Lambertian {
            albedo: Vec3::new(0.1, 0.2, 0.5),
        });

        let material_left = Arc::new(Dialectric {
            refraction_index: 1.5,
        });

        let material_right = Arc::new(Metal {
            albedo: Vec3::new(0.8, 0.6, 0.2),
            fuzz: 0.1,
        });

        vec![
            Box::new(Sphere {
                center: Vec3::new(0., -100.5, -1.),
                radius: 100.,
                material: material_ground,
            }),
            Box::new(Sphere {
                center: Vec3::new(0., 0., -1.),
                radius: 0.5,
                material: material_center,
            }),
            Box::new(Sphere {
                center: Vec3::new(-1., 0., -1.),
                radius: 0.5,
                material: material_left,
            }),
            Box::new(Sphere {
                center: Vec3::new(1., 0., -1.),
                radius: 0.5,
                material: material_right,
            }),
        ]
    };

    let scene_2: World = {
        let material_left = Arc::new(Lambertian {
            albedo: Vec3::new(0., 0., 1.),
        });

        let material_right = Arc::new(Lambertian {
            albedo: Vec3::new(1., 0., 0.),
        });

        let radius = f32::cos(PI / 4.);

        vec![
            Box::new(Sphere {
                center: Vec3::new(-radius, 0., -1.),
                radius,
                material: material_left,
            }),
            Box::new(Sphere {
                center: Vec3::new(radius, 0., -1.),
                radius,
                material: material_right,
            }),
        ]
    };

    let world = scene_2;

    // Camera
    let camera = {
        let camera_center = Vec3::new(0., 0., 0.);
        let vertical_fov = f32::to_radians(90.);

        let focal_length = 1.;
        let samples_per_pixel = 10;

        Camera::new(
            camera_center,
            vertical_fov,
            image_size,
            focal_length,
            samples_per_pixel,
        )
    };

    let max_depth = 10;

    let mut image = String::new();
    image += &format!("P3\n{} {}\n255\n", image_size.x, image_size.y);

    let all_samples = camera.all_samples();

    let pixels = all_samples
        .into_par_iter()
        .progress()
        .map(move |samples| {
            let mut color = Vec3::zero();

            for pixel in samples {
                let ray = Ray::new(camera.camera_center, pixel);
                color += ray_color(ray, max_depth, &world);
            }

            color /= camera.samples_per_pixel as f32;
            color = color.map(|c| c.sqrt()); // map from linear to gamma 2

            let color = color.map(|c| (c * 255.).round() as u8);

            format!("{} {} {}\n", color.x, color.y, color.z)
        })
        .collect::<String>();

    image += &pixels;

    fs::write("image.ppm", image).unwrap();
}
