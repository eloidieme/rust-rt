use crate::math::vec3::{Color, Vec3};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SceneConfig {
    pub width: u32,
    pub aspect_ratio: f64,
    pub camera: CameraConfig,
    pub objects: Vec<ObjectConfig>,
    #[serde(default = "default_background")]
    pub background: Background,
}

fn default_background() -> Background {
    Background::VerticalGradient {
        top: Color::new(0.5, 0.7, 1.0),
        bottom: Color::new(1.0, 1.0, 1.0),
    }
}

#[derive(Deserialize)]
pub struct CameraConfig {
    pub look_from: Vec3,
    pub look_at: Vec3,
    pub vup: Vec3,
    pub fov: f64,
    pub defocus_angle: f64,
    pub focus_dist: f64,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum ObjectConfig {
    Sphere {
        center: Vec3,
        radius: f64,
        material: MaterialConfig,
    },
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum MaterialConfig {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f64 },
    Dielectric { index: f64 },
}

#[derive(Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Background {
    Solid {
        color: Color,
    },
    VerticalGradient {
        top: Color,
        bottom: Color,
    },
    HorizontalGradient {
        left: Color,
        right: Color,
    },
    BilinearGradient {
        top_left: Color,
        top_right: Color,
        bottom_left: Color,
        bottom_right: Color,
    },
}

impl Background {
    pub fn color_at(&self, u: f64, v: f64) -> Color {
        match self {
            Background::Solid { color } => *color,
            Background::VerticalGradient { top, bottom } => *bottom * (1.0 - v) + *top * v,
            Background::HorizontalGradient { left, right } => *left * (1.0 - u) + *right * u,
            Background::BilinearGradient {
                top_left,
                top_right,
                bottom_left,
                bottom_right,
            } => {
                let top = *top_left * (1.0 - u) + *top_right * u;
                let bottom = *bottom_left * (1.0 - u) + *bottom_right * u;
                bottom * (1.0 - v) + top * v
            }
        }
    }
}
