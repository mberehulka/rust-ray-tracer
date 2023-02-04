use sdl2::pixels::Color;

mod point;
pub use point::Point;

use crate::object::IntersectionResult;

pub struct IlluminateResult {
    pub color: Color,
    pub intensity: f32
}

pub trait Light: Send + Sync {
    fn illuminate(&self, intr_res: &IntersectionResult) -> Option<IlluminateResult>;
}

pub struct Lights(pub Vec<Box<dyn Light>>);
impl Lights {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    #[inline]
    pub fn add<T: Light + 'static>(&mut self, light: T) {
        self.0.push(Box::new(light))
    }
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }
    #[inline]
    pub fn get(&self, index: usize) -> &Box<dyn Light> {
        &self.0[index]
    }
}