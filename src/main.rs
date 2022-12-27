use std::f32::consts::PI;

use parry3d::math::Point;
use parry3d::{math::Isometry, shape::Ball};
use ray::camera::Camera;
use ray::materials::Material;
use ray::objects::{Object, Scene};

fn main() {
    tracing_subscriber::fmt::init();
    let mut axes = vec![
        Object {
            shape: Ball { radius: 1.0 },
            isometry: Isometry::translation(0.0, 0.0, 0.0),
            material: Material::new_light((0.5, 0.8, 0.2).into()),
        },
        Object {
            shape: Ball { radius: 1.0 },
            isometry: Isometry::translation(4.0, 0.0, 0.0),
            material: Material::new_light((0.5, 0.8, 0.2).into()),
        },
        Object {
            shape: Ball { radius: 1.0 },
            isometry: Isometry::translation(0.0, 4.0, 0.0),
            material: Material::new_light((0.5, 0.8, 0.2).into()),
        },
        Object {
            shape: Ball { radius: 1.0 },
            isometry: Isometry::translation(0.0, 0.0, 4.0),
            material: Material::new_light((0.5, 0.8, 0.2).into()),
        },
        Object {
            shape: Ball { radius: 1.0 },
            isometry: Isometry::translation(0.0, 0.0, 8.0),
            material: Material::new_light((0.5, 0.8, 0.2).into()),
        },
    ];

    let mut objects = vec![
        Object {
            shape: Ball { radius: 3.0 },
            isometry: Isometry::translation(0.0, 0.0, 0.0),
            material: Material::new_lambertian((0.9, 0.8, 0.2).into()),
        },
        Object {
            shape: Ball { radius: 3.0 },
            isometry: Isometry::translation(10.0, 0.0, 0.0),
            material: Material::new_lambertian((0.1, 0.8, 0.9).into()),
        },
        Object {
            shape: Ball { radius: 3.0 },
            isometry: Isometry::translation(5.0, 3.0, 11.0),
            material: Material::new_lambertian((0.8, 0.1, 0.8).into()),
        },
        Object {
            shape: Ball { radius: 3.0 },
            isometry: Isometry::translation(5.0, 3.0, -8.0),
            material: Material::new_lambertian((0.8, 0.2, 0.8).into()),
        },
        Object {
            shape: Ball { radius: 10.0 },
            isometry: Isometry::translation(10.0, 0.0, 40.0),
            material: Material::new_light((0.7, 1.0, 1.0).into()),
        },
        Object {
            shape: Ball { radius: 12.0 },
            isometry: Isometry::translation(0.0, 40.0, -30.0),
            material: Material::new_light((1.0, 0.8, 0.6).into()),
        },
    ];

    // objects.append(&mut axes);

    let scene = Scene::new(objects, (0.0, 0.0, 0.0).into());
    let width = 640;
    let height = 960;
    let mut camera = Camera::face_towards(
        Point::new(20.0, 20.0, 20.0),
        Point::new(3.0, 0.0, 2.5),
        PI / 4.0,
        width,
        height,
    );

    scene.render(&mut camera, (width * height) as usize / 14, 3, 500);

    camera.save_img("./out/out.png").unwrap();
}
