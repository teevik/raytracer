use crate::{
    data::{RayHit, ScatterResult},
    extensions::Vec3Ext,
};
use rand::Rng;
use std::option::Option;
use vek::{Ray, Rgb, Vec3};

pub fn scatter(albedo: Rgb<f32>, ray_hit: RayHit, rng: &mut impl Rng) -> Option<ScatterResult> {
    let mut scatter_direction = ray_hit.normal + Vec3::random_unit_vector(rng);

    // Catch degenerate scatter direction
    if scatter_direction.is_approx_zero() {
        scatter_direction = ray_hit.normal;
    }

    let scattered = Ray::new(ray_hit.point, scatter_direction);
    let attenuation = albedo;

    Some(ScatterResult {
        scattered,
        attenuation,
    })
}