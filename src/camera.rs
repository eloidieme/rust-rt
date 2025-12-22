#![allow(dead_code)]

use std::io::{BufWriter, StdoutLock, Write};

use indicatif::ProgressIterator;

use crate::{
    color::write_color,
    hittable::Hittable,
    hittable_list::HittableList,
    interval::Interval,
    material::ScatteredRay,
    ray::Ray,
    vec3::{Color, Vec3},
};

const DEFAULT_IMG_WIDTH: u32 = 400;
const DEFAULT_ASPECT_RATIO: f64 = 16.0 / 9.0;
const DEFAULT_FOCAL_LENGTH: f64 = 1.0;
const DEFAULT_SAMPLES_PER_PIXEL: u32 = 200;
const DEFAULT_MAX_DEPTH: u32 = 50;
const DEFAULT_VERTICAL_FOV: f64 = 20.0;
const DEFAULT_DEFOCUS_ANGLE: f64 = 0.6;
const DEFAULT_FOCUS_DIST: f64 = 10.0;

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
    lookat: Vec3,
    lookfrom: Vec3,
    p00_loc: Vec3,
    samples_per_pixel: u32,
    pixel_samples_scale: f64,
    max_depth: u32,
    vertical_fov: f64,
    defocus_angle: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        Camera::new(
            DEFAULT_ASPECT_RATIO,
            DEFAULT_IMG_WIDTH,
            Vec3::new(13.0, 2.0, 3.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            DEFAULT_SAMPLES_PER_PIXEL,
            DEFAULT_MAX_DEPTH,
            DEFAULT_VERTICAL_FOV,
            DEFAULT_DEFOCUS_ANGLE,
            DEFAULT_FOCUS_DIST,
        )
    }
}

impl Camera {
    pub fn new(
        target_aspect_ratio: f64,
        img_width: u32,
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        samples_per_pixel: u32,
        max_depth: u32,
        vertical_fov: f64,
        defocus_angle: f64,
        focus_dist: f64,
    ) -> Self {
        let img_height: u32 = if (img_width as f64 / target_aspect_ratio) < 1.0 {
            1
        } else {
            (img_width as f64 / target_aspect_ratio) as u32
        };

        let center: Vec3 = lookfrom;
        let actual_aspect_ratio: f64 = img_width as f64 / img_height as f64;
        let theta = vertical_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height: f64 = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * actual_aspect_ratio;

        let w: Vec3 = (lookfrom - lookat).unit_vector();
        let u: Vec3 = vup.cross(w).unit_vector();
        let v: Vec3 = w.cross(u);

        let viewport_x: Vec3 = u * viewport_width;
        let viewport_y: Vec3 = -v * viewport_height;

        let delta_x: Vec3 = viewport_x / img_width as f64;
        let delta_y: Vec3 = viewport_y / img_height as f64;
        let viewport_upper_left: Vec3 =
            center - w * focus_dist - viewport_x / 2.0 - viewport_y / 2.0;
        let p00_loc = viewport_upper_left + (delta_x + delta_y) * 0.5;

        let defocus_radius = (defocus_angle / 2.0).to_radians().tan() * focus_dist;
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Self {
            img_dims: Dimensions {
                width: img_width,
                height: img_height,
            },
            viewport_dims: Dimensions {
                width: viewport_width,
                height: viewport_height,
            },
            lookat,
            lookfrom,
            delta_x,
            delta_y,
            center,
            p00_loc,
            samples_per_pixel,
            pixel_samples_scale: 1.0 / samples_per_pixel as f64,
            max_depth,
            vertical_fov,
            defocus_disk_u,
            defocus_disk_v,
            defocus_angle,
        }
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        let p: Vec3 = Vec3::random_in_unit_disk();
        self.center + (self.defocus_disk_u * p.x) + (self.defocus_disk_v * p.y)
    }

    fn ray_color(&self, ray: &Ray, depth: u32, world: &HittableList) -> Vec3 {
        if depth <= 0 {
            return Vec3::default();
        }

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
        let normalized_direction: Vec3 = ray.direction.unit_vector();
        let white: Vec3 = Vec3::new(1.0, 1.0, 1.0);
        let blue: Vec3 = Vec3::new(0.5, 0.7, 1.0);
        let a: f64 = 0.5 * (normalized_direction.y + 1.0);
        blue * a + white * (1.0 - a)
    }

    pub fn render(&self, handle: &mut BufWriter<StdoutLock<'_>>, world: &HittableList) {
        // TODO: return a Vec<u8> here instead, to decouple computation with I/O
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
                    let offset = Vec3::random_offset_vector();
                    let pji = self.p00_loc
                        + (self.delta_x * (i as f64 + offset.x))
                        + (self.delta_y * (j as f64 + offset.y));
                    let ray_origin = if self.defocus_angle <= 0.0 {
                        self.center
                    } else {
                        self.defocus_disk_sample()
                    };
                    let r: Ray = Ray::new(ray_origin, pji - ray_origin);
                    color = color + self.ray_color(&r, self.max_depth, &world);
                }
                color = color * self.pixel_samples_scale;
                write_color(color, handle);
            }
        }
    }
}
