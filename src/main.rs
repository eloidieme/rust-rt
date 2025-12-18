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
    color::Color,
    hittable_list::HittableList,
    material::{Dielectric, Lambertian, Metal},
    sphere::Sphere,
    vec3::Vec3,
};

fn main() {
    let stdout = io::stdout();
    let mut handle = BufWriter::new(stdout.lock());

    let mut world = HittableList::new();

    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(1.0 / 1.33);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0);

    let sphere_center = Sphere::new(Vec3::new(0.0, 0.0, -1.2), 0.5, material_center);
    let sphere_left = Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left);
    let sphere_right = Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right);
    let ground = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, material_ground);
    world.add(sphere_center);
    world.add(sphere_left);
    world.add(sphere_right);
    world.add(ground);

    let camera: Camera = Camera::default();
    camera.render(&mut handle, &world);
}
