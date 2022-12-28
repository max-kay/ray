use crate::{camera::Camera, utils};
use image::Rgb;
use indicatif::ParallelProgressIterator;
use nalgebra::{Rotation3, UnitVector3};
use parry3d::{
    math::{Isometry, Real, Vector},
    query::{Ray, RayCast, RayIntersection},
    shape::{Ball, Cuboid},
};
use rayon::prelude::*;
use std::ops::{Add, AddAssign, Mul};

#[derive(Default)]
pub struct Scene {
    objects: Vec<Object>,
    background: Color,
}

impl Scene {
    pub fn new(objects: Vec<Object>, background: Color) -> Self {
        Self {
            objects,
            background,
        }
    }

    pub fn render(&self, camera: &mut Camera, recursion: usize, reflections: usize) {
        camera
            .get_rays()
            .into_par_iter()
            .progress()
            .for_each(|(ray, color)| {
                *color = self.render_pixel(&ray, recursion, reflections).into()
            });
    }

    fn render_pixel(&self, ray: &Ray, iteration: usize, reflections: usize) -> Color {
        if iteration == 0 {
            return self.background;
        }
        match self.closest_intersection(ray) {
            None => self.background,
            Some((idx, intersection)) => {
                let object = &self.objects[idx];
                let intersection_point = ray.point_at(intersection.toi);
                if object.is_light_source {
                    return object.color;
                }
                let mut color: Color = (0.0, 0.0, 0.0).into();
                let m = Rotation3::from_axis_angle(
                    &UnitVector3::<f32>::new_normalize(intersection.normal.cross(&Vector::z())),
                    -intersection.normal.angle(&Vector::z()),
                );
                for _ in 0..reflections {
                    let out_going = m * utils::rand_unit_vec();
                    let new_ray = Ray::new(intersection_point, out_going);
                    let factor = object.brdf.apply(ray.dir, out_going, intersection.normal)
                        * (intersection.normal.dot(&out_going));
                    color += object.color.scale(factor)
                        * self.render_pixel(&new_ray, iteration - 1, reflections);
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

pub struct Object {
    pub shape: Shape,
    pub isometry: Isometry<f32>,
    pub brdf: Brdf,
    pub color: Color,
    pub is_light_source: bool,
}

impl Object {
    pub fn get_intersection(&self, ray: &Ray) -> Option<RayIntersection> {
        self.shape.cast_ray_and_get_normal(&self.isometry, ray)
    }
}

pub enum Shape {
    Ball(Ball),
    Cuboid(Cuboid),
}

impl Shape {
    fn cast_ray_and_get_normal(
        &self,
        isometry: &Isometry<f32>,
        ray: &Ray,
    ) -> Option<RayIntersection> {
        match self {
            Shape::Ball(ball) => ball.cast_ray_and_get_normal(isometry, ray, Real::MAX, true),
            Shape::Cuboid(cuboid) => cuboid.cast_ray_and_get_normal(isometry, ray, Real::MAX, true),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Brdf {
    One,
}

impl Brdf {
    pub fn apply(&self, incident: Vector<f32>, out_going: Vector<f32>, normal: Vector<f32>) -> f32 {
        match self {
            Brdf::One => 1.0,
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl Color {
    pub fn scale(&self, factor: f32) -> Self {
        Self {
            r: self.r * factor,
            g: self.g * factor,
            b: self.b * factor,
        }
    }
}

impl From<Color> for Rgb<u8> {
    fn from(value: Color) -> Self {
        Self([
            (value.r * u8::MAX as f32) as u8,
            (value.g * u8::MAX as f32) as u8,
            (value.b * u8::MAX as f32) as u8,
        ])
    }
}

impl From<(f32, f32, f32)> for Color {
    fn from(color: (f32, f32, f32)) -> Self {
        Self {
            r: color.0,
            g: color.1,
            b: color.2,
        }
    }
}
