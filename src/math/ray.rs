use crate::math::vec3::{Point3, Vec3};

#[derive(Debug, Clone, Copy)]
/// Represents a ray with an origin and a direction.
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    /// Creates a new Ray.
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    /// Returns the point at distance t along the ray.
    #[inline]
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }
}
