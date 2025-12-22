use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;

use clap::Parser;
use rust_rt::{engine::Engine, scene};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Width of the output image (ignored if scene file is used)
    #[arg(short = 'w', long, default_value_t = 1200)]
    width: u32,

    /// Number of random samples per pixel
    #[arg(short = 's', long, default_value_t = 100)]
    samples: u32,

    /// Maximum number of ray bounces
    #[arg(short = 'd', long, default_value_t = 50)]
    depth: u32,

    /// Path to a scene YAML file. If omitted, generates a random scene.
    #[arg(long)]
    scene: Option<String>,

    /// Output filename
    #[arg(short = 'o', long, default_value = "renders/image.png")]
    output: String,
}

fn main() {
    let args = Args::parse();
    let engine = Engine::new(args.samples, args.depth);

    let (world, camera, width, height, background) = if let Some(ref path) = args.scene {
        println!("Loading scene from {}...", path);
        engine.load_scene_from_file(path)
    } else {
        println!("No scene file provided. Generating random book scene...");
        let aspect_ratio = 16.0 / 9.0;
        let (world, camera) = scene::random_book_scene(aspect_ratio);
        let height = (args.width as f64 / aspect_ratio) as u32;
        let default_bg = rust_rt::scene::Background::VerticalGradient {
            top: rust_rt::math::vec3::Color::new(0.5, 0.7, 1.0),
            bottom: rust_rt::math::vec3::Color::new(1.0, 1.0, 1.0),
        };
        (world, camera, args.width, height, default_bg)
    };

    let canvas = engine.render(&world, &camera, width, height, &background);

    let mut path = PathBuf::from(&args.output);
    if path.extension().is_none() {
        path.set_extension("ppm");
    }

    println!("Saving to {}...", path.display());

    let extension = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("png")
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
