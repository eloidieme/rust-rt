use std::{fmt, ops};

use crate::common;

pub type Point3 = Vec3;
pub type Color = Vec3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3::new(0.0, 0.0, 0.0)
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    #[inline]
    pub fn dot(&self, rhs: Vec3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: Vec3) -> Vec3 {
        Vec3::new(
            self.y * rhs.z - rhs.y * self.z,
            rhs.x * self.z - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    pub fn reflect(&self, normal: Vec3) -> Vec3 {
        *self - 2.0 * self.dot(normal) * normal
    }

    pub fn refract(&self, normal: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = f64::min(-self.dot(normal), 1.0);
        let r_out_perp = etai_over_etat * (*self + cos_theta * normal);
        let r_out_parallel = -f64::sqrt(f64::abs(1.0 - r_out_perp.length_squared())) * normal;

        r_out_perp + r_out_parallel
    }

    /// Generates a random vector with components in [0.0, 1.0).
    pub fn random() -> Self {
        Vec3::new(
            common::random_float(0.0, 1.0),
            common::random_float(0.0, 1.0),
            common::random_float(0.0, 1.0),
        )
    }

    /// Generates a random vector with components in [min, max)
    pub fn random_range(min: f64, max: f64) -> Self {
        Vec3::new(
            common::random_float(min, max),
            common::random_float(min, max),
            common::random_float(min, max),
        )
    }

    /// Generates a random vector inside the unit sphere (length < 1.0).
    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    /// Generates a random unit vector (normalized).
    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().unit_vector()
    }

    /// Generates a random vector inside the unit disk (z = 0).
    pub fn random_in_unit_disk() -> Self {
        loop {
            let p = Vec3::new(
                common::random_float(-1.0, 1.0),
                common::random_float(-1.0, 1.0),
                0.0,
            );
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    /// Generates a random offset for pixel sampling (Square distribution).
    pub fn random_offset_vector() -> Vec3 {
        Vec3::new(
            common::random_float(-0.5, 0.5),
            common::random_float(-0.5, 0.5),
            0.0,
        )
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}

// --- Operator Overloads ---

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        self + (-rhs)
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl ops::Div<Vec3> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl ops::DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_vec_eq {
        ($left:expr, $right:expr) => {
            assert_vec_eq!($left, $right, f64::EPSILON)
        };
        ($left:expr, $right:expr, $tol:expr) => {
            let l = $left;
            let r = $right;
            let dx = (l.x - r.x).abs();
            let dy = (l.y - r.y).abs();
            let dz = (l.z - r.z).abs();
            if dx > $tol || dy > $tol || dz > $tol {
                panic!(
                    "\nAssertion failed: vectors are not approximately equal\n\
                     Left:  {:?}\n\
                     Right: {:?}\n\
                     Diff:  [{}, {}, {}]\n\
                     Tol:   {}\n",
                    l, r, dx, dy, dz, $tol
                );
            }
        };
    }

    #[test]
    fn test_new_and_default() {
        let v_def = Vec3::default();
        assert_vec_eq!(v_def, Vec3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_add() {
        let res = Vec3::new(1.0, 2.0, 3.0) + Vec3::new(4.0, 5.0, 6.0);
        assert_vec_eq!(res, Vec3::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn test_add_assign() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        v += Vec3::new(1.0, 1.0, 1.0);
        assert_vec_eq!(v, Vec3::new(2.0, 3.0, 4.0));
    }

    #[test]
    fn test_sub() {
        let res = Vec3::new(4.0, 5.0, 6.0) - Vec3::new(1.0, 2.0, 3.0);
        assert_vec_eq!(res, Vec3::new(3.0, 3.0, 3.0));
    }

    #[test]
    fn test_sub_assign() {
        let mut v = Vec3::new(4.0, 5.0, 6.0);
        v -= Vec3::new(1.0, 1.0, 1.0);
        assert_vec_eq!(v, Vec3::new(3.0, 4.0, 5.0));
    }

    #[test]
    fn test_dot() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let res = v1.dot(v2);
        assert!((res - 32.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_cross() {
        let v1 = Vec3::new(1.0, 0.0, 0.0);
        let v2 = Vec3::new(0.0, 1.0, 0.0);
        let res = v1.cross(v2);
        // Cross product of X and Y axis should be Z axis
        assert_vec_eq!(res, Vec3::new(0.0, 0.0, 1.0));

        // Test arbitrary
        let v3 = Vec3::new(1.0, 2.0, 3.0);
        let v4 = Vec3::new(4.0, 5.0, 6.0);
        assert_vec_eq!(v3.cross(v4), Vec3::new(-3.0, 6.0, -3.0));
    }

    #[test]
    fn test_length() {
        let v = Vec3::new(1.0, 2.0, 2.0);
        assert!((v.length() - 3.0).abs() < f64::EPSILON);
        assert!((v.length_squared() - 9.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_unit_vector() {
        let v = Vec3::new(2.0, 0.0, 0.0);
        assert_vec_eq!(v.unit_vector(), Vec3::new(1.0, 0.0, 0.0));

        let v2 = Vec3::new(1.0, 2.0, 2.0);
        let unit = v2.unit_vector();
        assert!((unit.length() - 1.0).abs() < f64::EPSILON);
        assert_vec_eq!(unit, Vec3::new(1.0 / 3.0, 2.0 / 3.0, 2.0 / 3.0));
    }

    #[test]
    fn test_near_zero() {
        let small = Vec3::new(1e-9, 1e-9, 1e-9);
        let not_small = Vec3::new(0.1, 0.0, 0.0);
        assert!(small.near_zero());
        assert!(!not_small.near_zero());
    }

    #[test]
    fn test_neg() {
        let v = Vec3::new(1.0, -2.0, 3.0);
        assert_vec_eq!(-v, Vec3::new(-1.0, 2.0, -3.0));
    }

    #[test]
    fn test_div_scalar() {
        let v = Vec3::new(2.0, 4.0, 6.0);
        assert_vec_eq!(v / 2.0, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_div_vec() {
        // Element-wise division
        let v1 = Vec3::new(4.0, 9.0, 16.0);
        let v2 = Vec3::new(2.0, 3.0, 4.0);
        assert_vec_eq!(v1 / v2, Vec3::new(2.0, 3.0, 4.0));
    }

    #[test]
    fn test_div_assign() {
        let mut v = Vec3::new(4.0, 8.0, 12.0);
        let div = Vec3::new(2.0, 2.0, 2.0);
        v /= div;
        assert_vec_eq!(v, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_mul_scalar() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_vec_eq!(v * 2.0, Vec3::new(2.0, 4.0, 6.0));
        assert_vec_eq!(2.0 * v, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_mul_vec() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let res = v1 * v2;
        assert_vec_eq!(res, Vec3::new(4.0, 10.0, 18.0));
    }

    #[test]
    fn test_mul_assign() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        let factor = Vec3::new(2.0, 2.0, 2.0);
        v *= factor;
        assert_vec_eq!(v, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_display() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(format!("{}", v), "[1, 2, 3]");
    }

    // --- Physics / Optical Tests ---

    #[test]
    fn test_reflect() {
        // Incoming vector v hits a surface with normal n.
        // v = (1, -1, 0) -> moving 45 degrees down-right
        // n = (0, 1, 0)  -> surface normal pointing up
        // expected = (1, 1, 0) -> moving 45 degrees up-right
        let v = Vec3::new(1.0, -1.0, 0.0);
        let n = Vec3::new(0.0, 1.0, 0.0);
        let reflected = v.reflect(n);
        assert_vec_eq!(reflected, Vec3::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn test_refract_identity() {
        // If refractive index ratio is 1.0 (e.g., air to air),
        // the vector should pass through unchanged.
        let uv = Vec3::new(1.0, -1.0, 0.0).unit_vector();
        let n = Vec3::new(0.0, 1.0, 0.0);
        let etai_over_etat = 1.0;

        let refracted = uv.refract(n, etai_over_etat);
        assert_vec_eq!(refracted, uv);
    }

    #[test]
    fn test_refract_perpendicular() {
        // Light hitting straight down (0, -1, 0) should continue straight down
        // regardless of refractive index.
        let uv = Vec3::new(0.0, -1.0, 0.0);
        let n = Vec3::new(0.0, 1.0, 0.0);
        let etai_over_etat = 1.5; // Example index

        let refracted = uv.refract(n, etai_over_etat);
        assert_vec_eq!(refracted, uv);
    }

    // --- Random Generation Tests ---

    #[test]
    fn test_random_bounds() {
        for _ in 0..100 {
            let r = Vec3::random();
            assert!(r.x >= 0.0 && r.x < 1.0);
            assert!(r.y >= 0.0 && r.y < 1.0);
            assert!(r.z >= 0.0 && r.z < 1.0);
        }
    }

    #[test]
    fn test_random_range() {
        let min = -5.0;
        let max = 5.0;
        for _ in 0..100 {
            let r = Vec3::random_range(min, max);
            assert!(r.x >= min && r.x < max);
            assert!(r.y >= min && r.y < max);
            assert!(r.z >= min && r.z < max);
        }
    }

    #[test]
    fn test_random_in_unit_sphere() {
        for _ in 0..100 {
            let p = Vec3::random_in_unit_sphere();
            assert!(p.length_squared() < 1.0);
        }
    }

    #[test]
    fn test_random_unit_vector() {
        for _ in 0..100 {
            let p = Vec3::random_unit_vector();
            assert!((p.length() - 1.0).abs() < 1e-8);
        }
    }

    #[test]
    fn test_random_in_unit_disk() {
        for _ in 0..100 {
            let p = Vec3::random_in_unit_disk();
            assert!(p.length_squared() < 1.0);
            assert!(p.z.abs() < f64::EPSILON);
        }
    }
}
