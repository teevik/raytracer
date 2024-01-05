use crate::camera::Camera;
use crate::scenes::Scene;
use crate::{materials::Material, sphere::Sphere};
use std::f32::consts::PI;
use vek::{Rgb, Vec3};

#[allow(dead_code)]
pub fn scene_2() -> Scene {
    let camera = Camera {
        position: Vec3::new(13., 2., 3.),
        target: Vec3::new(0., 0., 0.),
        up: Vec3::new(0., 1., 0.),

        vertical_fov: (20_f32).to_radians(),
        defocus_angle: (0.6_f32).to_radians(),
        focus_distance: 10.,
    };

    let radius = f32::cos(PI / 4.);

    let spheres = vec![
        Sphere {
            center: Vec3::new(-radius, 0., -1.),
            radius,
            material: Material::Diffuse {
                albedo: Rgb::new(0., 0., 1.),
            },
        },
        Sphere {
            center: Vec3::new(radius, 0., -1.),
            radius,
            material: Material::Diffuse {
                albedo: Rgb::new(1., 0., 0.),
            },
        },
    ];

    Scene { camera, spheres }
}
