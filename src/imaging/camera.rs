use crate::math::{ray::Ray, vec3::Vec3};

#[derive(Debug, Clone)]
pub struct Camera {
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    defocus_angle: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn builder() -> CameraBuilder {
        CameraBuilder::default()
    }

    /// Generates a ray for a given normalized coordinate (s, t).
    /// s and t should be in the range [0.0, 1.0].
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let pixel_center = self.pixel00_loc + (self.pixel_delta_u * s) + (self.pixel_delta_v * t);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };

        let ray_direction = pixel_center - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        let p = Vec3::random_in_unit_disk();
        self.center + (self.defocus_disk_u * p.x) + (self.defocus_disk_v * p.y)
    }
}

pub struct CameraBuilder {
    aspect_ratio: f64,
    vertical_fov: f64,
    lookfrom: Vec3,
    lookat: Vec3,
    vup: Vec3,
    defocus_angle: f64,
    focus_dist: f64,
}

impl Default for CameraBuilder {
    fn default() -> Self {
        Self {
            aspect_ratio: 16.0 / 9.0, // Standard HDTV
            vertical_fov: 20.0,       // Standard telephoto-ish lens
            lookfrom: Vec3::new(13.0, 2.0, 3.0),
            lookat: Vec3::new(0.0, 0.0, 0.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0, // Perfect sharp focus
            focus_dist: 10.0,
        }
    }
}

impl CameraBuilder {
    pub fn aspect_ratio(mut self, ratio: f64) -> Self {
        self.aspect_ratio = ratio;
        self
    }

    pub fn fov(mut self, vertical_fov: f64) -> Self {
        self.vertical_fov = vertical_fov;
        self
    }

    pub fn look_from(mut self, from: Vec3) -> Self {
        self.lookfrom = from;
        self
    }

    pub fn look_at(mut self, at: Vec3) -> Self {
        self.lookat = at;
        self
    }

    #[allow(dead_code)]
    pub fn vup(mut self, vup: Vec3) -> Self {
        self.vup = vup;
        self
    }

    pub fn defocus_angle(mut self, angle: f64) -> Self {
        self.defocus_angle = angle;
        self
    }

    pub fn focus_dist(mut self, dist: f64) -> Self {
        self.focus_dist = dist;
        self
    }

    pub fn build(self) -> Camera {
        let theta = self.vertical_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * self.aspect_ratio;

        let w = (self.lookfrom - self.lookat).unit_vector();
        let u = self.vup.cross(w).unit_vector();
        let v = w.cross(u);

        let viewport_u = u * viewport_width;
        let viewport_v = -v * viewport_height;

        let center = self.lookfrom;
        let viewport_upper_left =
            center - (w * self.focus_dist) - viewport_u / 2.0 - viewport_v / 2.0;

        let defocus_radius = self.focus_dist * (self.defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Camera {
            center,
            pixel_delta_u: viewport_u,
            pixel_delta_v: viewport_v,
            pixel00_loc: viewport_upper_left,
            defocus_angle: self.defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
        }
    }
}
