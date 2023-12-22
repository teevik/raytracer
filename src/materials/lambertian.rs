use crate::{
    data::{Material, RayHit, ScatterResult},
    extensions::Vec3Ext,
};
use rand::thread_rng;
use std::option::Option;
use vek::{geom::repr_simd::Ray, vec::repr_simd::Vec3};

#[derive(Debug, Clone)]
pub struct Lambertian {
    pub albedo: Vec3<f32>,
}

impl Material for Lambertian {
    fn scatter(&self, _ray: Ray<f32>, ray_hit: RayHit) -> Option<ScatterResult> {
        let mut scatter_direction = ray_hit.normal + Vec3::random_unit_vector(&mut thread_rng());

        // Catch degenerate scatter direction
        if scatter_direction.is_approx_zero() {
            scatter_direction = ray_hit.normal;
        }

        let scattered = Ray::new(ray_hit.point, scatter_direction);
        let attenuation = self.albedo;

        Some(ScatterResult {
            scattered,
            attenuation,
        })
    }
}