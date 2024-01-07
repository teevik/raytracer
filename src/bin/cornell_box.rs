use raytracer::camera::Camera;
use raytracer::materials::Material;
use raytracer::shapes::quad::Quad;
use raytracer::texture::Texture;
use raytracer::{render_image, Scene};
use vek::{Mat4, Rgb, Vec3};

fn make_box(
    a: Vec3<f32>,
    b: Vec3<f32>,
    model_matrix: Mat4<f32>,
    material: Material,
) -> impl Iterator<Item = Quad> {
    let min = a.map2(b, |a, b| f32::min(a, b));
    let max = a.map2(b, |a, b| f32::max(a, b));

    // let min = model_matrix.mul_point(min);
    // let max = model_matrix.mul_point(max);

    let dx = Vec3::new(max.x - min.x, 0., 0.);
    let dy = Vec3::new(0., max.y - min.y, 0.);
    let dz = Vec3::new(0., 0., max.z - min.z);

    // let dx = model_matrix.mul_point(dx);
    // let dy = model_matrix.mul_point(dy);
    // let dz = model_matrix.mul_point(dz);

    // let dx = model_matrix.mul_point(dx);
    // let dy = model_matrix.mul_point(dy);
    // let dz = model_matrix.mul_point(dz);

    let sides = [
        Quad::new(Vec3::new(min.x, min.y, max.z), dx, dy, material.clone()), // front
        Quad::new(Vec3::new(max.x, min.y, max.z), -dz, dy, material.clone()), // right
        Quad::new(Vec3::new(max.x, min.y, min.z), -dx, dy, material.clone()), // back
        Quad::new(Vec3::new(min.x, min.y, min.z), dz, dy, material.clone()), // left
        Quad::new(Vec3::new(min.x, max.y, max.z), dx, -dz, material.clone()), // top
        Quad::new(Vec3::new(min.x, min.y, min.z), dx, dz, material),         // bottom
    ]
    .map(
        |Quad {
             origin,
             u,
             v,
             normal,
             w,
             distance,
             bounding_box,
             material,
         }| {
            Quad::new(
                model_matrix.mul_point(origin),
                model_matrix.mul_direction(u),
                model_matrix.mul_direction(v),
                material,
            )
        },
    );

    sides.into_iter()
}

fn main() {
    let camera = Camera {
        position: Vec3::new(278., 278., -800.),
        target: Vec3::new(278., 278., 0.),
        up: Vec3::new(0., 1., 0.),

        background_color: Rgb::zero(),
        vertical_fov: (40_f32).to_radians(),
        defocus_angle: (0_f32).to_radians(),
        focus_distance: 10.,
    };

    let red_material = Material::Diffuse {
        albedo: Texture::solid(Rgb::new(0.65, 0.05, 0.05)),
    };

    let white_material = Material::Diffuse {
        albedo: Texture::solid(Rgb::new(0.73, 0.73, 0.73)),
    };

    let green_material = Material::Diffuse {
        albedo: Texture::solid(Rgb::new(0.12, 0.45, 0.15)),
    };

    let light_material = Material::DiffuseLight {
        strength: Texture::solid(Rgb::new(15., 15., 15.)),
    };

    let mut quads = vec![
        Quad::new(
            Vec3::new(555., 0., 0.),
            Vec3::new(0., 555., 0.),
            Vec3::new(0., 0., 555.),
            green_material,
        ),
        Quad::new(
            Vec3::new(0., 0., 0.),
            Vec3::new(0., 555., 0.),
            Vec3::new(0., 0., 555.),
            red_material,
        ),
        Quad::new(
            Vec3::new(343., 554., 332.),
            Vec3::new(-130., 0., 0.),
            Vec3::new(0., 0., -105.),
            light_material,
        ),
        Quad::new(
            Vec3::new(0., 0., 0.),
            Vec3::new(555., 0., 0.),
            Vec3::new(0., 0., 555.),
            white_material.clone(),
        ),
        Quad::new(
            Vec3::new(555., 555., 555.),
            Vec3::new(-555., 0., 0.),
            Vec3::new(0., 0., -555.),
            white_material.clone(),
        ),
        Quad::new(
            Vec3::new(0., 0., 555.),
            Vec3::new(555., 0., 0.),
            Vec3::new(0., 555., 0.),
            white_material.clone(),
        ),
    ];

    let model_matrix = Mat4::identity()
        .translated_3d(Vec3::new(205., 0., 295.))
        .rotated_y(15_f32.to_radians());

    quads.extend(make_box(
        Vec3::new(0., 0., 0.),
        Vec3::new(165., 330., 165.),
        model_matrix,
        white_material.clone(),
    ));

    let model_matrix = Mat4::identity()
        .translated_3d(Vec3::new(160., 0., 0.))
        .rotated_y(-18_f32.to_radians());

    quads.extend(make_box(
        Vec3::new(0., 0., 0.),
        Vec3::new(165., 165., 165.),
        model_matrix,
        white_material.clone(),
    ));

    let scene = Scene {
        camera,
        quads,
        ..Default::default()
    };

    let image = render_image(scene);
    image.save("image.png").unwrap();
}
