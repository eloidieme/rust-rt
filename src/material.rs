use crate::{
    common,
    hittable::HitRecord,
    ray::Ray,
    vec3::{Color, Vec3},
};

#[derive(Debug)]
/// Result of a ray scattering off a material.
pub struct ScatteredRay {
    pub attenuation: Color,
    pub scattered: Ray,
}

/// Trait for materials that can scatter rays.
pub trait Material {
    /// Scatters an incoming ray.
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatteredRay>;
}

#[derive(Debug)]
/// A diffuse material (matte).
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    /// Creates a new Lambertian material with the given albedo.
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<ScatteredRay> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction);
        Some(ScatteredRay {
            attenuation: self.albedo,
            scattered,
        })
    }
}

#[derive(Debug)]
/// A metallic material.
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    /// Creates a new Metal material with albedo and fuzziness.
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.clamp(0.0, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatteredRay> {
        let mut reflected = r_in.direction.reflect(rec.normal);
        reflected = reflected.unit_vector() + Vec3::random_unit_vector() * self.fuzz;
        let scattered = Ray::new(rec.p, reflected);
        if scattered.direction.dot(rec.normal) > 0.0 {
            Some(ScatteredRay {
                attenuation: self.albedo,
                scattered,
            })
        } else {
            None
        }
    }
}

#[derive(Debug)]
/// A dielectric material (glass, water, etc.).
pub struct Dielectric {
    refraction_ratio: f64,
}

impl Dielectric {
    /// Creates a new Dielectric material with the given refractive index.
    pub fn new(refraction_ratio: f64) -> Self {
        Self { refraction_ratio }
    }

    fn reflectance(cosine: f64, refraction_ratio: f64) -> f64 {
        let mut r0 = (1.0 - refraction_ratio) / (1.0 + refraction_ratio);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatteredRay> {
        let attenuation = Color::new(1.0, 1.0, 1.0);

        let refraction_ratio = if rec.front_face {
            1.0 / self.refraction_ratio
        } else {
            self.refraction_ratio
        };

        let unit_direction = r_in.direction.unit_vector();
        let cos_theta = f64::min(-unit_direction.dot(rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction = if cannot_refract
            || Self::reflectance(cos_theta, refraction_ratio) > common::random()
        {
            unit_direction.reflect(rec.normal)
        } else {
            unit_direction.refract(rec.normal, refraction_ratio)
        };

        Some(ScatteredRay {
            attenuation,
            scattered: Ray::new(rec.p, direction),
        })
    }
}
