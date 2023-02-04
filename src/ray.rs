use cgmath::Vector3;

#[derive(Copy, Clone)]
pub struct Ray {
    pub a: Vector3<f32>,
    pub b: Vector3<f32>,
    pub ab: Vector3<f32>
}
impl Ray {
    pub fn new(a: Vector3<f32>, b: Vector3<f32>) -> Self {
        Self {
            a,
            b,
            ab: b - a
        }
    }
}