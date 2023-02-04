use cgmath::Vector3;
use sdl2::pixels::Color;

use crate::ray::Ray;

mod sphere;
pub use sphere::Sphere;
mod transform;
pub use transform::Transform;
mod plane;
pub use plane::Plane;

pub struct IntersectionResult {
    pub point: Vector3<f32>,
    pub normal: Vector3<f32>
}

#[allow(unused_variables)]
pub trait Object: Send + Sync {
    fn test_intersection(&self, ray: &Ray) -> Option<IntersectionResult>;
    fn base_color(&self) -> Color;
}

pub struct Objects(pub Vec<Box<dyn Object>>);
impl Objects {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    #[inline]
    pub fn add<T: Object + 'static>(&mut self, object: T) {
        self.0.push(Box::new(object))
    }
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }
    #[inline]
    pub fn get(&self, index: usize) -> &Box<dyn Object> {
        &self.0[index]
    }
}