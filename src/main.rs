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
mod sphere;
mod vec3;

use std::io::BufWriter;
use std::sync::Arc;
use std::{fs::File, path::Path};

use clap::Parser;

use crate::{
    camera::Camera,
    canvas::Canvas,
    hittable_list::HittableList,
    material::{Dielectric, Lambertian, Material, Metal},
    renderer::Renderer,
    sphere::Sphere,
    vec3::{Color, Vec3},
};

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

    let mut world = HittableList::default();

    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = common::random();
            let center = Vec3::new(
                a as f64 + 0.9 * common::random(),
                0.2,
                b as f64 + 0.9 * common::random(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn Material + Send + Sync>;

                if choose_mat < 0.8 {
                    // Diffuse
                    let albedo = Color::random() * Color::random();
                    sphere_material = Arc::new(Lambertian::new(albedo));
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = common::random_range(0.0, 0.5);
                    sphere_material = Arc::new(Metal::new(albedo, fuzz));
                } else {
                    // Glass
                    sphere_material = Arc::new(Dielectric::new(1.5));
                }

                world.add(Sphere::new(center, 0.2, sphere_material));
            }
        }
    }

    let material_1 = Arc::new(Dielectric::new(1.5));
    world.add(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material_1));

    let material_2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material_2));

    let material_3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material_3));

    let camera = Camera::builder()
        .aspect_ratio(aspect_ratio)
        .look_from(Vec3::new(13.0, 2.0, 3.0))
        .look_at(Vec3::new(0.0, 0.0, 0.0))
        .vup(Vec3::new(0.0, 1.0, 0.0))
        .fov(20.0)
        .defocus_angle(0.6)
        .focus_dist(10.0)
        .build();

    let mut canvas = Canvas::new(args.width, aspect_ratio);

    let renderer = Renderer::new(args.samples, args.depth);

    println!(
        "Rendering {}x{} image with {} samples...",
        canvas.width, canvas.height, args.samples
    );

    renderer.render(&world, &camera, &mut canvas);

    println!("Saving to {}...", args.output);

    let path = Path::new(&args.output);
    let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("ppm");

    match extension.to_lowercase().as_str() {
        "png" => {
            canvas.save_png(path).expect("Failed to save PNG");
        }
        "ppm" => {
            let file = File::create(path).expect("Failed to create file");
            let mut writer = BufWriter::new(file);
            canvas.write_ppm(&mut writer).expect("Failed to save PPM");
        }
        _ => {
            eprintln!("Unknown format '{}'. Defaulting to PNG.", extension);
            canvas.save_png(path).expect("Failed to save PNG");
        }
    }

    println!("Done!");
}
