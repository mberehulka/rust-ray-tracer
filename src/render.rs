use sdl2::pixels::Color;

use crate::{camera::Camera, object::Objects, light::Lights};

#[inline]
pub fn render(
    camera: &Camera,
    objects: &Objects,
    lights: &Lights,
    pixel_x: f32,
    pixel_y: f32
) -> Color {
    let mut ray = camera.generate_ray(pixel_x, pixel_y);
    let mut color = Color::BLACK;

    // loop throw objects
    for object_id in 0..objects.len() {
        let object = objects.get(object_id);

        // test for intersection
        if let Some(intr_res) = object.test_intersection(&mut ray) {
            let mut c = object.base_color();
            let mut visible = false;

            // loop throw lights
            for light_id in 0..lights.len() {
                let light = lights.get(light_id);

                // illuminate object with the light
                if let Some(ill_res) = light.illuminate(&intr_res) {
                    c.r = (c.r as f32 * ill_res.intensity) as u8;
                    c.g = (c.g as f32 * ill_res.intensity) as u8;
                    c.b = (c.b as f32 * ill_res.intensity) as u8;
                    visible = true;
                }

            }

            if visible {
                color = c
            }
        }

    }

    color
}