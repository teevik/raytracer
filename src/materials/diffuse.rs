use crate::{
    data::{RayHit, ScatterResult},
    extensions::Vec3Ext,
};
use rand::Rng;
use std::option::Option;
use vek::{Ray, Vec3};

use super::Texture;

pub fn scatter(albedo: &Texture, ray_hit: &RayHit, rng: &mut impl Rng) -> Option<ScatterResult> {
    let mut scatter_direction = ray_hit.normal + Vec3::random_unit_vector(rng);

    // Catch degenerate scatter direction
    if scatter_direction.is_approx_zero() {
        scatter_direction = ray_hit.normal;
    }

    let scattered = Ray::new(ray_hit.point, scatter_direction);
    let attenuation = albedo.color_at(ray_hit.uv, ray_hit.point);

    Some(ScatterResult {
        scattered,
        attenuation,
    })
}
