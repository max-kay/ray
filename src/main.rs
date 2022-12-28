use parry3d::math::{Point, Vector};
use parry3d::shape::Cuboid;
use parry3d::{math::Isometry, shape::Ball};
use ray::camera::Camera;
use ray::objects::{Brdf, Object, Scene, Shape};
use std::f32::consts::PI;
#[allow(dead_code, unused_variables, unused_mut)]

fn main() {
    let mut axes = vec![
        Object {
            shape: Shape::Ball(Ball { radius: 1.0 }),
            isometry: Isometry::translation(0.0, 0.0, 0.0),
            brdf: Brdf::One,
            color: (0.8, 0.8, 0.8).into(),
            is_light_source: false,
        },
        Object {
            shape: Shape::Ball(Ball { radius: 1.0 }),
            isometry: Isometry::translation(4.0, 0.0, 0.0),
            brdf: Brdf::One,
            color: (1.0, 0.0, 0.0).into(),
            is_light_source: false,
        },
        Object {
            shape: Shape::Ball(Ball { radius: 1.0 }),
            isometry: Isometry::translation(0.0, 4.0, 0.0),
            brdf: Brdf::One,
            color: (0.0, 1.0, 0.0).into(),
            is_light_source: false,
        },
        Object {
            shape: Shape::Ball(Ball { radius: 1.0 }),
            isometry: Isometry::translation(0.0, 0.0, 4.0),
            brdf: Brdf::One,
            color: (0.0, 0.0, 1.0).into(),
            is_light_source: false,
        },
        Object {
            shape: Shape::Ball(Ball { radius: 1.0 }),
            isometry: Isometry::translation(0.0, 0.0, 8.0),
            brdf: Brdf::One,
            color: (0.5, 0.8, 0.2).into(),
            is_light_source: false,
        },
    ];

    let mut objects = vec![
        Object {
            shape: Shape::Ball(Ball { radius: 3.0 }),
            isometry: Isometry::translation(0.0, 0.0, 0.0),
            brdf: Brdf::One,
            color: (0.9, 0.8, 0.2).into(),
            is_light_source: false,
        },
        Object {
            shape: Shape::Ball(Ball { radius: 3.0 }),
            isometry: Isometry::translation(10.0, 0.0, 0.0),
            brdf: Brdf::One,
            color: (0.1, 0.8, 0.9).into(),
            is_light_source: false,
        },
        Object {
            shape: Shape::Ball(Ball { radius: 3.0 }),
            isometry: Isometry::translation(5.0, 3.0, 11.0),
            brdf: Brdf::One,
            color: (0.8, 0.1, 0.8).into(),
            is_light_source: false,
        },
        Object {
            shape: Shape::Ball(Ball { radius: 3.0 }),
            isometry: Isometry::translation(5.0, 3.0, -8.0),
            brdf: Brdf::One,
            color: (0.8, 0.2, 0.8).into(),
            is_light_source: false,
        },
    ];

    let mut lights = vec![
        Object {
            shape: Shape::Ball(Ball { radius: 10.0 }),
            isometry: Isometry::translation(10.0, 0.0, 40.0),
            brdf: Brdf::One,
            color: (10.0, 15.0, 15.0).into(),
            is_light_source: true,
        },
        Object {
            shape: Shape::Ball(Ball { radius: 12.0 }),
            isometry: Isometry::translation(0.0, 40.0, -30.0),
            brdf: Brdf::One,
            color: (15.0, 12.0, 10.0).into(),
            is_light_source: true,
        },
    ];

    lights.append(&mut axes);

    lights.push(Object {
        shape: Shape::Cuboid(Cuboid::new(Vector::new(8.0, 8.0, 8.0))),
        isometry: Isometry::translation(0.0, 0.0, -9.0),
        brdf: Brdf::One,
        color: (0.8, 0.8, 0.8).into(),
        is_light_source: false,
    });

    let scene = Scene::new(lights, (0.0, 0.0, 0.0).into());
    let mut camera = Camera::face_towards(
        Point::new(20.0, 20.0, 20.0),
        Point::new(0.0, 0.0, 0.0),
        PI / 4.0,
        400,
        350,
    );

    scene.render(&mut camera, 3, 200);

    camera.save_img("./out/out.png").unwrap();
}
