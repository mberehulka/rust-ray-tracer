use std::f32::consts::PI;

use sdl2::pixels::Color;
use cgmath::{Vector3, InnerSpace};

use super::{Light, IlluminateResult};

pub struct Point {
    pub position: Vector3<f32>,
    pub color: Color,
    pub intensity: f32
}
impl Point {
    pub fn new(position: impl Into<Vector3<f32>>, color: impl Into<Color>, intensity: f32) -> Self {
        Self {
            position: position.into(),
            color: color.into(),
            intensity
        }
    }
}
impl Light for Point {
    fn illuminate(&self, intr_res: &crate::object::IntersectionResult) -> Option<IlluminateResult> {
        let light_dir = (self.position - intr_res.point).normalize();
        let angle = intr_res.normal.dot(light_dir).acos();
        if angle > PI / 2. {
            None // Light not hitting the surface
        } else {
            Some(IlluminateResult {
                color: self.color,
                intensity: self.intensity * (1. - (angle / (PI / 2.)))
            })
        }
    }
}