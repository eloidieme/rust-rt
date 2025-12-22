mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod sphere;
mod utils;
mod vec3;

use std::io::{self, BufWriter};

use crate::{
    camera::Camera,
    color::{Color, random_color},
    hittable_list::HittableList,
    material::{Dielectric, Lambertian, Metal},
    sphere::Sphere,
    utils::random_float,
    vec3::Vec3,
};

fn main() {
    let stdout = io::stdout();
    let mut handle = BufWriter::new(stdout.lock());

    let mut world = HittableList::new();

    let material_ground = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    world.add(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_float(0.0, 1.0);
            let center = Vec3::new(
                a as f64 + 0.9 * random_float(0.0, 1.0),
                0.2,
                b as f64 + 0.9 * random_float(0.0, 1.0),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = random_color(0.0, 1.0) * random_color(0.0, 1.0);
                    world.add(Sphere::new(center, 0.2, Lambertian::new(albedo)));
                } else if choose_mat < 0.95 {
                    let albedo = random_color(0.5, 1.0);
                    let fuzz = random_float(0.0, 0.5);
                    world.add(Sphere::new(center, 0.2, Metal::new(albedo, fuzz)));
                } else {
                    world.add(Sphere::new(center, 0.2, Dielectric::new(1.5)));
                }
            }
        }
    }

    let material_1 = Dielectric::new(1.50);
    world.add(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material_1));

    let material_2 = Lambertian::new(Vec3::new(0.4, 0.2, 0.1));
    world.add(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material_2));

    let material_3 = Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0);
    world.add(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material_3));

    let camera: Camera = Camera::default();
    camera.render(&mut handle, &world);
}
