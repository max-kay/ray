use parry3d::math::{Point, Vector};
use parry3d::shape::{Capsule, Cuboid, Segment};
use parry3d::{math::Isometry, shape::Ball};
use ray::camera::Camera;
use ray::objects::{Brdf, Object, Scene, Shape, Color};
use std::f32::consts::TAU;
use std::time::Instant;

fn main() {
    let scene = make_scene();

    let mut camera = Camera::face_towards(
        Point::new(13.0, 20.0, 5.0),
        Point::new(4.0, 0.0, 8.0),
        TAU / 5.0,
        400,
        400,
    );

    println!("Scene 3: Room");
    let start = Instant::now();
    scene.render_par_with_progress(&mut camera, 800, 4);
    println!("took {:.2?}", start.elapsed());

    camera.save_img("./out/scene_3.png").unwrap();
    camera.apply_blur(1.5);
    camera.save_img("./out/scene_3_blured.png").unwrap();
}

fn make_scene() -> Scene {
    Scene::new(Color(0.0, 0.0, 0.0))
        .add_objects(vec![
            Object {
                shape: Shape::Cuboid(Cuboid::new(Vector::new(2.0, 10.0, 10.0))),
                isometry: Isometry::translation(-2.0, 10.0, 10.0),
                brdf: Brdf::One,
                color: Color(0.3, 0.9, 0.9),
                is_light_source: false,
            },
            Object {
                shape: Shape::Cuboid(Cuboid::new(Vector::new(10.0, 2.0, 10.0))),
                isometry: Isometry::translation(10.0, -2.0, 10.0),
                brdf: Brdf::One,
                color: Color(0.9, 0.3, 0.9),
                is_light_source: false,
            },
            Object {
                shape: Shape::Cuboid(Cuboid::new(Vector::new(10.0, 10.0, 2.0))),
                isometry: Isometry::translation(10.0, 10.0, -2.0),
                brdf: Brdf::One,
                color: Color(0.9, 0.9, 0.3),
                is_light_source: false,
            },
        ])
        .add_objects(vec![
            Object {
                shape: Shape::Ball(Ball::new(1.0)),
                isometry: Isometry::translation(3.5, 6.0, 1.0),
                brdf: Brdf::One,
                color: Color(0.9, 0.8, 0.2),
                is_light_source: false,
            },
            Object {
                shape: Shape::Cuboid(Cuboid::new(Vector::new(3.0, 2.0, 3.5))),
                isometry: Isometry::translation(7.0, 4.0, 0.0),
                brdf: Brdf::One,
                color: Color(0.9, 0.2, 0.3),
                is_light_source: false,
            },
        ])
        .add_object(Object {
            shape: Shape::Capsule(Capsule {
                segment: Segment {
                    a: Point::new(4.0, 0.2, 10.0),
                    b: Point::new(11.0, 0.2, 10.0),
                },
                radius: 0.2,
            }),
            isometry: Isometry::identity(),
            brdf: Brdf::One,
            color: Color(1.0, 1.0, 0.5).scale(100.0),
            is_light_source: true,
        })
        // .add_object(Object {
        //     shape: Shape::Cuboid(Cuboid::new(Vector::new(5.0, 0.2, 0.2))),
        //     isometry: Isometry::translation(10.0, 3.0, 10.0),
        //     brdf: Brdf::One,
        //     color: (20.0, 20.0, 9.0).into(),
        //     is_light_source: true,
        // })
}
