mod bvh;
mod camera;
mod data;
mod extensions;
mod interval;
mod materials;
mod scenes;
mod sphere;

use crate::data::RayHit;
use crate::extensions::Vec2Ext;
use crate::scenes::two_perlin_spheres::two_perlin_spheres_scene;
use crate::scenes::Scene;
use crate::{bvh::BvhNode, camera::calculate_viewport};
use bvh::Aabb;
use data::Hittable;
use indicatif::ParallelProgressIterator;
use interval::Interval;
use rand::thread_rng;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use sphere::Sphere;
use std::{fs, time::Instant};
use vek::{Ray, Rgb, Vec2, Vec3};

pub struct World {
    pub spheres: BvhNode<Sphere>,
}

impl Hittable for World {
    fn bounding_box(&self) -> Aabb {
        self.spheres.bounding_box()
    }

    fn raycast(&self, ray: Ray<f32>, interval: Interval) -> Option<RayHit> {
        self.spheres.raycast(ray, interval)
    }
}

fn ray_color(ray: Ray<f32>, world: &World, max_depth: u32, rng: &mut impl Rng) -> Vec3<f32> {
    let mut accumulated_color = Vec3::one();
    let mut next_ray = ray;

    for _ in 0..max_depth {
        let interval = Interval::new(0.001, f32::INFINITY);

        if let Some(ray_hit) = world.raycast(next_ray, interval) {
            if let Some(scatter_result) = ray_hit.material.scatter(next_ray, &ray_hit, rng) {
                accumulated_color *= scatter_result.attenuation;
                next_ray = scatter_result.scattered;
            } else {
                // Didn't scatter
                return Vec3::zero();
            }
        } else {
            // Didn't hit anything
            let unit_direction = next_ray.direction.normalized();
            let a = (unit_direction.y + 1.) / 2.;
            let background_color = Vec3::broadcast(1. - a) + a * Vec3::new(0.5, 0.7, 1.);

            accumulated_color *= background_color;

            return accumulated_color;
        }
    }

    // Reached max depth
    Vec3::zero()
}

fn pixel_sample_offset(rng: &mut impl Rng) -> Vec2<f32> {
    Vec2::new(rng.gen(), rng.gen()) - (Vec2::broadcast(0.5)) // From -0.5 to 0.5
}

fn defocus_sample_offset(rng: &mut impl Rng) -> Vec2<f32> {
    Vec2::random_in_unit_disk(rng)
}

fn main() {
    let image_size = Vec2::<u32>::new(800, 400);
    let amount_of_samples = 50;
    let max_depth = 50;

    let Scene { camera, spheres } = two_perlin_spheres_scene();
    let viewport = calculate_viewport(camera, image_size);

    let spheres_bvh = BvhNode::new(&spheres, &mut thread_rng());

    let world = World {
        spheres: spheres_bvh,
    };

    // Raytracing
    let mut ppm = String::new();
    ppm += &format!("P3\n{} {}\n255\n", image_size.x, image_size.y);

    let start_time = Instant::now();

    let rows = (0..image_size.y).into_par_iter().progress();
    let image = rows
        .map(|y| {
            let mut rng = SmallRng::from_entropy();
            let mut pixels = String::new();

            for x in 0..image_size.x {
                let pixel_position = Vec2::new(x, y);

                let mut color = Rgb::zero();

                for _ in 0..amount_of_samples {
                    let sample_position =
                        pixel_position.as_::<f32>() + pixel_sample_offset(&mut rng);

                    let pixel_center = viewport.upper_left_pixel_position
                        + sample_position.x * viewport.horizontal_pixel_delta
                        + sample_position.y * viewport.vertical_pixel_delta;

                    let defocus_offset = defocus_sample_offset(&mut rng);
                    let ray_origin = viewport.origin
                        + defocus_offset.x * viewport.horizontal_defocus_disk
                        + defocus_offset.y * viewport.vertical_defocus_disk;

                    let ray_direction = pixel_center - ray_origin;

                    let ray = Ray::new(ray_origin, ray_direction);

                    color += ray_color(ray, &world, max_depth, &mut rng);
                }

                color /= amount_of_samples as f32;
                color = color.map(|c| c.sqrt()); // map from linear to gamma 2

                let color = color.map(|c| (c * 255.).round() as u8);

                pixels += &format!("{} {} {}\n", color.r, color.g, color.b);
            }

            pixels
        })
        .collect::<String>();

    println!("Time taken: {:.2}s", start_time.elapsed().as_secs_f32());

    ppm += &image;

    fs::write("image.ppm", ppm).unwrap();
}
