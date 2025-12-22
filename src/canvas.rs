use image::{Rgb, RgbImage};
use std::io::{self, Write};
use std::path::Path;

use crate::vec3::Color;

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

    #[allow(dead_code)]
    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        let index = (y * self.width + x) as usize;
        if index < self.pixels.len() {
            self.pixels[index] = color;
        }
    }

    pub fn pixels_mut(&mut self) -> &mut [Color] {
        &mut self.pixels
    }

    pub fn save_png<P: AsRef<Path>>(&self, path: P) -> Result<(), image::ImageError> {
        let mut img = RgbImage::new(self.width, self.height);

        // Iterate over the image buffer coordinates
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            // Get linear color from our buffer
            let idx = (y * self.width + x) as usize;
            let linear_color = self.pixels[idx];

            // Apply Gamma Correction (Square root approx for Gamma 2.0)
            let r = linear_color.x.sqrt();
            let g = linear_color.y.sqrt();
            let b = linear_color.z.sqrt();

            // Quantize to u8 [0-255]
            let ir = (255.999 * r.clamp(0.0, 0.999)) as u8;
            let ig = (255.999 * g.clamp(0.0, 0.999)) as u8;
            let ib = (255.999 * b.clamp(0.0, 0.999)) as u8;

            *pixel = Rgb([ir, ig, ib]);
        }

        img.save(path)
    }

    pub fn write_ppm<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writeln!(writer, "P3\n{} {}\n255", self.width, self.height)?;

        for pixel in &self.pixels {
            crate::color::write_color(*pixel, writer);
        }

        Ok(())
    }
}
