use std::sync::Arc;

use crate::camera::Camera;
use crate::materials::Texture;
use crate::scenes::Scene;
use crate::{materials::Material, sphere::Sphere};
use vek::Vec3;

#[allow(dead_code)]
pub fn earth_scene() -> Scene {
    let camera = Camera {
        position: Vec3::new(0., 0., 12.),
        target: Vec3::new(0., 0., 0.),
        up: Vec3::new(0., 1., 0.),

        vertical_fov: (20_f32).to_radians(),
        defocus_angle: (0_f32).to_radians(),
        focus_distance: 1.,
    };

    let earth_image = image::open("./resources/earthmap.jpg")
        .unwrap()
        .into_rgb32f();

    let earth_material = Material::Diffuse {
        albedo: Texture::image(Arc::new(earth_image)),
    };

    let spheres = vec![Sphere {
        center: Vec3::new(0., 0., 0.),
        radius: 2.,
        material: earth_material,
    }];

    Scene { camera, spheres }
}
