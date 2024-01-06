use std::sync::Arc;

use crate::camera::Camera;
use crate::materials::Texture;
use crate::scenes::Scene;
use crate::{materials::Material, sphere::Sphere};
use noise::{Perlin, Turbulence};
use vek::Vec3;

#[allow(dead_code)]
pub fn two_perlin_spheres_scene() -> Scene {
    let camera = Camera {
        position: Vec3::new(13., 2., 3.),
        target: Vec3::new(0., 0., 0.),
        up: Vec3::new(0., 1., 0.),

        vertical_fov: (20_f32).to_radians(),
        defocus_angle: (0_f32).to_radians(),
        focus_distance: 1.,
    };

    // let perlin = Perlin::new(0);
    let perlin = Turbulence::<_, Perlin>::new(Perlin::new(0));

    let perlin_material = Material::Diffuse {
        albedo: Texture::noise(Arc::new(perlin), 5.),
    };

    let spheres = vec![
        Sphere {
            center: Vec3::new(0., -1000., 0.),
            radius: 1000.,
            material: perlin_material.clone(),
        },
        Sphere {
            center: Vec3::new(0., 2., 0.),
            radius: 2.,
            material: perlin_material,
        },
    ];

    Scene { camera, spheres }
}
