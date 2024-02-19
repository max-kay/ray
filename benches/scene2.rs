use parry3d::math::Point;
use parry3d::{math::Isometry, shape::Ball};
use ray::camera::Camera;
use ray::objects::{Brdf, Object, Scene, Shape, Color};
use std::f32::consts::TAU;
use std::time::Instant;

fn main() {
    let scene = make_scene();

    let mut camera = Camera::face_towards(
        Point::new(20.0, 20.0, 20.0),
        Point::new(4.0, 0.0, 3.0),
        TAU / 8.0,
        1600,
        2400,
    );

    println!("Scene 2: Balls");
    let start = Instant::now();
    scene.render_par(&mut camera, 3200, 10);
    println!("took {:.2?}", start.elapsed());
    camera.apply_blur(1.5);
    camera.save_img("./out/scene_2.png").unwrap();
}

fn make_scene() -> Scene {
    Scene::new(Color(0.0, 0.0, 0.0))
        // Balls
        .add_objects(vec![
            Object {
                shape: Shape::Ball(Ball { radius: 3.0 }),
                isometry: Isometry::translation(0.0, 0.0, 0.0),
                brdf: Brdf::One,
                color: Color(0.9, 0.9, 0.9),
                is_light_source: false,
            },
            Object {
                shape: Shape::Ball(Ball { radius: 3.0 }),
                isometry: Isometry::translation(10.0, 0.0, 0.0),
                brdf: Brdf::One,
                color: Color(0.1, 0.8, 0.9),
                is_light_source: false,
            },
            Object {
                shape: Shape::Ball(Ball { radius: 3.0 }),
                isometry: Isometry::translation(5.0, 3.0, 11.0),
                brdf: Brdf::One,
                color: Color(0.8, 0.1, 0.8),
                is_light_source: false,
            },
            Object {
                shape: Shape::Ball(Ball { radius: 3.0 }),
                isometry: Isometry::translation(5.0, 3.0, -8.0),
                brdf: Brdf::One,
                color: Color(0.8, 0.2, 0.8),
                is_light_source: false,
            },
        ])
        // Lights
        .add_objects(vec![
            Object {
                shape: Shape::Ball(Ball { radius: 10.0 }),
                isometry: Isometry::translation(10.0, 0.0, 40.0),
                brdf: Brdf::One,
                color: Color(25.0, 25.0, 25.0),
                is_light_source: true,
            },
            Object {
                shape: Shape::Ball(Ball { radius: 12.0 }),
                isometry: Isometry::translation(0.0, 40.0, -30.0),
                brdf: Brdf::One,
                color: Color(30.0, 24.0, 15.0),
                is_light_source: true,
            },
        ])
}
