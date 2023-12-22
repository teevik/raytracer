use crate::{materials::Material, sphere::Sphere, world::World};
use std::f32::consts::PI;
use vek::{Rgb, Vec3};

#[allow(dead_code)]
pub fn scene_2() -> World {
    let material_left = Material::Lambertian {
        albedo: Rgb::new(0., 0., 1.),
    };

    let material_right = Material::Lambertian {
        albedo: Rgb::new(1., 0., 0.),
    };

    let radius = f32::cos(PI / 4.);

    World(vec![
        (Sphere {
            center: Vec3::new(-radius, 0., -1.),
            radius,
            material: material_left,
        }),
        (Sphere {
            center: Vec3::new(radius, 0., -1.),
            radius,
            material: material_right,
        }),
    ])
}
