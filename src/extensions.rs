use rand::Rng;
use vek::{Rgb, Vec2, Vec3};

pub trait RngExtension: Rng {
    fn random_in_unit_disk(&mut self) -> Vec2<f32> {
        let mut random = || self.gen_range(-1. ..=1.);

        loop {
            let sample = Vec2::new(random(), random());

            if sample.magnitude_squared() < 1. {
                break sample;
            }
        }
    }

    fn random_in_unit_sphere(&mut self) -> Vec3<f32> {
        let mut random = || self.gen_range(-1. ..=1.);

        loop {
            let sample = Vec3::new(random(), random(), random());

            if sample.magnitude_squared() < 1. {
                break sample;
            }
        }
    }

    fn random_unit_vector(&mut self) -> Vec3<f32> {
        self.random_in_unit_sphere().normalized()
    }

    fn random_on_hemisphere(&mut self, normal: Vec3<f32>) -> Vec3<f32> {
        let on_unit_sphere = self.random_unit_vector();

        if on_unit_sphere.dot(normal) > 0. {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    fn random_color(&mut self) -> Rgb<f32> {
        Rgb::new(self.gen(), self.gen(), self.gen())
    }
}

impl<T: Rng> RngExtension for T {}
