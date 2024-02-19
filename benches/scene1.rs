use itertools::Itertools;
use parry3d::{
    math::{Isometry, Point, Vector},
    shape::{Ball, Cuboid, RoundCuboid},
};
use ray::{
    camera::Camera,
    objects::{Brdf, Color, Object, Scene, Shape},
};
use std::{f32::consts::TAU, time::Instant};

fn main() {
    println!("Scene 1: balls and round cuboid");
    let scene = make_scene();
    for input in (0..2)
        .map(|val| 1600 * (2_usize.pow(val)))
        .cartesian_product((0..3).map(|x| 3 * x + 3))
    {
        let mut camera = Camera::face_towards(
            Point::new(20.0, 20.0, 20.0),
            Point::new(0.0, 0.0, 0.0),
            TAU / 8.0,
            400,
            400,
        );

        let now = Instant::now();
        scene.render_par(&mut camera, input.0, input.1);
        println!(
            "{} rays, {} reflections, took: {:.2?}",
            input.0,
            input.1,
            now.elapsed()
        );

        camera
            .save_img(format!(
                "./out/scene_1_{:?}_rays_{:?}_reflections.png",
                input.0, input.1,
            ))
            .expect("failed to save");
    }
}

fn make_scene() -> Scene {
    Scene::new(Color(0.0, 0.0, 0.0))
        .add_objects(vec![
            Object {
                shape: Shape::Ball(Ball { radius: 1.0 }),
                isometry: Isometry::translation(0.0, 0.0, 0.0),
                brdf: Brdf::One,
                color: Color(0.8, 0.8, 0.8),
                is_light_source: false,
            },
            Object {
                shape: Shape::Ball(Ball { radius: 1.0 }),
                isometry: Isometry::translation(4.0, 0.0, 0.0),
                brdf: Brdf::One,
                color: Color(1.0, 0.0, 0.0),
                is_light_source: false,
            },
            Object {
                shape: Shape::Ball(Ball { radius: 1.0 }),
                isometry: Isometry::translation(0.0, 4.0, 0.0),
                brdf: Brdf::One,
                color: Color(0.0, 1.0, 0.0),
                is_light_source: false,
            },
            Object {
                shape: Shape::Ball(Ball { radius: 1.0 }),
                isometry: Isometry::translation(0.0, 0.0, 4.0),
                brdf: Brdf::One,
                color: Color(0.0, 0.0, 1.0),
                is_light_source: false,
            },
            Object {
                shape: Shape::Ball(Ball { radius: 1.0 }),
                isometry: Isometry::translation(0.0, 0.0, 8.0),
                brdf: Brdf::One,
                color: Color(0.5, 0.8, 0.2),
                is_light_source: false,
            },
        ])
        .add_objects(vec![
            Object {
                shape: Shape::Ball(Ball { radius: 10.0 }),
                isometry: Isometry::translation(10.0, 0.0, 40.0),
                brdf: Brdf::One,
                color: Color(20.0, 30.0, 30.0),
                is_light_source: true,
            },
            Object {
                shape: Shape::Ball(Ball { radius: 12.0 }),
                isometry: Isometry::translation(0.0, 40.0, -30.0),
                brdf: Brdf::One,
                color: Color(30.0, 24.0, 20.0),
                is_light_source: true,
            },
        ])
        .add_objects(vec![Object {
            shape: Shape::RoundCuboid(RoundCuboid {
                inner_shape: Cuboid::new(Vector::new(6.0, 6.0, 6.0)),
                border_radius: 0.75,
            }),
            isometry: Isometry::translation(0.0, 0.0, -8.0),
            brdf: Brdf::One,
            color: Color(0.8, 0.8, 0.8),
            is_light_source: false,
        }])
}
