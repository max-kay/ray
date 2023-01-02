use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use itertools::Itertools;
use parry3d::{
    math::{Isometry, Point, Vector},
    shape::{Ball, Cuboid, RoundCuboid},
};
use ray::{
    camera::Camera,
    objects::{Brdf, Object, Scene, Shape},
};
use std::f32::consts::TAU;

fn render(rays: usize, reflections: usize) {
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

    let mut lights = vec![
        Object {
            shape: Shape::Ball(Ball { radius: 10.0 }),
            isometry: Isometry::translation(10.0, 0.0, 40.0),
            brdf: Brdf::One,
            color: (20.0, 30.0, 30.0).into(),
            is_light_source: true,
        },
        Object {
            shape: Shape::Ball(Ball { radius: 12.0 }),
            isometry: Isometry::translation(0.0, 40.0, -30.0),
            brdf: Brdf::One,
            color: (30.0, 24.0, 20.0).into(),
            is_light_source: true,
        },
    ];

    lights.append(&mut axes);

    lights.push(Object {
        shape: Shape::RoundCuboid(RoundCuboid {
            inner_shape: Cuboid::new(Vector::new(6.0, 6.0, 6.0)),
            border_radius: 0.75,
        }),
        isometry: Isometry::translation(0.0, 0.0, -8.0),
        brdf: Brdf::One,
        color: (0.8, 0.8, 0.8).into(),
        is_light_source: false,
    });

    let scene = Scene::new(lights);

    let mut camera = Camera::face_towards(
        Point::new(20.0, 20.0, 20.0),
        Point::new(0.0, 0.0, 0.0),
        TAU / 8.0,
        400,
        400,
    );
    scene.render(&mut camera, rays, reflections);
    camera
        .save_img(format!(
            "./out/out_{rays:?}_rays_{reflections:?}_reflections.png"
        ))
        .expect("failed to save");
}

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("rays");
    let group = group.sample_size(10);
    for input in (0..2)
        .map(|val| 1600 * (2_usize.pow(2 * val)))
        .cartesian_product((0..3).map(|x| 3 * x + 3))
    {
        group.bench_with_input(
            BenchmarkId::new(
                "scene1",
                format!("reflections: {}, rays: {}", input.1, input.0),
            ),
            &input,
            |b, input| b.iter(|| render(input.0, input.1)),
        );
    }
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
