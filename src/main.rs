mod camera;
mod data;
mod extensions;
mod materials;
mod scenes;
mod sphere;
mod world;

use crate::scenes::scene_3::scene_3;
use camera::Camera;
use indicatif::ParallelProgressIterator;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{fs, time::Instant};
use vek::{Ray, Rgb, Vec2, Vec3};
use world::World;

fn ray_color(ray: Ray<f32>, depth_left: u32, world: &World, rng: &mut impl Rng) -> Rgb<f32> {
    if depth_left == 0 {
        return Rgb::black();
    }

    // World hit
    let ray_hit = world.raytrace(ray, 0.001..f32::INFINITY);

    if let Some(ray_hit) = ray_hit {
        if let Some(scatter_result) = ray_hit.material.scatter(ray, ray_hit, rng) {
            return scatter_result.attenuation
                * ray_color(scatter_result.scattered, depth_left - 1, world, rng);
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
    let world = scene_3();

    // Camera
    let camera = {
        let camera_position = Vec3::new(13., 2., 3.);
        let camera_target = Vec3::new(0., 0., 0.);
        let camera_up = Vec3::new(0., 1., 0.);

        let defocus_angle = f32::to_radians(0.6);
        let focus_distance = 10.;
        let vertical_fov = f32::to_radians(20.);

        let samples_per_pixel = 10;

        Camera::new(
            camera_position,
            camera_target,
            camera_up,
            defocus_angle,
            focus_distance,
            vertical_fov,
            image_size,
            samples_per_pixel,
        )
    };

    let max_depth = 10;

    let mut image = String::new();
    image += &format!("P3\n{} {}\n255\n", image_size.x, image_size.y);

    let start_time = Instant::now();

    // let all_samples = camera.all_samples().progress().flatten();
    let all_samples = camera.all_samples_vec().into_par_iter().progress();

    let pixels = all_samples
        .map(move |row| {
            let mut rng = SmallRng::from_entropy();

            row.into_iter()
                .map(|samples| {
                    let mut color = Rgb::zero();

                    for ray in samples {
                        color += ray_color(ray, max_depth, &world, &mut rng);
                    }

                    color /= camera.samples_per_pixel as f32;
                    color = color.map(|c| c.sqrt()); // map from linear to gamma 2

                    let color = color.map(|c| (c * 255.).round() as u8);

                    format!("{} {} {}\n", color.r, color.g, color.b)
                })
                .collect::<String>()
        })
        .collect::<String>();

    println!("Time taken: {:.2}s", start_time.elapsed().as_secs_f32());

    image += &pixels;

    fs::write("image.ppm", image).unwrap();
}
