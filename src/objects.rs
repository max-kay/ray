use image::Rgb;
use indicatif::ProgressIterator;
use nalgebra::{Rotation3, UnitVector3};
use parry3d::{
    math::{Isometry, Real, Vector},
    query::{Ray, RayCast, RayIntersection},
    shape::Ball,
};
use rayon::prelude::ParallelSlice;
use rayon::{prelude::*, slice::ParallelSliceMut};

use crate::{
    camera::Camera,
    materials::{Color, Material},
    utils,
};

#[derive(Default)]
pub struct Scene {
    objects: Vec<Object<Ball>>,
    background: Color,
}

impl Scene {
    pub fn new(objects: Vec<Object<Ball>>, background: Color) -> Self {
        Self {
            objects,
            background,
        }
    }

    pub fn render(
        &self,
        camera: &mut Camera,
        chunksize: usize,
        recursion: usize,
        reflections: usize,
    ) {
        camera
            .get_rays()
            .par_chunks_mut(chunksize)
            .for_each(|slice| {
                slice.into_iter().for_each(|(ray, color)| {
                    **color = self.find_color(*ray, recursion, reflections)
                })
            });
    }

    pub fn find_color(&self, ray: Ray, recursion: usize, reflections: usize) -> Rgb<u8> {
        self.recursive_finder(&ray, recursion, reflections).into()
    }

    fn recursive_finder(&self, ray: &Ray, iteration: usize, reflections: usize) -> Color {
        if iteration == 0 {
            return self.background;
        }
        match self.closest_intersection(ray) {
            None => self.background,
            Some((idx, intersection)) => {
                let material = self.objects[idx].material;
                let intersection_point = ray.point_at(intersection.toi);
                if material.is_light_source() {
                    return material.get_color(intersection_point);
                }
                let mut color: Color = (0.0, 0.0, 0.0).into();
                let m = Rotation3::from_axis_angle(
                    &UnitVector3::<f32>::new_normalize(intersection.normal.cross(&Vector::z())),
                    -intersection.normal.angle(&Vector::z()),
                );
                for _ in 0..reflections {
                    let out_going = m * utils::rand_unit_vec();
                    let new_ray = Ray::new(intersection_point, out_going);
                    let factor = material.brdf(ray.dir, out_going, intersection.normal)
                        * (intersection.normal.dot(&out_going));
                    color += material.get_color(intersection_point).scale(factor)
                        * self.recursive_finder(&new_ray, iteration - 1, reflections);
                }
                color.scale(1.0 / reflections as f32)
            }
        }
    }

    fn closest_intersection(&self, ray: &Ray) -> Option<(usize, RayIntersection)> {
        self.objects
            .iter()
            .enumerate()
            .map(|(i, obj)| (i, obj.get_intersection(ray)))
            .filter_map(|(i, opt)| match opt {
                Some(inter) => {
                    if inter.toi > 0.0 {
                        Some((i, inter))
                    } else {
                        None
                    }
                }
                None => None,
            })
            .min_by(|a, b| a.1.toi.total_cmp(&b.1.toi))
    }
}

pub struct Object<T: RayCast> {
    pub shape: T,
    pub isometry: Isometry<f32>,
    pub material: Material,
}

impl<T: RayCast> Object<T> {
    pub fn get_intersection(&self, ray: &Ray) -> Option<RayIntersection> {
        self.shape
            .cast_ray_and_get_normal(&self.isometry, ray, Real::MAX, true)
    }
}
