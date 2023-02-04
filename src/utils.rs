use crate::{app::App, render::render};

#[allow(unused)]
pub fn close_enough(a: f32, b: f32) -> bool {
    (a-b).abs() < f32::EPSILON
}

pub fn save_to_file(app: &App) {
    let size = app.canvas.output_size().unwrap();
    let mut buf = vec![255;(size.0*size.1*3)as usize];
    for y in 0..app.camera.height as i32 {
        let line = (y * app.camera.width as i32 * 3)as usize;
        let py = (y as f32 / app.camera.height) * 2. - 1.;
        for x in 0..app.camera.width as i32 {
            let px = (x as f32 / app.camera.width) * 2. -1.;
            let color = render(&app.camera, &app.objects, &app.lights, px, py);
            let i = (x * 3)as usize + line;
            buf[i] = color.r;
            buf[i + 1] = color.g;
            buf[i + 2] = color.b;
        }
    }
    image::save_buffer("./assets/screenshot.jpeg", &buf, size.0, size.1, image::ColorType::Rgb8).unwrap();
}