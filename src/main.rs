mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod render;
mod sphere;
mod utils;
mod vec3;
mod viewport;

use std::io::{self, BufWriter};

use crate::{
    hittable_list::HittableList, render::render, sphere::Sphere, vec3::Vec3, viewport::Viewport,
};

fn main() {
    let stdout = io::stdout();
    let mut handle = BufWriter::new(stdout.lock());

    let viewport: Viewport = Viewport::default();

    let mut world = HittableList::new();
    let sphere_1 = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
    let sphere_2 = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0);
    world.add(sphere_1);
    world.add(sphere_2);

    render(&mut handle, &viewport, &world);
}
