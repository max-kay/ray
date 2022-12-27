use std::ops::{Add, AddAssign, Mul};

use image::Rgb;
use parry3d::math::{Point, Vector};

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian(Lambertian),
    Light(Light),
}

impl Material {
    pub fn new_lambertian(color: Color) -> Self {
        Self::Lambertian(Lambertian { color })
    }

    pub fn new_light(color: Color) -> Self {
        Self::Light(Light { color })
    }
}

impl Material {
    pub fn brdf(&self, incident: Vector<f32>, out_going: Vector<f32>, normal: Vector<f32>) -> f32 {
        match self {
            Material::Lambertian(mat) => mat.brdf(incident, out_going, normal),
            Material::Light(mat) => mat.brdf(incident, out_going, normal),
        }
    }

    pub fn get_color(&self, _x: Point<f32>) -> Color {
        match self {
            Material::Lambertian(mat) => mat.get_color(),
            Material::Light(mat) => mat.get_color(),
        }
    }

    pub fn is_light_source(&self) -> bool {
        match self {
            Material::Light(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Lambertian {
    color: Color,
}

impl Lambertian {
    fn brdf(&self, incident: Vector<f32>, out_going: Vector<f32>, normal: Vector<f32>) -> f32 {
        1.0 //TODO
    }

    fn get_color(&self) -> Color {
        self.color
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Light {
    color: Color,
}

impl Light {
    fn brdf(&self, incident: Vector<f32>, out_going: Vector<f32>, normal: Vector<f32>) -> f32 {
        1.0 //TODO
    }

    fn get_color(&self) -> Color {
        self.color
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
            (value.r * 10.0 * u8::MAX as f32) as u8,
            (value.g * 10.0 * u8::MAX as f32) as u8,
            (value.b * 10.0 * u8::MAX as f32) as u8,
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
