use crate::data::{RayHit, ScatterResult};
use rand::Rng;
use vek::{Ray, Rgb};

mod dialectric;
mod lambertian;
mod metal;

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian { albedo: Rgb<f32> },
    Metal { albedo: Rgb<f32>, fuzz: f32 },
    Dialectric { refraction_index: f32 },
}

impl Material {
    pub fn scatter(
        self,
        ray: Ray<f32>,
        ray_hit: RayHit,
        rng: &mut impl Rng,
    ) -> Option<ScatterResult> {
        match self {
            Material::Lambertian { albedo } => lambertian::scatter(albedo, ray_hit, rng),
            Material::Metal { albedo, fuzz } => metal::scatter(albedo, fuzz, ray, ray_hit, rng),
            Material::Dialectric { refraction_index } => {
                dialectric::scatter(refraction_index, ray, ray_hit, rng)
            }
        }
    }
}
