use image::{Rgb, RgbImage};
use std::io::{self, Write};
use std::path::Path;

use crate::{imaging::color::to_rgb_bytes, math::vec3::Color};

/// Represents a canvas of pixels.
pub struct Canvas {
    pub width: u32,
    pub height: u32,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: u32, aspect_ratio: f64) -> Self {
        let mut height = (width as f64 / aspect_ratio) as u32;
        if height < 1 {
            height = 1;
        }

        Self {
            width,
            height,
            pixels: vec![Color::default(); (width * height) as usize],
        }
    }

    pub fn pixels_mut(&mut self) -> &mut [Color] {
        &mut self.pixels
    }

    pub fn save_png<P: AsRef<Path>>(&self, path: P) -> Result<(), image::ImageError> {
        let mut img = RgbImage::new(self.width, self.height);

        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let idx = (y * self.width + x) as usize;
            let linear_color = self.pixels[idx];

            let rgb = to_rgb_bytes(linear_color);

            *pixel = Rgb(rgb);
        }

        img.save(path)
    }

    pub fn write_ppm<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writeln!(writer, "P3\n{} {}\n255", self.width, self.height)?;

        for pixel in &self.pixels {
            crate::imaging::color::write_color(*pixel, writer);
        }

        Ok(())
    }
}
