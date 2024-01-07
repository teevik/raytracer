use rand::{thread_rng, Rng};
use raytracer::camera::Camera;
use raytracer::extensions::RngExtension;
use raytracer::materials::Material;
use raytracer::shapes::sphere::Sphere;
use raytracer::texture::Texture;
use raytracer::{render_image, Scene};
use vek::{Rgb, Vec3};

fn main() {
    let camera = Camera {
        position: Vec3::new(13., 2., 3.),
        target: Vec3::new(0., 0., 0.),
        up: Vec3::new(0., 1., 0.),

        background_color: Rgb::new(0.7, 0.8, 1.),
        vertical_fov: (20_f32).to_radians(),
        defocus_angle: (0_f32).to_radians(),
        focus_distance: 10.,
    };

    let mut spheres = vec![
        // Ground
        Sphere::new(
            Vec3::new(0., -1000., 0.),
            1000.,
            Material::Diffuse {
                // albedo: Texture::solid(Rgb::new(0.5, 0.5, 0.5)),
                albedo: Texture::checker(Rgb::new(0.2, 0.3, 0.1), Rgb::new(0.9, 0.9, 0.9), 0.32),
            },
        ),
        // Center sphere
        Sphere::new(
            Vec3::new(0., 1., 0.),
            1.,
            Material::Glass {
                refraction_index: 1.5,
            },
        ),
        // Left sphere
        Sphere::new(
            Vec3::new(-4., 1., 0.),
            1.,
            Material::Diffuse {
                albedo: Texture::solid(Rgb::new(0.4, 0.2, 0.1)),
            },
        ),
        // Right sphere
        Sphere::new(
            Vec3::new(4., 1., 0.),
            1.,
            Material::Metal {
                albedo: Texture::solid(Rgb::new(0.7, 0.6, 0.5)),
                fuzz: 0.,
            },
        ),
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
                    let albedo = Texture::solid(rng.random_color() * rng.random_color());

                    spheres.push(Sphere::new(center, 0.2, Material::Diffuse { albedo }));
                } else if choose_material < 0.95 {
                    // Metal
                    let mut random = || rng.gen_range(0.5..1.);
                    let albedo = Texture::solid(Rgb::new(random(), random(), random()));
                    let fuzz = rng.gen_range(0. ..0.5);

                    spheres.push(Sphere::new(center, 0.2, Material::Metal { albedo, fuzz }));
                } else {
                    spheres.push(Sphere::new(
                        center,
                        0.2,
                        Material::Glass {
                            refraction_index: 1.5,
                        },
                    ));
                }
            }
        }
    }

    let scene = Scene {
        camera,
        spheres,
        ..Default::default()
    };
    let image = render_image(scene);
    image.save("image.png").unwrap();
}
