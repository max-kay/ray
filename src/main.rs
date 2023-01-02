use parry3d::math::Point;
use parry3d::{math::Isometry, shape::Ball};
use ray::camera::Camera;
use ray::objects::{Brdf, Object, Scene, Shape};
use std::f32::consts::TAU;

fn main() {
    let mut objects = vec![
        Object {
            shape: Shape::Ball(Ball { radius: 3.0 }),
            isometry: Isometry::translation(0.0, 0.0, 0.0),
            brdf: Brdf::One,
            color: (0.9, 0.9, 0.9).into(),
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
            color: (25.0, 25.0, 25.0).into(),
            is_light_source: true,
        },
        Object {
            shape: Shape::Ball(Ball { radius: 12.0 }),
            isometry: Isometry::translation(0.0, 40.0, -30.0),
            brdf: Brdf::One,
            color: (30.0, 24.0, 15.0).into(),
            is_light_source: true,
        },
    ];

    lights.append(&mut objects);

    let scene = Scene::new(lights);
    let mut camera = Camera::face_towards(
        Point::new(20.0, 20.0, 20.0),
        Point::new(4.0, 0.0, 3.0),
        TAU / 8.0,
        1600,
        2400,
    );

    scene.render_with_progress(&mut camera, 3200, 10);

    camera.save_img("./out/out.png").unwrap();
}
