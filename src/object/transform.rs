use std::sync::Mutex;

use cgmath::{Vector3, Quaternion, ElementWise, Rotation, Rotation3, Deg};

use crate::ray::Ray;

pub struct Transform {
    pub translation: Vector3<f32>,
    pub rotation: Quaternion<f32>,
    pub scale: Vector3<f32>,

    pub inv_rotation: Mutex<Option<Quaternion<f32>>>,
    pub world_origin: Mutex<Option<Vector3<f32>>>
}

#[allow(dead_code)]
impl Transform {
    pub fn new(translation: Vector3<f32>, rotation: Quaternion<f32>, scale: Vector3<f32>) -> Self {
        Self {
            translation,
            rotation,
            scale,
            ..Default::default()
        }
    }
    pub fn from_translation(translation: Vector3<f32>) -> Self {
        Self { translation, ..Default::default() }
    }
    pub fn from_rotation(rotation: Quaternion<f32>) -> Self {
        Self { rotation: rotation.invert(), ..Default::default() }
    }
    pub fn from_scale(scale: Vector3<f32>) -> Self {
        Self { scale, ..Default::default() }
    }

    pub fn inv_rotation(&self) -> Quaternion<f32> {
        let mut inv_rotation = self.inv_rotation.lock().unwrap();
        if let Some(v) = *inv_rotation {
            v
        } else {
            let v = self.rotation.invert();
            *inv_rotation = Some(v);
            v
        }
    }
    pub fn world_origin(&self) -> Vector3<f32> {
        let mut world_origin = self.world_origin.lock().unwrap();
        if let Some(v) = *world_origin {
            v
        } else {
            let mut v = [0.;3].into();
            self.transform_point(&mut v);
            *world_origin = Some(v);
            v
        }
    }
    pub fn reset_inv_rotation(&self) {
        *self.inv_rotation.lock().unwrap() = None
    }
    pub fn reset_world_origin(&self) {
        *self.world_origin.lock().unwrap() = None
    }
    
    pub fn transform_point(&self, point: &mut Vector3<f32>) {
        *point += self.translation;
        *point = point.mul_element_wise(self.scale);
        *point = self.rotation * *point;
    }
    pub fn inverse_transform_point(&self, point: &mut Vector3<f32>) {
        *point += self.translation;
        *point = point.div_element_wise(self.scale);
        *point = self.inv_rotation() * *point;
    }
    pub fn transform_ray(&self, ray: &mut Ray) {
        self.transform_point(&mut ray.a);
        self.transform_point(&mut ray.b);
        ray.ab = ray.b - ray.a;
    }
    pub fn inverse_transform_ray(&self, ray: &mut Ray) {
        self.inverse_transform_point(&mut ray.a);
        self.inverse_transform_point(&mut ray.b);
        ray.ab = ray.b - ray.a;
    }
    
    pub fn set_translation(&mut self, v: Vector3<f32>) {
        self.translation = v;
        self.reset_world_origin();
    }
    pub fn set_rotation(&mut self, v: Quaternion<f32>) {
        self.rotation = v;
        self.reset_inv_rotation();
        self.reset_world_origin();
    }
    pub fn set_scale(&mut self, v: Vector3<f32>) {
        self.scale = v;
        self.reset_world_origin();
    }
    pub fn translate(&mut self, v: Vector3<f32>) {
        self.translation += v;
        self.reset_world_origin();
    }
    pub fn rotate(&mut self, v: Quaternion<f32>) {
        self.rotation = self.rotation * v;
        self.reset_inv_rotation();
        self.reset_world_origin();
    }
    pub fn scale(&mut self, v: Vector3<f32>) {
        self.scale = self.scale.mul_element_wise(v);
        self.reset_world_origin();
    }
}
impl Default for Transform {
    fn default() -> Self {
        Self {
            translation: [0.;3].into(),
            rotation: Quaternion::new(1., 0., 0., 0.),
            scale: [1.;3].into(),

            inv_rotation: Mutex::new(None),
            world_origin: Mutex::new(None)
        }
    }
}
impl Into<Transform> for ([f32;3], [f32;3], [f32;3]) {
    fn into(self) -> Transform {
        Transform {
            translation: self.0.into(),
            rotation:
                Quaternion::from_angle_z(Deg(self.1[0])) *
                Quaternion::from_angle_y(Deg(self.1[1])) *
                Quaternion::from_angle_x(Deg(self.1[2])),
            scale: self.2.into(),
            ..Default::default()
        }
    }
}