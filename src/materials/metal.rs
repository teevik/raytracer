use crate::{
    data::{RayHit, ScatterResult},
    extensions::Vec3Ext,
};
use rand::thread_rng;
use std::option::Option;
use vek::{Ray, Rgb, Vec3};

pub fn scatter(
    albedo: Rgb<f32>,
    fuzz: f32,
    ray: Ray<f32>,
    ray_hit: RayHit,
) -> Option<ScatterResult> {
    let reflected = ray.direction.normalized().reflected(ray_hit.normal);

    let scattered = Ray::new(
        ray_hit.point,
        reflected + Vec3::random_unit_vector(&mut thread_rng()) * fuzz,
    );
    let attenuation = albedo;

    if scattered.direction.dot(ray_hit.normal) > 0. {
        Some(ScatterResult {
            scattered,
            attenuation,
        })
    } else {
        None
    }
}
