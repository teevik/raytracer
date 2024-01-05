use crate::camera::Camera;
use crate::scenes::Scene;
use crate::{extensions::RgbExt, materials::Material, sphere::Sphere};
use rand::{thread_rng, Rng};
use vek::{Rgb, Vec3};

#[allow(dead_code)]
pub fn scene_3() -> Scene {
    let camera = Camera {
        position: Vec3::new(13., 2., 3.),
        target: Vec3::new(0., 0., 0.),
        up: Vec3::new(0., 1., 0.),

        vertical_fov: (20_f32).to_radians(),
        defocus_angle: (0_f32).to_radians(),
        focus_distance: 10.,
    };

    let mut spheres = vec![
        // Ground
        Sphere {
            center: Vec3::new(0., -1000., 0.),
            radius: 1000.,
            material: Material::Diffuse {
                albedo: Rgb::new(0.5, 0.5, 0.5),
            },
        },
        // Center sphere
        Sphere {
            center: Vec3::new(0., 1., 0.),
            radius: 1.,
            material: Material::Glass {
                refraction_index: 1.5,
            },
        },
        // Left sphere
        Sphere {
            center: Vec3::new(-4., 1., 0.),
            radius: 1.,
            material: Material::Diffuse {
                albedo: Rgb::new(0.4, 0.2, 0.1),
            },
        },
        // Right sphere
        Sphere {
            center: Vec3::new(4., 1., 0.),
            radius: 1.,
            material: Material::Metal {
                albedo: Rgb::new(0.7, 0.6, 0.5),
                fuzz: 0.,
            },
        },
    ];

    let rng = &mut thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let choose_material = rng.gen::<f32>();

            let center = Vec3::new(
                a as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.gen::<f32>(),
            );
            if center.distance(Vec3::new(4., 0.2, 0.)) > 0.9 {
                if choose_material < 0.8 {
                    // Diffuse
                    let albedo = Rgb::random(rng) * Rgb::random(rng);

                    spheres.push(Sphere {
                        center,
                        radius: 0.2,
                        material: Material::Diffuse { albedo },
                    });
                } else if choose_material < 0.95 {
                    // Metal
                    let mut random = || rng.gen_range(0.5..1.);
                    let albedo = Rgb::new(random(), random(), random());
                    let fuzz = rng.gen_range(0. ..0.5);

                    spheres.push(Sphere {
                        center,
                        radius: 0.2,
                        material: Material::Metal { albedo, fuzz },
                    });
                } else {
                    spheres.push(Sphere {
                        center,
                        radius: 0.2,
                        material: Material::Glass {
                            refraction_index: 1.5,
                        },
                    });
                }
            }
        }
    }

    Scene { camera, spheres }
}
