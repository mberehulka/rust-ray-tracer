use sdl2::pixels::Color;
use super::Object;

pub struct Plane {
    pub color: Color
}
impl Object for Plane {
    fn test_intersection(&self, ray: &crate::ray::Ray) -> Option<super::IntersectionResult> {
        None
    }
    fn base_color(&self) -> sdl2::pixels::Color {
        self.color
    }
}