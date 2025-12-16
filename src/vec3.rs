#![allow(dead_code)]

use num::{Float, zero};
use std::{fmt, ops};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3<T: Float> {
    x: T,
    y: T,
    z: T,
}

impl<T: Float> Default for Vec3<T> {
    fn default() -> Self {
        Vec3::new(zero(), zero(), zero())
    }
}

#[inline]
pub fn dot<T: Float>(lhs: Vec3<T>, rhs: Vec3<T>) -> T {
    lhs.x() * rhs.x() + lhs.y() * rhs.y() + lhs.z() * rhs.z()
}

pub fn cross<T: Float>(lhs: Vec3<T>, rhs: Vec3<T>) -> Vec3<T> {
    Vec3::new(
        lhs.y() * rhs.z() - rhs.y() * lhs.z(),
        rhs.x() * lhs.z() - lhs.x() * rhs.z(),
        lhs.x() * rhs.y() - lhs.y() * rhs.x(),
    )
}

impl<T: Float> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Vec3 { x, y, z }
    }

    pub fn x(&self) -> T {
        self.x
    }

    pub fn y(&self) -> T {
        self.y
    }

    pub fn z(&self) -> T {
        self.z
    }

    pub fn length_squared(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> T {
        self.length_squared().sqrt()
    }
}

impl fmt::Display for Vec3<f64> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}

impl fmt::Display for Vec3<f32> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}

impl<T: Float> ops::Add<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, rhs: Vec3<T>) -> Self::Output {
        Vec3::new(self.x + rhs.x(), self.y + rhs.y(), self.z + rhs.z())
    }
}

impl<T: Float> ops::Add<&Vec3<T>> for &Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, rhs: &Vec3<T>) -> Self::Output {
        *self + *rhs
    }
}

impl<T: Float> ops::Add<Vec3<T>> for &Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, rhs: Vec3<T>) -> Self::Output {
        *self + rhs
    }
}

impl<T: Float> ops::Add<&Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, rhs: &Vec3<T>) -> Self::Output {
        self + *rhs
    }
}

impl<T: Float> ops::Neg for Vec3<T> {
    type Output = Vec3<T>;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl<T: Float> ops::Neg for &Vec3<T> {
    type Output = Vec3<T>;

    fn neg(self) -> Self::Output {
        -*self
    }
}

impl<T: Float> ops::Sub<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;

    fn sub(self, rhs: Vec3<T>) -> Self::Output {
        self + (-rhs)
    }
}

impl<T: Float> ops::Sub<&Vec3<T>> for &Vec3<T> {
    type Output = Vec3<T>;

    fn sub(self, rhs: &Vec3<T>) -> Self::Output {
        *self - *rhs
    }
}

impl<T: Float> ops::Sub<&Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;

    fn sub(self, rhs: &Vec3<T>) -> Self::Output {
        self - *rhs
    }
}

impl<T: Float> ops::Sub<Vec3<T>> for &Vec3<T> {
    type Output = Vec3<T>;

    fn sub(self, rhs: Vec3<T>) -> Self::Output {
        *self - rhs
    }
}

impl<T: Float> ops::Div<T> for Vec3<T> {
    type Output = Vec3<T>;

    fn div(self, rhs: T) -> Self::Output {
        if rhs.is_zero() {
            panic!("division by zero");
        }

        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl<T: Float> ops::Div<T> for &Vec3<T> {
    type Output = Vec3<T>;

    fn div(self, rhs: T) -> Self::Output {
        *self / rhs
    }
}

impl<T: Float> ops::Mul<T> for Vec3<T> {
    type Output = Vec3<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl<T: Float> ops::Mul<T> for &Vec3<T> {
    type Output = Vec3<T>;

    fn mul(self, rhs: T) -> Self::Output {
        *self * rhs
    }
}

impl<T: Float> ops::Mul<Vec3<T>> for Vec3<T> {
    type Output = T;

    fn mul(self, rhs: Vec3<T>) -> Self::Output {
        dot(self, rhs)
    }
}

impl<T: Float> ops::Mul<&Vec3<T>> for &Vec3<T> {
    type Output = T;

    fn mul(self, rhs: &Vec3<T>) -> Self::Output {
        *self * *rhs
    }
}

impl<T: Float> ops::Mul<Vec3<T>> for &Vec3<T> {
    type Output = T;

    fn mul(self, rhs: Vec3<T>) -> Self::Output {
        *self * rhs
    }
}

impl<T: Float> ops::Mul<&Vec3<T>> for Vec3<T> {
    type Output = T;

    fn mul(self, rhs: &Vec3<T>) -> Self::Output {
        self * *rhs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let res: Vec3<f64> = Vec3::new(1.0, 2.0, 3.0) + Vec3::new(4.0, 5.0, 6.0);
        assert!((res.x() - 5.0) < 1e-8);
        assert!((res.y() - 7.0) < 1e-8);
        assert!((res.z() - 9.0) < 1e-8);
    }

    #[test]
    fn test_sub() {
        let res: Vec3<f64> = Vec3::new(4.0, 5.0, 6.0) - Vec3::new(1.0, 2.0, 3.0);
        assert!((res.x() - 3.0) < 1e-8);
        assert!((res.y() - 3.0) < 1e-8);
        assert!((res.z() - 3.0) < 1e-8);
    }

    #[test]
    fn test_dot() {
        let v1: Vec3<f64> = Vec3::new(1.0, 2.0, 3.0);
        let v2: Vec3<f64> = Vec3::new(4.0, 5.0, 6.0);
        let res: f64 = dot(v1, v2);
        assert!((res - 32.0) < 1e-8);
    }

    #[test]
    fn test_cross() {
        let v1: Vec3<f64> = Vec3::new(1.0, 2.0, 3.0);
        let v2: Vec3<f64> = Vec3::new(4.0, 5.0, 6.0);
        let res: Vec3<f64> = cross(v1, v2);
        assert!((res.x() + 3.0) < 1e-8);
        assert!((res.y() - 6.0) < 1e-8);
        assert!((res.z() - 3.0) < 1e-8);
    }

    #[test]
    fn test_length() {
        let v: Vec3<f64> = Vec3::new(1.0, 2.0, 2.0);
        let len: f64 = v.length();
        assert!((len - 3.0) < 1e-8);
    }

    #[test]
    fn test_neg() {
        let v: Vec3<f64> = Vec3::new(1.0, -2.0, 3.0);
        let neg_v: Vec3<f64> = -v;
        assert!((neg_v.x() + 1.0) < 1e-8);
        assert!((neg_v.y() - 2.0) < 1e-8);
        assert!((neg_v.z() + 3.0) < 1e-8);
    }

    #[test]
    fn test_div() {
        let v: Vec3<f64> = Vec3::new(2.0, 4.0, 6.0);
        let div_v: Vec3<f64> = v / 2.0;
        assert!((div_v.x() - 1.0) < 1e-8);
        assert!((div_v.y() - 2.0) < 1e-8);
        assert!((div_v.z() - 3.0) < 1e-8);
    }

    #[test]
    fn test_mul_scalar() {
        let v: Vec3<f64> = Vec3::new(1.0, 2.0, 3.0);
        let mul_v: Vec3<f64> = v * 2.0;
        assert!((mul_v.x() - 2.0) < 1e-8);
        assert!((mul_v.y() - 4.0) < 1e-8);
        assert!((mul_v.z() - 6.0) < 1e-8);
    }

    #[test]
    fn test_mul_vec() {
        let v1: Vec3<f64> = Vec3::new(1.0, 2.0, 3.0);
        let v2: Vec3<f64> = Vec3::new(4.0, 5.0, 6.0);
        let res: f64 = v1 * v2;
        assert!((res - 32.0) < 1e-8);
    }

    #[test]
    fn test_display() {
        let v: Vec3<f64> = Vec3::new(1.0, 2.0, 3.0);
        let v_str: String = format!("{}", v);
        assert_eq!(v_str, "[1, 2, 3]");
    }

    #[test]
    fn test_display_f32() {
        let v: Vec3<f32> = Vec3::new(1.0, 2.0, 3.0);
        let v_str: String = format!("{}", v);
        assert_eq!(v_str, "[1, 2, 3]");
    }
}
