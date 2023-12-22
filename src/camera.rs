use crate::extensions::Vec3Ext;
use indicatif::ParallelProgressIterator;
use rand::{random, thread_rng};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use vek::{Ray, Vec2, Vec3};

pub struct Camera {
    camera_position: Vec3<f32>,

    defocus_angle: f32,
    defocus_disk_u: Vec3<f32>,
    defocus_disk_v: Vec3<f32>,

    image_size: Vec2<usize>,
    pub samples_per_pixel: u32,

    pixel_delta_u: Vec3<f32>,
    pixel_delta_v: Vec3<f32>,
    first_pixel: Vec3<f32>,
}

impl Camera {
    pub fn new(
        camera_position: Vec3<f32>,
        camera_look_at: Vec3<f32>,
        camera_up: Vec3<f32>,

        defocus_angle: f32,
        focus_distance: f32,

        vertical_fov: f32,
        image_size: Vec2<usize>,
        samples_per_pixel: u32,
    ) -> Self {
        let aspect_ratio = image_size.x as f32 / image_size.y as f32;

        let w = (camera_position - camera_look_at).normalized();
        let u = camera_up.cross(w).normalized();
        let v = w.cross(u);

        let fov_height = f32::tan(vertical_fov / 2.);
        let viewport_height = 2. * fov_height * focus_distance;
        let viewport_width = viewport_height * aspect_ratio;

        let viewport_u = viewport_width * u;
        let viewport_v = -viewport_height * v;

        let viewport_upper_left =
            camera_position - (focus_distance * w) - (viewport_u / 2.) - (viewport_v / 2.);

        let pixel_delta_u = viewport_u / (image_size.x as f32);
        let pixel_delta_v = viewport_v / (image_size.y as f32);

        let first_pixel = viewport_upper_left + ((pixel_delta_u + pixel_delta_v) / 2.);

        let defocus_radius = focus_distance * f32::tan(defocus_angle / 2.);
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Self {
            camera_position,

            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,

            image_size,
            samples_per_pixel,

            pixel_delta_u,
            pixel_delta_v,
            first_pixel,
        }
    }
}

impl Camera {
    fn row(
        &self,
        y: usize,
    ) -> impl ParallelIterator<Item = impl Iterator<Item = Ray<f32>> + '_> + '_ {
        (0..self.image_size.x).into_par_iter().map(move |x| {
            let pixel_center = self.first_pixel
                + (self.pixel_delta_u * x as f32)
                + (self.pixel_delta_v * y as f32);

            (0..self.samples_per_pixel).map(move |_| {
                let origin = if self.defocus_angle > 0. {
                    let offset = Vec3::random_in_unit_disk(&mut thread_rng());

                    self.camera_position
                        + offset.x * self.defocus_disk_u
                        + offset.y * self.defocus_disk_v
                } else {
                    self.camera_position
                };

                let sample_offset = (random::<f32>() - 0.5) * self.pixel_delta_u
                    + (random::<f32>() - 0.5) * self.pixel_delta_v;

                let sample_position = pixel_center + sample_offset;
                let direction = sample_position - origin;

                Ray::new(origin, direction)
            })
        })
    }

    pub fn all_samples(
        &self,
    ) -> impl ParallelIterator<Item = impl Iterator<Item = Ray<f32>> + '_> + '_ {
        (0..self.image_size.y)
            .into_par_iter()
            .progress()
            .flat_map(|y| self.row(y))
    }
}
