use sdl2::{
    render::{Canvas, TextureCreator},
    video::{Window, WindowContext},
    Sdl, ttf::{Font, Sdl2TtfContext}, pixels::Color, rect::{Rect, Point}
};

use crate::{object::Objects, light::Lights, camera::Camera};

pub struct App {
    pub ctx: Sdl,
    pub canvas: Canvas<Window>,
    pub ttf_ctx: &'static Sdl2TtfContext,
    pub font: Font<'static, 'static>,
    pub texture_creator: TextureCreator<WindowContext>,
    pub font_size: f32,

    pub objects: Objects,
    pub lights: Lights,
    pub camera: Camera
}
impl App {
    pub fn new(title: &'static str, width: u32, height: u32) -> Self {
        let ctx = sdl2::init().unwrap();
        let ttf_ctx: &'static mut Sdl2TtfContext = Box::leak(Box::new(sdl2::ttf::init().unwrap())).into();
        let font = ttf_ctx.load_font("assets/font.ttf", 64).unwrap();
        let video = ctx.video().unwrap();
        let window = video
            .window(title, width, height)
            .resizable()
            .position_centered()
            .build().unwrap();
        let canvas = window.into_canvas().target_texture().build().unwrap();
        let texture_creator = canvas.texture_creator();

        let mut camera = Camera::new(canvas.output_size().unwrap());
        camera.update();

        Self {
            ctx,
            canvas,
            ttf_ctx,
            font,
            texture_creator,
            font_size: 0.2,
            objects: Objects::new(),
            lights: Lights::new(),
            camera
        }
    }
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        let size = self.canvas.output_size().unwrap();
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.fill_rect(Rect::new(0, 0, size.0, size.1)).unwrap()
    }
    pub fn print(
        &mut self,
        text: impl AsRef<str>,
        color: impl Into<Color>,
        x: i32,
        y: i32
    ) {
        let texture = self.texture_creator.create_texture_from_surface(
            &self.font.render(text.as_ref()).blended(color.into()).unwrap()
        ).unwrap();
        let query = texture.query();
        let dst = Some(Rect::new(x, y, (query.width as f32 * self.font_size) as u32, (query.height as f32 * self.font_size) as u32));
        self.canvas.copy(&texture, None, dst).unwrap();
    }
    #[allow(dead_code)]
    pub fn print_lines(
        &mut self,
        text: impl AsRef<str>,
        color: impl Into<Color>,
        x: i32,
        mut y: i32
    ) {
        let color = color.into();
        let line_height = (self.font.height() as f32 * self.font_size) as i32;
        for line in text.as_ref().lines() {
            self.print(line, color, x, y);
            y += line_height;
        }
    }
    pub fn set_pixel(&mut self, x: i32, y: i32, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas.draw_point(Point::new(x, y)).unwrap();
    }
}