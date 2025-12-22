mod camera;
mod canvas;
mod color;
mod common;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod renderer;
mod scene;
mod sphere;
mod vec3;

use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;

use clap::Parser;

use crate::{canvas::Canvas, renderer::Renderer};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Width of the output image in pixels
    #[arg(short = 'w', long, default_value_t = 1200)]
    width: u32,

    /// Number of random samples per pixel (higher = less noise, slower)
    #[arg(short = 's', long, default_value_t = 100)]
    samples: u32,

    /// Maximum number of ray bounces (recursion depth)
    #[arg(short = 'd', long, default_value_t = 50)]
    depth: u32,

    /// Output filename (e.g., render.ppm)
    #[arg(short = 'o', long, default_value = "image.ppm")]
    output: String,
}

fn main() {
    let args = Args::parse();

    let aspect_ratio = 16.0 / 9.0;
    let (world, camera) = scene::random_book_scene(aspect_ratio);

    let mut canvas = Canvas::new(args.width, aspect_ratio);
    let renderer = Renderer::new(args.samples, args.depth);

    println!(
        "Rendering {}x{} image with {} samples...",
        canvas.width, canvas.height, args.samples
    );

    renderer.render(&world, &camera, &mut canvas);
    let mut path = PathBuf::from(&args.output);

    if path.extension().is_none() {
        path.set_extension("ppm");
    }
    println!("Saving to {}...", path.display());

    let extension = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("ppm")
        .to_lowercase();

    match extension.as_str() {
        "png" => {
            canvas.save_png(&path).expect("Failed to save PNG");
        }
        "ppm" => {
            let file = File::create(&path).expect("Failed to create file");
            let mut writer = BufWriter::new(file);
            canvas.write_ppm(&mut writer).expect("Failed to save PPM");
        }
        _ => {
            eprintln!("Unknown format '{}'. Defaulting to PNG.", extension);
            canvas.save_png(&path).expect("Failed to save PNG");
        }
    }

    println!("Done!");
}
