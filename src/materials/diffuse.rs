use crate::data::{Ray, RayHit, ScatterResult};
use crate::extensions::RngExtension;
use crate::texture::Texture;
use rand::Rng;
use std::option::Option;

pub fn scatter(albedo: &Texture, ray_hit: &RayHit, rng: &mut impl Rng) -> Option<ScatterResult> {
    let mut scatter_direction = ray_hit.normal + rng.random_unit_vector();

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
