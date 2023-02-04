use sdl2::pixels::Color;

use cgmath::InnerSpace;

use crate::{object::{Object, IntersectionResult, Transform}, ray::Ray};

/// at² + bt + c = 0
/// 
/// D = direction
/// P = position
/// 
/// a = D ⋅ D
/// b = 2(P ⋅ D)
/// c = (P ⋅ P) - r²
/// t = ( -b ± √(b²-4ac) ) / 2a
/// b² - 4ac > 0
pub struct Sphere {
    pub transform: Transform,
    pub color: Color
}
impl Sphere {
    pub fn new(transform: impl Into<Transform>, color: impl Into<Color>) -> Self {
        Self {
            transform: transform.into(),
            color: color.into()
        }
    }
}
impl Object for Sphere {
    fn test_intersection(&self, ray: &Ray) -> Option<IntersectionResult> {
        let mut ray = ray.clone();
        self.transform.inverse_transform_ray(&mut ray);
        
        let vhat = ray.ab.normalize();
        
        // let a = D² = always 1., since D is normalized
        let b = 2. * ray.a.dot(vhat);
        let c = ray.a.dot(ray.a) - 1.;

        let intr_test = (b*b) - 4. * c;
        if intr_test > 0. {
            let sqrt = intr_test.sqrt();

            let intr1 = (-b + sqrt) * 0.5;
            if intr1 < 0. { return None } // part of the object is behind the camera

            let intr2 = (-b - sqrt) * 0.5;
            if intr2 < 0. { return None } // part of the object is behind the camera

            // determine which point of intersection is closest to the camera
            let mut intr_point = if intr1 < intr2 {
                ray.a + (vhat * intr1)
            } else {
                ray.a + (vhat * intr2)
            };
            
            self.transform.transform_point(&mut intr_point);

            let normal = (intr_point - self.transform.world_origin()).normalize();
            
            return Some(IntersectionResult {
                point: intr_point,
                normal
            })
        } else {
            None // object not visible
        }
    }
    fn base_color(&self) -> sdl2::pixels::Color {
        self.color
    }
}