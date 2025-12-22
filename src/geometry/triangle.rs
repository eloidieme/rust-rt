use crate::{
    geometry::hittable::{HitRecord, Hittable},
    imaging::material::MaterialKind,
    math::{interval::Interval, ray::Ray, vec3::Vec3},
};

pub struct Triangle {
    v0: Vec3,
    v1: Vec3,
    v2: Vec3,
    normal: Vec3,
    material: MaterialKind,
}

impl Triangle {
    pub fn new(v0: Vec3, v1: Vec3, v2: Vec3, material: MaterialKind) -> Self {
        let edge1 = v1 - v0;
        let edge2 = v2 - v0;
        let normal = edge1.cross(edge2).unit_vector();

        Self {
            v0,
            v1,
            v2,
            normal,
            material,
        }
    }
}

impl Hittable for Triangle {
    #[allow(non_snake_case)]
    fn hit(&self, ray: &Ray, bounds: Interval) -> Option<HitRecord> {
        const EPSILON: f64 = 1e-8;

        let E1 = self.v1 - self.v0;
        let E2 = self.v2 - self.v0;

        let P = ray.direction.cross(E2);
        let det = E1.dot(P);

        if det.abs() < EPSILON {
            return None;
        }

        let inv_det = 1.0 / det;
        let T = ray.origin - self.v0;

        let u = T.dot(P) * inv_det;
        if u < 0.0 || u > 1.0 {
            return None;
        }

        let Q = T.cross(E1);
        let v = ray.direction.dot(Q) * inv_det;

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = E2.dot(Q) * inv_det;

        if !bounds.surrounds(t) {
            return None;
        }

        let p = ray.at(t);

        Some(HitRecord::new(
            p,
            self.normal,
            t,
            ray,
            self.material.clone(),
        ))
    }
}
