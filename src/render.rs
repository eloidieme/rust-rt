use std::io::{BufWriter, StdoutLock, Write};

use indicatif::ProgressIterator;

use crate::{
    hittable::Hittable, hittable_list::HittableList, interval::Interval, ray::Ray,
    utils::random_offset_vector, vec3::Vec3, viewport::Viewport,
};

fn write_color(color: Vec3<f64>, handle: &mut BufWriter<StdoutLock<'_>>) {
    let ir: u8 = (255.999 * color.x().clamp(0.0, 0.999)) as u8;
    let ig: u8 = (255.999 * color.y().clamp(0.0, 0.999)) as u8;
    let ib: u8 = (255.999 * color.z().clamp(0.0, 0.999)) as u8;
    writeln!(handle, "{ir} {ig} {ib}").unwrap();
}

fn ray_color(ray: &Ray, world: &HittableList) -> Vec3<f64> {
    // Sphere intersection
    if let Some(rec) = world.hit(ray, Interval::new(0.0, f64::INFINITY)) {
        let normal: Vec3<f64> = rec.normal;
        return Vec3::new(
            0.5 * (normal.x() + 1.0),
            0.5 * (normal.y() + 1.0),
            0.5 * (normal.z() + 1.0),
        );
    }

    // Background
    let normalized_direction: Vec3<f64> = ray.direction() / ray.direction().length();
    let white: Vec3<f64> = Vec3::new(1.0, 1.0, 1.0);
    let blue: Vec3<f64> = Vec3::new(0.5, 0.7, 1.0);
    let a: f64 = 0.5 * (normalized_direction.y() + 1.0);
    blue * a + white * (1.0 - a)
}

pub fn render(handle: &mut BufWriter<StdoutLock<'_>>, viewport: &Viewport, world: &HittableList) {
    writeln!(
        handle,
        "P3\n{} {}\n255\n",
        viewport.img_dims().width(),
        viewport.img_dims().height()
    )
    .unwrap();
    for j in (0..viewport.img_dims().height()).progress() {
        for i in 0..viewport.img_dims().width() {
            // (row=j, col=i) represents a single pixel on the screen
            let mut color: Vec3<f64> = Vec3::default();
            for _ in 0..viewport.samples_per_pixel() {
                let offset = random_offset_vector();
                let pji = viewport.p00_loc()
                    + (viewport.delta_x() * (i as f64 + offset.x()))
                    + (viewport.delta_y() * (j as f64 + offset.y()));
                let r: Ray = Ray::new(viewport.camera(), pji - viewport.camera());
                color = color + ray_color(&r, &world);
            }
            color = color * viewport.pixel_samples_scale();
            write_color(color, handle);
        }
    }
}
