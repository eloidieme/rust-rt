#![allow(dead_code)]

use crate::vec3::Vec3;

const DEFAULT_IMG_WIDTH: u32 = 1280;
const DEFAULT_VIEWPORT_WIDTH: f64 = 3.5;
const DEFAULT_ASPECT_RATIO: f64 = 16.0 / 9.0;
const DEFAULT_FOCAL_LENGTH: f64 = 1.0;
const DEFAULT_SAMPLES_PER_PIXEL: u32 = 100;

#[derive(Debug, Clone, Copy)]
pub struct Dimensions<T: Copy + Clone + PartialEq> {
    width: T,
    height: T,
}

impl<T: Copy + Clone + PartialEq> Dimensions<T> {
    pub fn width(&self) -> T {
        self.width
    }

    pub fn height(&self) -> T {
        self.height
    }
}

#[derive(Debug)]
pub struct Viewport {
    img_dims: Dimensions<u32>,
    viewport_dims: Dimensions<f64>,
    delta_x: Vec3<f64>,
    delta_y: Vec3<f64>,
    camera: Vec3<f64>,
    p00_loc: Vec3<f64>,
    samples_per_pixel: u32,
    pixel_samples_scale: f64,
}

impl Default for Viewport {
    fn default() -> Self {
        Viewport::new(
            DEFAULT_ASPECT_RATIO,
            DEFAULT_IMG_WIDTH,
            DEFAULT_VIEWPORT_WIDTH,
            Vec3::new(0.0, 0.0, 0.0),
            DEFAULT_FOCAL_LENGTH,
            DEFAULT_SAMPLES_PER_PIXEL,
        )
    }
}

impl Viewport {
    pub fn new(
        target_aspect_ratio: f64,
        img_width: u32,
        viewport_width: f64,
        camera: Vec3<f64>,
        focal_length: f64,
        samples_per_pixel: u32,
    ) -> Self {
        let img_height: u32 = if (img_width as f64 / target_aspect_ratio) < 1.0 {
            1
        } else {
            (img_width as f64 / target_aspect_ratio) as u32
        };
        let actual_aspect_ratio: f64 = img_width as f64 / img_height as f64;
        let viewport_height: f64 = viewport_width / actual_aspect_ratio;

        let viewport_x: Vec3<f64> = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_y: Vec3<f64> = Vec3::new(0.0, -viewport_height, 0.0);
        let delta_x: Vec3<f64> = viewport_x / img_width as f64;
        let delta_y: Vec3<f64> = viewport_y / img_height as f64;
        let viewport_upper_left: Vec3<f64> =
            camera - Vec3::new(0.0, 0.0, focal_length) - viewport_x / 2.0 - viewport_y / 2.0;
        let p00_loc = viewport_upper_left + (delta_x + delta_y) * 0.5;

        Self {
            img_dims: Dimensions {
                width: img_width,
                height: img_height,
            },
            viewport_dims: Dimensions {
                width: viewport_width,
                height: viewport_height,
            },
            delta_x,
            delta_y,
            camera,
            p00_loc,
            samples_per_pixel,
            pixel_samples_scale: 1.0 / samples_per_pixel as f64,
        }
    }

    pub fn camera(&self) -> Vec3<f64> {
        self.camera
    }

    pub fn delta_x(&self) -> Vec3<f64> {
        self.delta_x
    }

    pub fn delta_y(&self) -> Vec3<f64> {
        self.delta_y
    }

    pub fn p00_loc(&self) -> Vec3<f64> {
        self.p00_loc
    }

    pub fn img_dims(&self) -> Dimensions<u32> {
        self.img_dims
    }

    pub fn viewport_dims(&self) -> Dimensions<f64> {
        self.viewport_dims
    }

    pub fn samples_per_pixel(&self) -> u32 {
        self.samples_per_pixel
    }

    pub fn pixel_samples_scale(&self) -> f64 {
        self.pixel_samples_scale
    }
}
