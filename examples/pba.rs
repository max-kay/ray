use parry3d::{
    math::{Isometry, Point, Vector},
    shape::Ball,
};

use ray::{
    camera::Camera,
    objects::{Brdf, Color, Object, Scene, Shape},
};
use std::{f32::consts::TAU, time::Instant};

const HALF_UNIT_CELLS: u32 = 3;
const RESOLUTION: u32 = 1200;

fn main() {
    println!("Prussian Blue Analog");
    let scene = make_scene();
    let center = (HALF_UNIT_CELLS - 1) as f32 * DIST_MN_MN / 4.0;
    let mut camera = Camera::face_towards(
        Point::new(30.0, 18.0, 21.0),
        Point::new(center, center, center),
        TAU / 10.0,
        RESOLUTION,
        RESOLUTION,
    );
    let now = Instant::now();
    scene.render_par(&mut camera, 3000, 4);
    println!("took: {:.2?}", now.elapsed());

    camera.save_img("./out/pba.png").expect("failed to save");
}

fn make_scene() -> Scene {
    let mut scene = Scene::new(Color(1.0, 1.0, 1.0));

    for x in 0..HALF_UNIT_CELLS {
        for y in 0..HALF_UNIT_CELLS {
            for z in 0..HALF_UNIT_CELLS {
                // if x == 2 && y == 1 && z == 2 {
                //     continue;
                // }
                // if x == 2 && y == 2 && z == 1 {
                //     continue;
                // }
                // if x == 1 && y == 1 && z == 1 {
                //     continue;
                // }
                // if (x, y, z) == (0, 0, 1) {
                //     continue;
                // }
                let position: Vector<f32> = [
                    x as f32 * DIST_MN_MN / 2.0,
                    y as f32 * DIST_MN_MN / 2.0,
                    z as f32 * DIST_MN_MN / 2.0,
                ]
                .into();
                if (x + y + z) % 2 == 0 {
                    scene = place_ion(scene, Ion::Mn, position)
                } else {
                    scene = place_ion(scene, Ion::Cyanocobaltate, position)
                }
            }
        }
    }

    scene = scene.add_objects(vec![
        Object {
            shape: Shape::Ball(Ball { radius: 10.0 }),
            isometry: Isometry::translation(30.0, 30.0, 20.0),
            brdf: Brdf::One,
            color: Color(15.0, 15.0, 15.0),
            is_light_source: true,
        },
        Object {
            shape: Shape::Ball(Ball { radius: 10.0 }),
            isometry: Isometry::translation(40.0, 0.0, 35.0),
            brdf: Brdf::One,
            color: Color(15.0, 15.0, 15.0),
            is_light_source: true,
        },
    ]);
    scene
}

const MN_RADIUS: f32 = 1.2;
const MN_COLOR: Color = Color(0.37, 0.4, 0.82);
const CO_RADIUS: f32 = 1.05;
const CO_COLOR: Color = Color(0.75, 0.08, 0.77);
const C_RADIUS: f32 = 0.76 * 0.9;
const C_COLOR: Color = Color(0.6, 0.06, 0.06);
const N_RADIUS: f32 = 0.71 * 0.9;
const N_COLOR: Color = Color(0.17, 0.67, 0.24);

const DIST_MN_MN: f32 = 10.0003;
const CO_C: f32 = 1.89;
const CO_N: f32 = 3.03;

fn place_ion(mut scene: Scene, ion: Ion, coord_armstrong: Vector<f32>) -> Scene {
    match ion {
        Ion::Mn => add_atom(scene, coord_armstrong, MN_COLOR, MN_RADIUS),
        Ion::Cyanocobaltate => {
            scene = add_atom(scene, coord_armstrong, CO_COLOR, CO_RADIUS);

            // find nicer way to generate these unit vecs
            let all_axis: Vec<Vector<f32>> = (0..3)
                .map(|i| {
                    let mut e = [0.0, 0.0, 0.0];
                    e[i] = 1.0;
                    e.into()
                })
                .collect();
            for dir in all_axis {
                let c_offset = dir.scale(CO_C);
                scene = add_atom(scene, coord_armstrong + c_offset, C_COLOR, C_RADIUS);
                scene = add_atom(scene, coord_armstrong - c_offset, C_COLOR, C_RADIUS);
                let n_offset = dir.scale(CO_N);
                scene = add_atom(scene, coord_armstrong + n_offset, N_COLOR, N_RADIUS);
                scene = add_atom(scene, coord_armstrong - n_offset, N_COLOR, N_RADIUS);
            }
            scene
        }
    }
}

fn add_atom(scene: Scene, coord_armstrong: Vector<f32>, color: Color, radius: f32) -> Scene {
    scene.add_object(Object {
        shape: Shape::Ball(Ball { radius }),
        isometry: Isometry::translation(coord_armstrong.x, coord_armstrong.y, coord_armstrong.z),
        brdf: Brdf::One,
        color,
        is_light_source: false,
    })
}

/// A type for Ions
#[derive(Clone, Copy)]
pub enum Ion {
    Mn,
    Cyanocobaltate,
}
