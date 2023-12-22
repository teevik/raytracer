use crate::{extensions::RgbExt, materials::Material, sphere::Sphere, world::World};
use rand::{thread_rng, Rng};
use vek::{Rgb, Vec3};

#[allow(dead_code)]
pub fn scene_3() -> World {
    let mut world = Vec::new();

    // Ground
    world.push(Sphere {
        center: Vec3::new(0., -1000., 0.),
        radius: 1000.,
        material: Material::Lambertian {
            albedo: Rgb::new(0.5, 0.5, 0.5),
        },
    });

    // Center sphere
    world.push(Sphere {
        center: Vec3::new(0., 1., 0.),
        radius: 1.,
        material: Material::Dialectric {
            refraction_index: 1.5,
        },
    });

    // Left sphere
    world.push(Sphere {
        center: Vec3::new(-4., 1., 0.),
        radius: 1.,
        material: Material::Lambertian {
            albedo: Rgb::new(0.4, 0.2, 0.1),
        },
    });

    // Right sphere
    world.push(Sphere {
        center: Vec3::new(4., 1., 0.),
        radius: 1.,
        material: Material::Metal {
            albedo: Rgb::new(0.7, 0.6, 0.5),
            fuzz: 0.,
        },
    });

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

                    world.push(Sphere {
                        center,
                        radius: 0.2,
                        material: Material::Lambertian { albedo },
                    });
                } else if choose_material < 0.95 {
                    // Metal
                    let mut random = || rng.gen_range(0.5..1.);
                    let albedo = Rgb::new(random(), random(), random());
                    let fuzz = rng.gen_range(0. ..0.5);

                    world.push(Sphere {
                        center,
                        radius: 0.2,
                        material: Material::Metal { albedo, fuzz },
                    });
                } else {
                    world.push(Sphere {
                        center,
                        radius: 0.2,
                        material: Material::Dialectric {
                            refraction_index: 1.5,
                        },
                    });
                }
            }
        }
    }

    World(world)
}
