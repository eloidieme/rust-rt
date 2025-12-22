use std::io::Write;

use crate::{
    common::random_range,
    vec3::{Color, Vec3},
};

pub fn write_color<T: Write>(color: Color, handle: &mut T) {
    let r = linear_to_gamma(color.x).clamp(0.0, 0.999);
    let g = linear_to_gamma(color.y).clamp(0.0, 0.999);
    let b = linear_to_gamma(color.z).clamp(0.0, 0.999);

    let ir: u8 = (255.999 * r) as u8;
    let ig: u8 = (255.999 * g) as u8;
    let ib: u8 = (255.999 * b) as u8;
    writeln!(handle, "{ir} {ig} {ib}").unwrap();
}

pub fn random_color(min: f64, max: f64) -> Color {
    Vec3::new(
        random_range(min, max),
        random_range(min, max),
        random_range(min, max),
    )
}

pub fn linear_to_gamma(linear: f64) -> f64 {
    if linear > 0.0 {
        return linear.sqrt();
    }

    return 0.0;
}
