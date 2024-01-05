use crate::data::{RayHit, ScatterResult};
use rand::Rng;
use vek::{Ray, Rgb};

mod diffuse;
mod glass;
mod metal;

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Diffuse { albedo: Rgb<f32> },
    Metal { albedo: Rgb<f32>, fuzz: f32 },
    Glass { refraction_index: f32 },
}

impl Material {
    pub fn scatter(
        self,
        ray: Ray<f32>,
        ray_hit: RayHit,
        rng: &mut impl Rng,
    ) -> Option<ScatterResult> {
        match self {
            Material::Diffuse { albedo } => diffuse::scatter(albedo, ray_hit, rng),
            Material::Metal { albedo, fuzz } => metal::scatter(albedo, fuzz, ray, ray_hit, rng),
            Material::Glass { refraction_index } => {
                glass::scatter(refraction_index, ray, ray_hit, rng)
            }
        }
    }
}
