use cgmath::{Vector3, InnerSpace};

use crate::ray::Ray;

pub const FORWARD: Vector3<f32> = Vector3 { x: 0., y: 0., z: 1. };

#[derive(Debug, Clone)]
pub struct Camera {
    pub width: f32,
    pub height: f32,
    
    pub position: Vector3<f32>,
    pub lookat: Vector3<f32>,
    pub length: f32,
    pub horizontal_size: f32,
    pub aspect_ratio: f32,

    pub direction: Vector3<f32>,
    pub centre: Vector3<f32>,
    pub u: Vector3<f32>,
    pub v: Vector3<f32>
}
impl Camera {
    pub fn new(canvas_size: (u32, u32)) -> Self {
        Self {
            width: canvas_size.0 as f32,
            height: canvas_size.1 as f32,
            
            position: Vector3::new(0., -10., 0.),
            lookat: Vector3::new(0., 0., 0.),
            length: 1.,
            horizontal_size: 0.25,
            aspect_ratio: canvas_size.0 as f32 / canvas_size.1 as f32,
            
            direction: Vector3::new(0., 0., 1.),
            centre: Vector3::new(0., 0., 0.),
            u: Vector3::new(0., 0., 0.),
            v: Vector3::new(0., 0., 0.)
        }
    }
    pub fn resize(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
        self.aspect_ratio = width / height;
        self.update()
    }
    /// Return the screen point position into the world coordinates
    /// Pixel coordinates must be between -1. and 1.
    pub fn generate_ray(&self, pixel_x: f32, pixel_y: f32) -> Ray {
        let world_position = self.centre + (self.u * pixel_x) + (self.v * pixel_y);
        Ray::new(self.position, world_position)
    }
    pub fn update(&mut self) {
        self.direction = (self.lookat - self.position).normalize();
        self.u = self.direction.cross(FORWARD).normalize() * self.horizontal_size;
        self.v = self.u.cross(self.direction).normalize() * (self.horizontal_size / self.aspect_ratio);
        self.centre = self.position + (self.length * self.direction);
    }
}