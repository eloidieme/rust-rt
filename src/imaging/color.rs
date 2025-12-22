use crate::math::vec3::Color;
use std::io::Write;

/// Converts a linear color to gamma-corrected RGB bytes.
pub fn to_rgb_bytes(color: Color) -> [u8; 3] {
    let r = linear_to_gamma(color.x);
    let g = linear_to_gamma(color.y);
    let b = linear_to_gamma(color.z);

    let ir = (255.999 * r.clamp(0.0, 0.999)) as u8;
    let ig = (255.999 * g.clamp(0.0, 0.999)) as u8;
    let ib = (255.999 * b.clamp(0.0, 0.999)) as u8;

    [ir, ig, ib]
}

/// Writes a color to a writer in PPM format (space-separated RGB values).
pub fn write_color<T: Write>(color: Color, handle: &mut T) {
    let [ir, ig, ib] = to_rgb_bytes(color);
    writeln!(handle, "{ir} {ig} {ib}").unwrap();
}

pub fn linear_to_gamma(linear: f64) -> f64 {
    if linear > 0.0 {
        return linear.sqrt();
    }
    return 0.0;
}
