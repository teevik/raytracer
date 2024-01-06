use crate::{
    data::{RayHit, ScatterResult},
    extensions::Vec3Ext,
};
use rand::Rng;
use std::option::Option;
use vek::{Ray, Vec3};

use super::Texture;

pub fn scatter(
    albedo: &Texture,
    fuzz: f32,
    ray: Ray<f32>,
    ray_hit: &RayHit,
    rng: &mut impl Rng,
) -> Option<ScatterResult> {
    let reflected = ray.direction.normalized().reflected(ray_hit.normal);

    let scattered = Ray::new(
        ray_hit.point,
        reflected + Vec3::random_unit_vector(rng) * fuzz,
    );
    let attenuation = albedo.color_at(ray_hit.uv, ray_hit.point);

    if scattered.direction.dot(ray_hit.normal) > 0. {
        Some(ScatterResult {
            scattered,
            attenuation,
        })
    } else {
        None
    }
}
