use crate::camera::Camera;
use crate::materials::Texture;
use crate::scenes::Scene;
use crate::{materials::Material, sphere::Sphere};
use vek::{Rgb, Vec3};

#[allow(dead_code)]
pub fn scene_4() -> Scene {
    let camera = Camera {
        position: Vec3::new(13., 2., 3.),
        target: Vec3::new(0., 0., 0.),
        up: Vec3::new(0., 1., 0.),

        vertical_fov: (20_f32).to_radians(),
        defocus_angle: (0_f32).to_radians(),
        focus_distance: 1.,
    };

    let checker = Material::Diffuse {
        albedo: Texture::checker(Rgb::new(0.2, 0.3, 0.1), Rgb::new(0.9, 0.9, 0.9), 0.4),
    };

    let spheres = vec![
        Sphere {
            center: Vec3::new(0., -10., 0.),
            radius: 10.,
            material: checker.clone(),
        },
        Sphere {
            center: Vec3::new(0., 10., 0.),
            radius: 10.,
            material: checker,
        },
    ];

    Scene { camera, spheres }
}
