use crate::camera::Camera;
use crate::materials::Texture;
use crate::scenes::Scene;
use crate::{materials::Material, sphere::Sphere};
use vek::{Rgb, Vec3};

#[allow(dead_code)]
pub fn scene_1() -> Scene {
    let camera = Camera {
        position: Vec3::new(0., 0., 0.),
        target: Vec3::new(0., 0., -1.),
        up: Vec3::new(0., 1., 0.),

        vertical_fov: (90_f32).to_radians(),
        defocus_angle: (0_f32).to_radians(),
        focus_distance: 1.,
    };

    let spheres = vec![
        Sphere {
            center: Vec3::new(0., -100.5, -1.),
            radius: 100.,
            material: Material::Diffuse {
                albedo: Texture::solid(Rgb::new(0.8, 0.8, 0.)),
            },
        },
        Sphere {
            center: Vec3::new(0., 0., -1.),
            radius: 0.5,
            material: Material::Diffuse {
                albedo: Texture::solid(Rgb::new(0.1, 0.2, 0.5)),
            },
        },
        Sphere {
            center: Vec3::new(-1., 0., -1.),
            radius: 0.5,
            material: Material::Glass {
                refraction_index: 1.5,
            },
        },
        Sphere {
            center: Vec3::new(-1., 0., -1.),
            radius: -0.4,
            material: Material::Glass {
                refraction_index: 1.5,
            },
        },
        Sphere {
            center: Vec3::new(1., 0., -1.),
            radius: 0.5,
            material: Material::Metal {
                albedo: Texture::solid(Rgb::new(0.8, 0.6, 0.2)),
                fuzz: 0.1,
            },
        },
    ];

    Scene { camera, spheres }
}
