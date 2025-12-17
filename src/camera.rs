#![allow(dead_code)]

use std::io::{BufWriter, StdoutLock, Write};

use indicatif::ProgressIterator;

use crate::{
    color::{Color, write_color},
    hittable::Hittable,
    hittable_list::HittableList,
    interval::Interval,
    material::ScatteredRay,
    ray::Ray,
    utils::random_offset_vector,
    vec3::Vec3,
};

const DEFAULT_IMG_WIDTH: u32 = 1280;
const DEFAULT_VIEWPORT_WIDTH: f64 = 3.55;
const DEFAULT_ASPECT_RATIO: f64 = 16.0 / 9.0;
const DEFAULT_FOCAL_LENGTH: f64 = 1.0;
const DEFAULT_SAMPLES_PER_PIXEL: u32 = 100;
const DEFAULT_MAX_DEPTH: u32 = 50;

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
pub struct Camera {
    img_dims: Dimensions<u32>,
    viewport_dims: Dimensions<f64>,
    delta_x: Vec3,
    delta_y: Vec3,
    center: Vec3,
    p00_loc: Vec3,
    samples_per_pixel: u32,
    pixel_samples_scale: f64,
    max_depth: u32,
}

impl Default for Camera {
    fn default() -> Self {
        Camera::new(
            DEFAULT_ASPECT_RATIO,
            DEFAULT_IMG_WIDTH,
            DEFAULT_VIEWPORT_WIDTH,
            Vec3::new(0.0, 0.0, 0.0),
            DEFAULT_FOCAL_LENGTH,
            DEFAULT_SAMPLES_PER_PIXEL,
            DEFAULT_MAX_DEPTH,
        )
    }
}

impl Camera {
    pub fn new(
        target_aspect_ratio: f64,
        img_width: u32,
        viewport_width: f64,
        center: Vec3,
        focal_length: f64,
        samples_per_pixel: u32,
        max_depth: u32,
    ) -> Self {
        let img_height: u32 = if (img_width as f64 / target_aspect_ratio) < 1.0 {
            1
        } else {
            (img_width as f64 / target_aspect_ratio) as u32
        };
        let actual_aspect_ratio: f64 = img_width as f64 / img_height as f64;
        let viewport_height: f64 = viewport_width / actual_aspect_ratio;

        let viewport_x: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_y: Vec3 = Vec3::new(0.0, -viewport_height, 0.0);
        let delta_x: Vec3 = viewport_x / img_width as f64;
        let delta_y: Vec3 = viewport_y / img_height as f64;
        let viewport_upper_left: Vec3 =
            center - Vec3::new(0.0, 0.0, focal_length) - viewport_x / 2.0 - viewport_y / 2.0;
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
            center,
            p00_loc,
            samples_per_pixel,
            pixel_samples_scale: 1.0 / samples_per_pixel as f64,
            max_depth,
        }
    }

    fn ray_color(&self, ray: &Ray, depth: u32, world: &HittableList) -> Vec3 {
        if depth <= 0 {
            return Vec3::default();
        }

        // Sphere intersection
        if let Some(rec) = world.hit(ray, Interval::new(0.001, f64::INFINITY)) {
            if let Some(ScatteredRay {
                attenuation,
                scattered,
            }) = rec.material.scatter(ray, &rec)
            {
                return self.ray_color(&scattered, depth - 1, world) * attenuation;
            } else {
                return Vec3::default();
            }
        }

        // Background
        let normalized_direction: Vec3 = ray.direction() / ray.direction().length();
        let white: Vec3 = Vec3::new(1.0, 1.0, 1.0);
        let blue: Vec3 = Vec3::new(0.5, 0.7, 1.0);
        let a: f64 = 0.5 * (normalized_direction.y() + 1.0);
        blue * a + white * (1.0 - a)
    }

    pub fn render(&self, handle: &mut BufWriter<StdoutLock<'_>>, world: &HittableList) {
        writeln!(
            handle,
            "P3\n{} {}\n255\n",
            self.img_dims.width(),
            self.img_dims.height()
        )
        .unwrap();
        for j in (0..self.img_dims.height()).progress() {
            for i in 0..self.img_dims.width() {
                // (row=j, col=i) represents a single pixel on the screen
                let mut color: Color = Vec3::default();
                // Anti-aliasing
                for _ in 0..self.samples_per_pixel {
                    let offset = random_offset_vector();
                    let pji = self.p00_loc
                        + (self.delta_x * (i as f64 + offset.x()))
                        + (self.delta_y * (j as f64 + offset.y()));
                    let r: Ray = Ray::new(self.center, pji - self.center);
                    color = color + self.ray_color(&r, self.max_depth, &world);
                }
                color = color * self.pixel_samples_scale;
                write_color(color, handle);
            }
        }
    }
}
