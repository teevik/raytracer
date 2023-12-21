use rand::random;
use rand::rngs::ThreadRng;
use rand::thread_rng;
use rand::Rng;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::{fs, ops::Range};
use vek::{
    geom::repr_simd::{Ray, Sphere},
    vec::repr_simd::{Vec2, Vec3},
};

trait Vec3Ext {
    fn random_in_unit_sphere(rng: &mut ThreadRng) -> Vec3<f32>;
    fn random_unit_vector(rng: &mut ThreadRng) -> Vec3<f32>;
    fn random_on_hemisphere(normal: Vec3<f32>, rng: &mut ThreadRng) -> Vec3<f32>;
}

impl Vec3Ext for Vec3<f32> {
    fn random_in_unit_sphere(rng: &mut ThreadRng) -> Vec3<f32> {
        let mut random = || rng.gen_range(-1. ..=1.);

        loop {
            let sample = Vec3::new(random(), random(), random());
            if sample.magnitude_squared() < 1. {
                break sample;
            }
        }
    }

    fn random_unit_vector(rng: &mut ThreadRng) -> Vec3<f32> {
        Self::random_in_unit_sphere(rng).normalized()
    }

    fn random_on_hemisphere(normal: Vec3<f32>, rng: &mut ThreadRng) -> Vec3<f32> {
        let on_unit_sphere = Self::random_unit_vector(rng);

        if on_unit_sphere.dot(normal) > 0. {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Face {
    Front,
    Back,
}

trait RayExt {
    fn at(self, t: f32) -> Vec3<f32>;

    fn get_face(self, outward_normal: Vec3<f32>) -> Face;
}

impl RayExt for Ray<f32> {
    fn at(self, t: f32) -> Vec3<f32> {
        self.origin + (self.direction * t)
    }

    fn get_face(self, outward_normal: Vec3<f32>) -> Face {
        let direction = Vec3::dot(self.direction, outward_normal);

        if direction < 0. {
            Face::Front
        } else {
            Face::Back
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct RayHit {
    /// Distance to hit
    distance: f32,

    /// The point where the ray hit
    point: Vec3<f32>,

    /// Which face
    face: Face,

    /// Normal, unit length
    normal: Vec3<f32>,
}

trait Hittable {
    fn hit(&self, ray: Ray<f32>, range: Range<f32>) -> Option<RayHit>;
}

type World = Vec<Box<dyn Hittable + Send + Sync>>;

impl Hittable for World {
    fn hit(&self, ray: Ray<f32>, range: Range<f32>) -> Option<RayHit> {
        let mut closest_hit = None;
        let mut closest_distance = range.end;

        for hittable in self {
            if let Some(hit) = hittable.hit(ray, range.start..closest_distance) {
                closest_hit = Some(hit);
                closest_distance = hit.distance;
            }
        }

        closest_hit
    }
}

impl Hittable for Sphere<f32, f32> {
    fn hit(&self, ray: Ray<f32>, range: Range<f32>) -> Option<RayHit> {
        let center_to_origin = ray.origin - self.center;
        let a = ray.direction.magnitude_squared();
        let half_b = Vec3::dot(center_to_origin, ray.direction);
        let c = center_to_origin.magnitude_squared() - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0. {
            return None;
        }

        let discriminant_sqrt = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let mut root = (-half_b - discriminant_sqrt) / a;
        if !range.contains(&root) {
            root = (-half_b + discriminant_sqrt) / a;

            if !range.contains(&root) {
                return None;
            }
        }

        let distance = root;
        let point = ray.at(distance);

        let outward_normal = (point - self.center) / self.radius;
        let face = ray.get_face(outward_normal);

        let normal = match face {
            Face::Front => outward_normal,
            Face::Back => -outward_normal,
        };

        Some(RayHit {
            distance,
            point,
            face,
            normal,
        })
    }
}

struct Camera {
    camera_center: Vec3<f32>,

    image_size: Vec2<usize>,
    viewport: Vec2<f32>,
    focal_length: f32,
    samples_per_pixel: u32,
}

impl Camera {
    fn pixels(&self) -> impl ParallelIterator<Item = impl Iterator<Item = Vec3<f32>> + '_> + '_ {
        let pixel_delta = self.viewport / self.image_size.as_::<f32>();

        let viewport_upper_left =
            self.camera_center - Vec3::new(0., 0., self.focal_length) - (self.viewport / 2.);

        let first_pixel = viewport_upper_left + (pixel_delta / 2.);

        (0..self.image_size.y).into_par_iter().flat_map(move |y| {
            (0..self.image_size.x).into_par_iter().map(move |x| {
                let pixel = Vec2::new(x, y);
                let pixel_center = first_pixel + (pixel_delta * pixel.as_::<f32>());
                (0..self.samples_per_pixel).map(move |_| {
                    let sample_offset =
                        Vec2::new(random::<f32>() - 0.5, random::<f32>() - 0.5) * pixel_delta;
                    let sample_position = pixel_center + sample_offset;

                    sample_position - self.camera_center
                })
            })
        })
    }
}

fn ray_color(ray: Ray<f32>, depth_left: u32, world: &World) -> Vec3<f32> {
    if depth_left == 0 {
        return Vec3::zero();
    }

    let mut rng = thread_rng();

    // World hit
    let ray_hit = world.hit(ray, 0.001..f32::INFINITY);

    if let Some(ray_hit) = ray_hit {
        // Bounce in random direction
        let direction = ray_hit.normal + Vec3::random_unit_vector(&mut rng);
        // let direction = Vec3::random_on_hemisphere(ray_hit.normal, &mut rng);
        let reflectance = 0.5;
        return ray_color(Ray::new(ray_hit.point, direction), depth_left - 1, world) * reflectance;
    }

    // Background gradient
    let unit_direction = ray.direction.normalized();
    let a = (unit_direction.y + 1.) / 2.;

    Vec3::broadcast(1. - a) + (a * Vec3::new(0.5, 0.7, 1.0))
}

fn main() {
    // World
    let world: World = vec![
        Box::new(Sphere::new(Vec3::new(0., 0., -1.), 0.5)),
        Box::new(Sphere::new(Vec3::new(0., -100.5, -1.), 100.)),
    ];

    // Camera
    let camera_center = Vec3::new(0., 0., 0.);

    let image_size = Vec2::new(800, 400);
    let aspect_ratio = image_size.x as f32 / image_size.y as f32;

    let viewport_height = 2.;
    let viewport_width = viewport_height * aspect_ratio;
    let viewport = Vec2::new(viewport_width, -viewport_height);

    let focal_length = 1.;

    let camera = Camera {
        camera_center,
        image_size,
        viewport,
        focal_length,
        samples_per_pixel: 100,
    };

    let max_depth = 10;

    let mut image = String::new();
    image += &format!("P3\n{} {}\n255\n", image_size.x, image_size.y);

    let pixels = camera
        .pixels()
        .map(move |samples| {
            let mut color = Vec3::zero();

            for pixel in samples {
                let ray = Ray::new(camera_center, pixel);
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
