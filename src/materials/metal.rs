use crate::{
    data::{Material, RayHit, ScatterResult},
    extensions::Vec3Ext,
};
use rand::thread_rng;
use std::option::Option;
use vek::{geom::repr_simd::Ray, vec::repr_simd::Vec3};

#[derive(Debug, Clone)]
pub struct Metal {
    pub albedo: Vec3<f32>,
    pub fuzz: f32,
}

impl Material for Metal {
    fn scatter(&self, ray: Ray<f32>, ray_hit: RayHit) -> Option<ScatterResult> {
        let reflected = ray.direction.normalized().reflected(ray_hit.normal);

        let scattered = Ray::new(
            ray_hit.point,
            reflected + Vec3::random_unit_vector(&mut thread_rng()) * self.fuzz,
        );
        let attenuation = self.albedo;

        if scattered.direction.dot(ray_hit.normal) > 0. {
            Some(ScatterResult {
                scattered,
                attenuation,
            })
        } else {
            None
        }
    }
}