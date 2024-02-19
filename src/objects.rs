use crate::{camera::Camera, utils};
use image::Rgb;
use indicatif::{ParallelProgressIterator, ProgressIterator};
use nalgebra::{Rotation3, UnitVector3};
use parry3d::{
    math::{Isometry, Real, Vector},
    query::{Ray, RayCast, RayIntersection},
    shape::{Ball, Capsule, Cuboid, HalfSpace, RoundCuboid},
};
use rayon::prelude::*;
use std::ops::{Add, AddAssign, Mul};

#[derive(Default)]
pub struct Scene {
    objects: Vec<Object>,
    background: Color,
}

impl Scene {
    pub fn new(background: Color) -> Self {
        Self {
            background,
            objects: Vec::new(),
        }
    }

    pub fn add_objects(mut self, mut objects: Vec<Object>) -> Self {
        self.objects.append(&mut objects);
        self
    }
    pub fn add_object(mut self, objects: Object) -> Self {
        self.objects.push(objects);
        self
    }

    pub fn render_par_with_progress(
        &self,
        camera: &mut Camera,
        rays: usize,
        max_reflections: usize,
    ) {
        camera
            .get_rays()
            .into_par_iter()
            .progress()
            .for_each(|(ray, color)| *color = self.render_ray(&ray, rays, max_reflections).into());
    }

    pub fn render_with_progress(&self, camera: &mut Camera, rays: usize, max_reflections: usize) {
        camera
            .get_rays()
            .into_iter()
            .progress()
            .for_each(|(ray, color)| *color = self.render_ray(&ray, rays, max_reflections).into());
    }

    pub fn render_par(&self, camera: &mut Camera, rays: usize, max_reflections: usize) {
        camera
            .get_rays()
            .into_par_iter()
            .for_each(|(ray, color)| *color = self.render_ray(&ray, rays, max_reflections).into());
    }

    pub fn render(&self, camera: &mut Camera, rays: usize, max_reflections: usize) {
        camera
            .get_rays()
            .into_iter()
            .for_each(|(ray, color)| *color = self.render_ray(&ray, rays, max_reflections).into());
    }

    fn render_ray(&self, ray: &Ray, rays: usize, max_reflections: usize) -> Color {
        if max_reflections == 0 {
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
                let mut color = Color(0.0, 0.0, 0.0);
                let direction_transform = Rotation3::from_axis_angle(
                    &UnitVector3::<f32>::new_normalize(intersection.normal.cross(&Vector::z())),
                    -intersection.normal.angle(&Vector::z()),
                );
                for _ in 0..rays {
                    let out_going = direction_transform * utils::rand_unit_vec_pos_z();
                    let new_ray = Ray::new(intersection_point, out_going);
                    let factor = object.brdf(ray.dir, out_going, intersection.normal)
                        * (intersection.normal.dot(&out_going));
                    color += object.color.scale(factor)
                        * self.render_ray(&new_ray, 1, max_reflections - 1);
                }
                color.scale(1.0 / rays as f32)
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

    fn brdf(&self, in_coming: Vector<f32>, out_going: Vector<f32>, normal: Vector<f32>) -> f32 {
        self.brdf.apply(in_coming, out_going)
    }
}

pub enum Shape {
    Ball(Ball),
    Cuboid(Cuboid),
    Capsule(Capsule),
    RoundCuboid(RoundCuboid),
    HalfSpace(HalfSpace),
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
            Shape::Capsule(capsule) => {
                capsule.cast_ray_and_get_normal(isometry, ray, Real::MAX, true)
            }
            Shape::RoundCuboid(cuboid) => {
                cuboid.cast_ray_and_get_normal(isometry, ray, Real::MAX, true)
            }
            Shape::HalfSpace(half_space) => {
                half_space.cast_ray_and_get_normal(isometry, ray, Real::MAX, true)
            }
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub enum Brdf {
    Diffuse,
    Mirror,
    Glossy,
    #[default]
    One,
}

impl Brdf {
    /// computes the given brdf
    /// fullfills po
    fn apply(&self, in_comming: Vector<f32>, out_going: Vector<f32>) -> f32 {
        match self {
            Brdf::Diffuse => todo!(),
            Brdf::Mirror => todo!(),
            Brdf::Glossy => todo!(),
            Brdf::One => 1.0,
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Color(pub f32, pub f32, pub f32);

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl Color {
    pub fn scale(&self, factor: f32) -> Self {
        Self(self.0 * factor, self.1 * factor, self.2 * factor)
    }
}

impl From<Color> for Rgb<u8> {
    fn from(value: Color) -> Self {
        Self([
            (value.0 * u8::MAX as f32) as u8,
            (value.1 * u8::MAX as f32) as u8,
            (value.2 * u8::MAX as f32) as u8,
        ])
    }
}
