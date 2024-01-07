pub mod bvh;
pub mod camera;
pub mod data;
pub mod extensions;
pub mod interval;
pub mod materials;
pub mod shapes;
pub mod texture;

use crate::camera::Camera;
use crate::data::{Ray, RayHit};
use crate::extensions::RngExtension;
use crate::shapes::sphere::Sphere;
use crate::{bvh::BvhNode, camera::calculate_viewport};
use bvh::Aabb;
use data::{Hittable, ScatterResult};
use image::RgbImage;
use indicatif::{ParallelProgressIterator, ProgressStyle};
use interval::Interval;
use rand::thread_rng;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use shapes::quad::Quad;
use std::time::Instant;
use vek::{Rgb, Vec2};

#[derive(Debug, Clone, Default)]
pub struct Scene {
    pub camera: Camera,
    pub spheres: Vec<Sphere>,
    pub quads: Vec<Quad>,
}

pub struct World {
    pub spheres: Option<BvhNode<Sphere>>,
    pub quads: Option<BvhNode<Quad>>,

    pub bounding_box: Aabb,
}

impl World {
    pub fn new(scene: &Scene) -> Self {
        let spheres = BvhNode::new(&scene.spheres, &mut thread_rng());
        let quads = BvhNode::new(&scene.quads, &mut thread_rng());

        let bounding_box = [
            spheres.as_ref().map(|spheres| spheres.bounding_box()),
            quads.as_ref().map(|quads| quads.bounding_box()),
        ]
        .into_iter()
        .flatten()
        .collect::<Option<Aabb>>()
        .expect("Empty scene");

        Self {
            spheres,
            quads,
            bounding_box,
        }
    }
}

impl Hittable for World {
    fn bounding_box(&self) -> Aabb {
        self.bounding_box
    }

    fn raycast(&self, ray: Ray, interval: Interval) -> Option<RayHit> {
        let raycasts = [
            self.spheres
                .as_ref()
                .and_then(|spheres| spheres.raycast(ray, interval)),
            self.quads
                .as_ref()
                .and_then(|quads| quads.raycast(ray, interval)),
        ];

        let closest = raycasts
            .into_iter()
            .flatten()
            .min_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

        closest
    }
}

fn ray_color(
    ray: Ray,
    world: &World,
    depth_left: u32,
    background_color: Rgb<f32>,
    rng: &mut impl Rng,
) -> Rgb<f32> {
    if depth_left == 0 {
        return Rgb::zero();
    }

    let interval = Interval::new(0.001, f32::INFINITY);

    if let Some(ray_hit) = world.raycast(ray, interval) {
        let emission_color = ray_hit.material.emit(ray_hit.uv, ray_hit.point);

        if let Some(scatter_result) = ray_hit.material.scatter(ray, &ray_hit, rng) {
            let ScatterResult {
                scattered,
                attenuation,
            } = scatter_result;

            attenuation * ray_color(scattered, world, depth_left - 1, background_color, rng)
        } else {
            emission_color
        }
    } else {
        // Didn't hit anything
        background_color
    }
}

fn pixel_sample_offset(rng: &mut impl Rng) -> Vec2<f32> {
    Vec2::new(rng.gen(), rng.gen()) - (Vec2::broadcast(0.5)) // From -0.5 to 0.5
}

fn defocus_sample_offset(rng: &mut impl Rng) -> Vec2<f32> {
    rng.random_in_unit_disk()
}

pub fn render_image(scene: Scene) -> RgbImage {
    let image_size = Vec2::<u32>::new(600, 600);
    let amount_of_samples = 10000;
    let max_depth = 100;

    let world = World::new(&scene);
    let viewport = calculate_viewport(scene.camera, image_size);

    // Raytracing
    let start_time = Instant::now();

    let rows = (0..image_size.y).into_par_iter().progress_with_style(
        ProgressStyle::with_template(
            "[{elapsed} / {eta}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
        )
        .unwrap(),
    );
    let rows = rows
        .map(|y| {
            let mut pixels = Vec::with_capacity(image_size.x as usize);
            let mut rng = SmallRng::from_entropy();

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

                    color += ray_color(ray, &world, max_depth, viewport.background_color, &mut rng);
                }

                color /= amount_of_samples as f32;
                color = color.map(|c| c.sqrt()); // map from linear to gamma 2

                let color = color.map(|c| (c * 255.).round() as u8);

                pixels.push(color);
            }

            pixels
        })
        .collect::<Vec<_>>();

    let mut image = RgbImage::new(image_size.x, image_size.y);

    for (y, row) in rows.into_iter().enumerate() {
        for (x, pixel) in row.into_iter().enumerate() {
            image.put_pixel(x as u32, y as u32, pixel.into_array().into());
        }
    }

    eprintln!("Time taken: {:.2}s", start_time.elapsed().as_secs_f32());

    image
}
