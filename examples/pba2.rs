use std::{f32::consts::TAU, time::Instant};

use parry3d::{
    math::{Isometry, Point},
    shape::{Ball, Capsule, Segment},
};
use ray::{
    camera::Camera,
    objects::{Color, Object, Scene, Shape},
};

const BALL_RAD: f32 = 0.2;
const RESOLUTION: u32 = 1200;
const MN_COLOR: Color = Color(0.37, 0.4, 0.82);
const CO_COLOR: Color = Color(0.75, 0.08, 0.77);

fn main() {
    let co_color = CO_COLOR;
    let balls = vec![
        Object {
            shape: Shape::Ball(Ball { radius: BALL_RAD }),
            isometry: Isometry::translation(0.0, 0.0, 1.0),
            brdf: ray::objects::Brdf::One,
            color: co_color,
            is_light_source: false,
        },
        Object {
            shape: Shape::Ball(Ball { radius: BALL_RAD }),
            isometry: Isometry::translation(0.0, 0.0, -1.0),
            brdf: ray::objects::Brdf::One,
            color: co_color,
            is_light_source: false,
        },
        Object {
            shape: Shape::Ball(Ball { radius: BALL_RAD }),
            isometry: Isometry::translation(0.0, 1.0, 0.0),
            brdf: ray::objects::Brdf::One,
            color: co_color,
            is_light_source: false,
        },
        Object {
            shape: Shape::Ball(Ball { radius: BALL_RAD }),
            isometry: Isometry::translation(0.0, -1.0, 0.0),
            brdf: ray::objects::Brdf::One,
            color: co_color,
            is_light_source: false,
        },
        Object {
            shape: Shape::Ball(Ball { radius: BALL_RAD }),
            isometry: Isometry::translation(1.0, 0.0, 0.0),
            brdf: ray::objects::Brdf::One,
            color: co_color,
            is_light_source: false,
        },
        Object {
            shape: Shape::Ball(Ball { radius: BALL_RAD }),
            isometry: Isometry::translation(-1.0, 0.0, 0.0),
            brdf: ray::objects::Brdf::One,
            color: co_color,
            is_light_source: false,
        },
    ];

    let lights = vec![
        Object {
            shape: Shape::Ball(Ball { radius: 5.0 }),
            isometry: Isometry::translation(10.0, 10.0, 10.0),
            brdf: ray::objects::Brdf::One,
            color: Color(14.0, 14.0, 14.0),
            is_light_source: true,
        },
        Object {
            shape: Shape::Ball(Ball { radius: 5.0 }),
            isometry: Isometry::translation(10.0, 10.0, 30.0),
            brdf: ray::objects::Brdf::One,
            color: Color(10.0, 9.0, 9.0),
            is_light_source: true,
        },
    ];

    let scene = Scene::new(Color(1.0, 1.0, 1.0))
        .add_object(Object {
            shape: Shape::Ball(Ball { radius: BALL_RAD }),
            isometry: Isometry::identity(),
            brdf: ray::objects::Brdf::One,
            color: MN_COLOR,
            is_light_source: false,
        })
        .add_objects(balls)
        .add_objects(lights)
        .add_object(Object {
            shape: Shape::Capsule(Capsule {
                segment: Segment::new([1.0, 0.0, 0.0].into(), [0.0, 0.0, 1.0].into()),
                radius: 0.05,
            }),
            isometry: Isometry::identity(),
            brdf: ray::objects::Brdf::One,
            color: Color(0.8, 0.8, 0.8),
            is_light_source: false,
        });

    let mut camera = Camera::face_towards(
        Point::new(4.5, 5.9, 2.6),
        Point::new(0.0, 0.0, 0.0),
        TAU / 12.0,
        RESOLUTION,
        RESOLUTION,
    );
    let now = Instant::now();
    scene.render_par(&mut camera, 3000, 4);
    println!("took: {:.2?}", now.elapsed());

    camera.save_img("./out/shwo.png").expect("failed to save");
}
