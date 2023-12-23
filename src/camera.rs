use std::cell::RefCell;

use crate::extensions::{Vec2Ext, Vec3InVec2Ext};
use rand::{random, rngs::SmallRng, SeedableRng};
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use vek::{Mat4, Ray, Vec2, Vec3};

pub struct Camera {
    camera_position: Vec3<f32>,

    defocus_angle: f32,
    defocus_disk: Vec2<Vec3<f32>>,

    image_size: Vec2<usize>,
    pub samples_per_pixel: u32,

    pixel_deltas: Vec2<Vec3<f32>>,
    first_pixel: Vec3<f32>,
}

impl Camera {
    pub fn new(
        camera_position: Vec3<f32>,
        camera_target: Vec3<f32>,
        camera_up: Vec3<f32>,

        defocus_angle: f32,
        focus_distance: f32,

        vertical_fov: f32,
        image_size: Vec2<usize>,
        samples_per_pixel: u32,
    ) -> Self {
        let aspect_ratio = image_size.x as f32 / image_size.y as f32;

        let view_matrix = Mat4::<f32>::look_at_rh(camera_position, camera_target, camera_up);

        let fov_height = f32::tan(vertical_fov / 2.);
        let viewport_height = 2. * fov_height * focus_distance;
        let viewport_width = viewport_height * aspect_ratio;

        let viewport_directions = Vec2::new(
            Vec3::unit_x() * viewport_width,
            Vec3::unit_y() * -viewport_height,
        )
        .map(|direction| direction.with_w(1.) * view_matrix)
        .map(|direction| direction.xyz());

        let focal_length = (Vec3::unit_z() * focus_distance).with_w(1.) * view_matrix;

        let viewport_upper_left =
            camera_position - focal_length.xyz() - (viewport_directions.sum() / 2.);

        let pixel_deltas = viewport_directions.div_elements(image_size.as_::<f32>());
        let first_pixel = viewport_upper_left + ((pixel_deltas.sum()) / 2.);

        let defocus_radius = focus_distance * f32::tan(defocus_angle / 2.);
        let defocus_disk = Vec2::new(
            Vec3::new(defocus_radius, 0., 0.),
            Vec3::new(0., defocus_radius, 0.),
        )
        .map(|defocus| defocus.with_w(1.) * view_matrix)
        .map(|defocus| defocus.xyz());

        Self {
            camera_position,

            defocus_angle,
            defocus_disk,

            image_size,
            samples_per_pixel,

            pixel_deltas,
            first_pixel,
        }
    }
}

impl Camera {
    fn row(&self, y: usize) -> impl Iterator<Item = impl Iterator<Item = Ray<f32>> + '_> + '_ {
        (0..self.image_size.x).map(move |x| {
            let mut rng = SmallRng::from_entropy();
            let pixel = Vec2::new(x, y);

            let pixel_offset = self.pixel_deltas.mul_elements(pixel.as_::<f32>()).sum();
            let pixel_center = self.first_pixel + pixel_offset;

            (0..self.samples_per_pixel).map(move |_| {
                let origin = if self.defocus_angle > 0. {
                    let offset = Vec2::random_in_unit_disk(&mut rng);

                    self.camera_position + self.defocus_disk.mul_elements(offset).sum()
                } else {
                    self.camera_position
                };

                let sample_offset = self
                    .pixel_deltas
                    .mul_elements(Vec2::new(random::<f32>() - 0.5, random::<f32>() - 0.5))
                    .sum();

                let sample_position = pixel_center + sample_offset;
                let direction = sample_position - origin;

                Ray::new(origin, direction)
            })
        })
    }

    #[allow(dead_code)]
    pub fn all_samples(
        &self,
    ) -> impl ExactSizeIterator<Item = impl Iterator<Item = impl Iterator<Item = Ray<f32>> + '_> + '_> + '_
    {
        (0..self.image_size.y).map(|y| self.row(y))
    }

    #[allow(dead_code)]
    pub fn all_samples_parallel(
        &self,
    ) -> impl IndexedParallelIterator<
        Item = impl Iterator<Item = impl Iterator<Item = Ray<f32>> + '_> + '_,
    > + '_ {
        (0..self.image_size.y).into_par_iter().map(|y| self.row(y))
    }

    pub fn all_samples_vec(&self) -> Vec<Vec<Vec<Ray<f32>>>> {
        let rng = &RefCell::new(SmallRng::from_entropy());

        (0..self.image_size.y)
            .map(|y| {
                (0..self.image_size.x)
                    .map(move |x| {
                        let pixel = Vec2::new(x, y);

                        let pixel_offset = self.pixel_deltas.mul_elements(pixel.as_::<f32>()).sum();
                        let pixel_center = self.first_pixel + pixel_offset;

                        (0..self.samples_per_pixel)
                            .map(move |_| {
                                let origin = if self.defocus_angle > 0. {
                                    let offset = Vec2::random_in_unit_disk(&mut *rng.borrow_mut());

                                    self.camera_position
                                        + self.defocus_disk.mul_elements(offset).sum()
                                } else {
                                    self.camera_position
                                };

                                let sample_offset = self
                                    .pixel_deltas
                                    .mul_elements(Vec2::new(
                                        random::<f32>() - 0.5,
                                        random::<f32>() - 0.5,
                                    ))
                                    .sum();

                                let sample_position = pixel_center + sample_offset;
                                let direction = sample_position - origin;

                                Ray::new(origin, direction)
                            })
                            .collect()
                    })
                    .collect()
            })
            .collect()
    }
}
