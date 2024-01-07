use crate::data::{Ray, RayHit, ScatterResult};
use crate::extensions::RngExtension;
use crate::texture::Texture;
use rand::Rng;
use std::option::Option;

pub fn scatter(
    albedo: &Texture,
    fuzz: f32,
    ray: Ray,
    ray_hit: &RayHit,
    rng: &mut impl Rng,
) -> Option<ScatterResult> {
    let reflected = ray.direction.normalized().reflected(ray_hit.normal);

    let scattered = Ray::new(ray_hit.point, reflected + rng.random_unit_vector() * fuzz);
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
