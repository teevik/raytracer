use vek::geom::repr_simd::Ray;

use crate::data::{Material, RayHit, ScatterResult};

use self::{dialectric::Dialectric, lambertian::Lambertian, metal::Metal};

pub mod dialectric;
pub mod lambertian;
pub mod metal;

#[derive(Debug, Clone)]
pub enum Materials {
    Lambertian(Lambertian),
    Metal(Metal),
    Dialectric(Dialectric),
}

impl Material for Materials {
    fn scatter(&self, ray: Ray<f32>, ray_hit: RayHit) -> Option<ScatterResult> {
        match self {
            Materials::Lambertian(lambertian) => lambertian.scatter(ray, ray_hit),
            Materials::Metal(metal) => metal.scatter(ray, ray_hit),
            Materials::Dialectric(dialectric) => dialectric.scatter(ray, ray_hit),
        }
    }
}
