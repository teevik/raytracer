use raytracer::camera::Camera;
use raytracer::materials::Material;
use raytracer::shapes::sphere::Sphere;
use raytracer::texture::Texture;
use raytracer::{render_image, Scene};
use std::sync::Arc;
use vek::{Rgb, Vec3};

fn main() {
    let camera = Camera {
        position: Vec3::new(0., 0., 12.),
        target: Vec3::new(0., 0., 0.),
        up: Vec3::new(0., 1., 0.),

        background_color: Rgb::new(0.7, 0.8, 1.),
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

    let spheres = vec![Sphere::new(Vec3::new(0., 0., 0.), 2., earth_material)];

    let scene = Scene {
        camera,
        spheres,
        ..Default::default()
    };
    let image = render_image(scene);
    image.save("image.png").unwrap();
}
