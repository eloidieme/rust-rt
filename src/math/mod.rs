pub mod interval;
pub mod ray;
pub mod vec3;

// Re-export common types so you can use `crate::math::Vec3` directly
pub use interval::Interval;
pub use ray::Ray;
pub use vec3::{Color, Point3, Vec3};
