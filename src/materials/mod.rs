use crate::data::{Ray, RayHit, ScatterResult};
use crate::texture::Texture;
use rand::Rng;
use std::fmt::Debug;
use vek::{Rgb, Vec2, Vec3};

mod diffuse;
mod diffuse_light;
mod glass;
mod metal;

#[derive(Debug, Clone)]
pub enum Material {
    Diffuse { albedo: Texture },
    Metal { albedo: Texture, fuzz: f32 },
    Glass { refraction_index: f32 },
    DiffuseLight { strength: Texture },
}

impl Material {
    pub fn scatter(&self, ray: Ray, ray_hit: &RayHit, rng: &mut impl Rng) -> Option<ScatterResult> {
        match self {
            Material::Diffuse { albedo } => diffuse::scatter(albedo, ray_hit, rng),
            Material::Metal { albedo, fuzz } => metal::scatter(albedo, *fuzz, ray, ray_hit, rng),
            Material::Glass { refraction_index } => {
                glass::scatter(*refraction_index, ray, ray_hit, rng)
            }
            Material::DiffuseLight { .. } => None,
        }
    }

    pub fn emit(&self, uv: Vec2<f32>, point: Vec3<f32>) -> Rgb<f32> {
        let none = Rgb::zero();

        match self {
            Material::Diffuse { .. } => none,
            Material::Metal { .. } => none,
            Material::Glass { .. } => none,
            Material::DiffuseLight { strength } => diffuse_light::emit(strength, uv, point),
        }
    }
}
