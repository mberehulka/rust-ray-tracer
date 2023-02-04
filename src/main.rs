use sdl2::{event::{Event, WindowEvent}, keyboard::Keycode, pixels::Color};
use utils::save_to_file;

mod app;
mod ray;
mod camera;
mod object;
mod utils;
mod timer;
mod light;
mod render;

fn main() {
    let mut app = app::App::new("Rust ray tracer", 800, 600);
    let mut timer = timer::Timer::new(60);
    
    app.objects.add(object::Sphere::new(([ -1.5, 0., 0. ],[ 0., 0., 0. ],[  0.5,  0.5, 0.75 ]), Color::RED));
    app.objects.add(object::Sphere::new(([   0., 0., 0. ],[ 0., 0., 0. ],[ 0.75,  0.5,  0.5 ]), Color::GREEN));
    app.objects.add(object::Sphere::new(([  1.5, 0., 0. ],[ 0., 0., 0. ],[ 0.75, 0.75, 0.75 ]), Color::BLUE));

    app.lights.add(light::Point::new([5., -10., -5.], Color::WHITE, 1.));
    
    loop {
        for event in app.ctx.event_pump().unwrap().poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } | Event::Quit { .. } => return,
                Event::Window { win_event: WindowEvent::Resized(width, height), .. } =>
                    app.camera.resize(width as f32, height as f32),
                Event::KeyDown { keycode: Some(Keycode::P), .. } => save_to_file(&app),
                _ => {}
            }
        }
        
        app.camera.update();

        for y in 0..app.camera.height as i32 {
            let py = (y as f32 / app.camera.height) * 2. - 1.;
            for x in 0..app.camera.width as i32 {
                let px = (x as f32 / app.camera.width) * 2. -1.;
                app.set_pixel(x, y, render::render(&app.camera, &app.objects, &app.lights, px, py));
            }
        }

        timer.update();
        // timer.show_fps(&mut app);
        app.canvas.present();
    }
}