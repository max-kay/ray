use std::{f32::consts::PI, path::Path};

use image::{ImageResult, Rgb, RgbImage};
use itertools::Itertools;
use parry3d::{
    math::{Isometry, Point, Vector},
    query::Ray,
};

pub struct Camera {
    fov: f32,
    isometry: Isometry<f32>,
    image: RgbImage,
}

impl Camera {
    pub fn face_towards(
        eye: Point<f32>,
        target: Point<f32>,
        fov: f32,
        width: u32,
        height: u32,
    ) -> Self {
        let isometry = Isometry::face_towards(&eye, &target, &Vector::z());
        Self {
            fov,
            isometry,
            image: RgbImage::new(width, height),
        }
    }

    pub fn get_rays(&mut self) -> Vec<(Ray, &mut Rgb<u8>)> {
        let width = self.image.width();
        let height = self.image.height();

        let eye = self.isometry * Point::new(0.0, 0.0, 0.0);
        let direction = self.isometry * Vector::new(0.0, 0.0, 1.0);

        let pixel_length = 2.0 / width as f32 * (self.fov / 2.0).tan();
        let img_x = self.isometry * Vector::new(-pixel_length, 0.0, 0.0);
        let img_y = self.isometry * Vector::new(0.0, pixel_length, 0.0);

        let xs = (0..width).map(|val| val as f32 - width as f32 / 2.0);
        let ys = (0..height).map(|val| height as f32 / 2.0 - val as f32);
        ys.cartesian_product(xs)
            .map(|(y, x)| Ray::new(eye, (direction + img_x * x + img_y * y).normalize()))
            .zip(self.image.pixels_mut())
            .collect()
    }
}

impl Camera {
    pub fn save_img(&self, path: impl AsRef<Path>) -> ImageResult<()> {
        self.image.save(path)
    }

    pub fn width(&self) -> u32 {
        self.image.width()
    }

    pub fn height(&self) -> u32 {
        self.image.height()
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::face_towards(
            Point::new(10.0, 5.0, 8.0),
            Point::new(0.0, 0.0, 0.0),
            PI / 3.0,
            100,
            100,
        )
    }
}
