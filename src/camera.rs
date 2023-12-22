use itertools::Itertools;
use rand::random;
use vek::{Vec2, Vec3};

pub struct Camera {
    pub camera_center: Vec3<f32>,

    pub image_size: Vec2<usize>,
    pub viewport: Vec2<f32>,
    pub focal_length: f32,
    pub samples_per_pixel: u32,
}

impl Camera {
    pub fn new(
        camera_center: Vec3<f32>,
        vertical_fov: f32,
        image_size: Vec2<usize>,
        focal_length: f32,
        samples_per_pixel: u32,
    ) -> Self {
        let aspect_ratio = image_size.x as f32 / image_size.y as f32;
        let fov_height = f32::tan(vertical_fov / 2.);

        let viewport_height = 2. * fov_height * focal_length;
        let viewport_width = viewport_height * aspect_ratio;
        let viewport = Vec2::new(viewport_width, -viewport_height);

        Self {
            camera_center,
            image_size,
            viewport,
            focal_length,
            samples_per_pixel,
        }
    }
}

impl Camera {
    pub fn all_samples(&self) -> Vec<Vec<Vec3<f32>>> {
        let pixel_delta = self.viewport / self.image_size.as_::<f32>();

        let viewport_upper_left =
            self.camera_center - Vec3::new(0., 0., self.focal_length) - (self.viewport / 2.);

        let first_pixel = viewport_upper_left + (pixel_delta / 2.);

        let row = |y| {
            (0..self.image_size.x).map(move |x| {
                let pixel = Vec2::new(x, y);
                let pixel_center = first_pixel + (pixel_delta * pixel.as_::<f32>());

                (0..self.samples_per_pixel)
                    .map(move |_| {
                        let sample_offset =
                            Vec2::new(random::<f32>() - 0.5, random::<f32>() - 0.5) * pixel_delta;
                        let sample_position = pixel_center + sample_offset;

                        sample_position - self.camera_center
                    })
                    .collect_vec()
            })
        };

        (0..self.image_size.y).flat_map(row).collect_vec()
    }
}
