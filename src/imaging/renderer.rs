use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rayon::prelude::*;

use crate::{
    common,
    geometry::{hittable::Hittable, hittable_list::HittableList},
    imaging::{camera::Camera, canvas::Canvas},
    math::{interval::Interval, ray::Ray, vec3::Color},
};

/// Handles the rendering process.
pub struct Renderer {
    pub samples_per_pixel: u32,
    pub max_depth: u32,
}

impl Renderer {
    /// Creates a new Renderer with specified settings.
    pub fn new(samples_per_pixel: u32, max_depth: u32) -> Self {
        Self {
            samples_per_pixel,
            max_depth,
        }
    }

    /// Renders the scene to the canvas.
    pub fn render(&self, world: &HittableList, camera: &Camera, canvas: &mut Canvas) {
        let width = canvas.width as usize;
        let height = canvas.height as usize;

        let pb = ProgressBar::new(height as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
                .unwrap()
                .progress_chars("##-"),
        );

        canvas
            .pixels_mut()
            .par_chunks_mut(width)
            .progress_with(pb)
            .enumerate()
            .for_each(|(j, row)| {
                for (i, pixel) in row.iter_mut().enumerate() {
                    let mut pixel_color = Color::default();

                    for _ in 0..self.samples_per_pixel {
                        let r_i = i as f64 + common::random_range(-0.5, 0.5);
                        let r_j = j as f64 + common::random_range(-0.5, 0.5);
                        let u = r_i / (width as f64);
                        let v = r_j / (height as f64);

                        let ray = camera.get_ray(u, v);
                        pixel_color = pixel_color + self.ray_color(&ray, self.max_depth, world);
                    }

                    let scale = 1.0 / self.samples_per_pixel as f64;
                    *pixel = pixel_color * scale;
                }
            });
    }

    fn ray_color(&self, ray: &Ray, depth: u32, world: &HittableList) -> Color {
        if depth == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        if let Some(rec) = world.hit(ray, Interval::new(0.001, f64::INFINITY)) {
            if let Some(scattered) = rec.material.scatter(ray, &rec) {
                return self.ray_color(&scattered.scattered, depth - 1, world)
                    * scattered.attenuation;
            }
            return Color::new(0.0, 0.0, 0.0);
        }

        let unit_direction = ray.direction.unit_vector();
        let a = 0.5 * (unit_direction.y + 1.0);

        Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
    }
}
