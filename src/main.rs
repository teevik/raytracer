mod camera;
mod data;
mod extensions;
mod materials;
mod scenes;
mod sphere;
mod world;

use camera::Camera;
use indicatif::ParallelProgressIterator;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use scenes::scene_1::scene_1;
use std::fs;
use vek::{Ray, Rgb, Vec2, Vec3};
use world::World;

fn ray_color(ray: Ray<f32>, depth_left: u32, world: &World) -> Rgb<f32> {
    if depth_left == 0 {
        return Rgb::black();
    }

    // World hit
    let ray_hit = world.raytrace(ray, 0.001..f32::INFINITY);

    if let Some(ray_hit) = ray_hit {
        if let Some(scatter_result) = ray_hit.material.scatter(ray, ray_hit) {
            return scatter_result.attenuation
                * ray_color(scatter_result.scattered, depth_left - 1, world);
        } else {
            return Rgb::black();
        }
    }

    // Background gradient
    let unit_direction = ray.direction.normalized();
    let a = (unit_direction.y + 1.) / 2.;

    Rgb::broadcast(1. - a) + (a * Rgb::new(0.5, 0.7, 1.0))
}

fn main() {
    let image_size = Vec2::new(800, 400);

    // World
    let world = scene_1();

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
