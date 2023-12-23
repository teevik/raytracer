use rand::Rng;
use vek::{Ray, Rgb, Vec2, Vec3};

use crate::data::Face;

pub trait Vec2Ext<T> {
    fn random_in_unit_disk(rng: &mut impl Rng) -> Vec2<T>;
}

impl Vec2Ext<f32> for Vec2<f32> {
    fn random_in_unit_disk(rng: &mut impl Rng) -> Vec2<f32> {
        let mut random = || rng.gen_range(-1. ..=1.);

        loop {
            let sample = Vec2::new(random(), random());

            if sample.magnitude_squared() < 1. {
                break sample;
            }
        }
    }
}

pub trait Vec3Ext<T> {
    fn random_in_unit_sphere(rng: &mut impl Rng) -> Vec3<T>;
    fn random_unit_vector(rng: &mut impl Rng) -> Vec3<T>;
    fn random_on_hemisphere(normal: Vec3<T>, rng: &mut impl Rng) -> Vec3<T>;
}

impl Vec3Ext<f32> for Vec3<f32> {
    fn random_in_unit_sphere(rng: &mut impl Rng) -> Vec3<f32> {
        let mut random = || rng.gen_range(-1. ..=1.);

        loop {
            let sample = Vec3::new(random(), random(), random());

            if sample.magnitude_squared() < 1. {
                break sample;
            }
        }
    }

    fn random_unit_vector(rng: &mut impl Rng) -> Vec3<f32> {
        Self::random_in_unit_sphere(rng).normalized()
    }

    fn random_on_hemisphere(normal: Vec3<f32>, rng: &mut impl Rng) -> Vec3<f32> {
        let on_unit_sphere = Self::random_unit_vector(rng);

        if on_unit_sphere.dot(normal) > 0. {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }
}

pub trait Vec3InVec2Ext<T> {
    fn div_elements(self, other: Vec2<T>) -> Self;
    fn mul_elements(self, other: Vec2<T>) -> Self;
}

impl Vec3InVec2Ext<f32> for Vec2<Vec3<f32>> {
    fn div_elements(self, other: Vec2<f32>) -> Self {
        Vec2::new(self.x / other.x, self.y / other.y)
    }

    fn mul_elements(self, other: Vec2<f32>) -> Self {
        Vec2::new(self.x * other.x, self.y * other.y)
    }
}

pub trait RgbExt<T> {
    fn random(rng: &mut impl Rng) -> Rgb<T>;
}

impl RgbExt<f32> for Rgb<f32> {
    fn random(rng: &mut impl Rng) -> Rgb<f32> {
        Rgb::new(rng.gen(), rng.gen(), rng.gen())
    }
}

pub trait RayExt<T> {
    fn at(self, t: T) -> Vec3<T>;

    fn get_face(self, outward_normal: Vec3<T>) -> Face;
}

impl RayExt<f32> for Ray<f32> {
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
