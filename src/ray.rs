#![allow(dead_code)]

use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    #[inline]
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
    pub fn origin(&self) -> Vec3 {
        self.origin
    }
    pub fn direction(&self) -> Vec3 {
        self.direction
    }
}
