#![allow(dead_code)]

use std::{fmt, ops};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
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

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
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
        let s: f64 = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    #[inline]
    pub fn dot(lhs: Vec3, rhs: Vec3) -> f64 {
        lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
    }

    pub fn cross(lhs: Vec3, rhs: Vec3) -> Vec3 {
        Vec3::new(
            lhs.y * rhs.z - rhs.y * lhs.z,
            rhs.x * lhs.z - lhs.x * rhs.z,
            lhs.x * rhs.y - lhs.y * rhs.x,
        )
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
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

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        if rhs.abs() < 1e-20 {
            panic!("division by zero");
        }

        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let res: Vec3 = Vec3::new(1.0, 2.0, 3.0) + Vec3::new(4.0, 5.0, 6.0);
        assert!((res.x() - 5.0) < 1e-8);
        assert!((res.y() - 7.0) < 1e-8);
        assert!((res.z() - 9.0) < 1e-8);
    }

    #[test]
    fn test_sub() {
        let res: Vec3 = Vec3::new(4.0, 5.0, 6.0) - Vec3::new(1.0, 2.0, 3.0);
        assert!((res.x() - 3.0) < 1e-8);
        assert!((res.y() - 3.0) < 1e-8);
        assert!((res.z() - 3.0) < 1e-8);
    }

    #[test]
    fn test_dot() {
        let v1: Vec3 = Vec3::new(1.0, 2.0, 3.0);
        let v2: Vec3 = Vec3::new(4.0, 5.0, 6.0);
        let res: f64 = Vec3::dot(v1, v2);
        assert!((res - 32.0) < 1e-8);
    }

    #[test]
    fn test_cross() {
        let v1: Vec3 = Vec3::new(1.0, 2.0, 3.0);
        let v2: Vec3 = Vec3::new(4.0, 5.0, 6.0);
        let res: Vec3 = Vec3::cross(v1, v2);
        assert!((res.x() + 3.0) < 1e-8);
        assert!((res.y() - 6.0) < 1e-8);
        assert!((res.z() - 3.0) < 1e-8);
    }

    #[test]
    fn test_length() {
        let v: Vec3 = Vec3::new(1.0, 2.0, 2.0);
        let len: f64 = v.length();
        assert!((len - 3.0) < 1e-8);
    }

    #[test]
    fn test_neg() {
        let v: Vec3 = Vec3::new(1.0, -2.0, 3.0);
        let neg_v: Vec3 = -v;
        assert!((neg_v.x() + 1.0) < 1e-8);
        assert!((neg_v.y() - 2.0) < 1e-8);
        assert!((neg_v.z() + 3.0) < 1e-8);
    }

    #[test]
    fn test_div() {
        let v: Vec3 = Vec3::new(2.0, 4.0, 6.0);
        let div_v: Vec3 = v / 2.0;
        assert!((div_v.x() - 1.0) < 1e-8);
        assert!((div_v.y() - 2.0) < 1e-8);
        assert!((div_v.z() - 3.0) < 1e-8);
    }

    #[test]
    fn test_mul_scalar() {
        let v: Vec3 = Vec3::new(1.0, 2.0, 3.0);
        let mul_v: Vec3 = v * 2.0;
        assert!((mul_v.x() - 2.0) < 1e-8);
        assert!((mul_v.y() - 4.0) < 1e-8);
        assert!((mul_v.z() - 6.0) < 1e-8);
    }

    #[test]
    fn test_mul_vec() {
        let v1: Vec3 = Vec3::new(1.0, 2.0, 3.0);
        let v2: Vec3 = Vec3::new(4.0, 5.0, 6.0);
        let res: f64 = Vec3::dot(v1, v2);
        assert!((res - 32.0) < 1e-8);
    }

    #[test]
    fn test_display() {
        let v: Vec3 = Vec3::new(1.0, 2.0, 3.0);
        let v_str: String = format!("{}", v);
        assert_eq!(v_str, "[1, 2, 3]");
    }

    #[test]
    fn test_display_f32() {
        let v: Vec3 = Vec3::new(1.0, 2.0, 3.0);
        let v_str: String = format!("{}", v);
        assert_eq!(v_str, "[1, 2, 3]");
    }
}
