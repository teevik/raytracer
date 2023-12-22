use crate::{materials::Material, sphere::Sphere, world::World};
use vek::{Rgb, Vec3};

#[allow(dead_code)]
pub fn scene_1() -> World {
    let material_ground = Material::Lambertian {
        albedo: Rgb::new(0.8, 0.8, 0.),
    };

    let material_center = Material::Lambertian {
        albedo: Rgb::new(0.1, 0.2, 0.5),
    };

    let material_left = Material::Dialectric {
        refraction_index: 1.5,
    };

    let material_right = Material::Metal {
        albedo: Rgb::new(0.8, 0.6, 0.2),
        fuzz: 0.1,
    };

    World(vec![
        (Sphere {
            center: Vec3::new(0., -100.5, -1.),
            radius: 100.,
            material: material_ground,
        }),
        (Sphere {
            center: Vec3::new(0., 0., -1.),
            radius: 0.5,
            material: material_center,
        }),
        (Sphere {
            center: Vec3::new(-1., 0., -1.),
            radius: -0.4,
            material: material_left,
        }),
        (Sphere {
            center: Vec3::new(1., 0., -1.),
            radius: 0.5,
            material: material_right,
        }),
    ])
}
