use crate::data::{Face, RayHit, ScatterResult};
use rand::random;
use std::option::Option;
use vek::{Ray, Rgb, Vec3};

fn reflectance(cosine: f32, refraction_ratio: f32) -> f32 {
    let r0 = (1. - refraction_ratio) / (1. + refraction_ratio);
    let r0 = r0 * r0;

    r0 + (1. - r0) * f32::powi(1. - cosine, 5)
}

pub fn scatter(refraction_index: f32, ray: Ray<f32>, ray_hit: RayHit) -> Option<ScatterResult> {
    let refraction_ratio = match ray_hit.face {
        Face::Front => 1. / refraction_index,
        Face::Back => refraction_index,
    };

    let unit_direction = ray.direction.normalized();
    let cos_theta = f32::min(Vec3::dot(-unit_direction, ray_hit.normal), 1.);
    let sin_theta = f32::sqrt(1. - cos_theta * cos_theta);

    let cannot_refract =
        (refraction_ratio * sin_theta > 1.) || reflectance(cos_theta, refraction_ratio) > random();

    let direction = if cannot_refract {
        unit_direction.reflected(ray_hit.normal)
    } else {
        unit_direction.refracted(ray_hit.normal, refraction_ratio)
    };

    let scattered = Ray::new(ray_hit.point, direction);
    let attenuation = Rgb::white();

    Some(ScatterResult {
        scattered,
        attenuation,
    })
}
