use sdl2::pixels::Color;
use std::time::{Instant, Duration};

use crate::app::App;

pub struct Timer {
    pub last_time: Instant,
    pub min_secs: f64,
    pub secs: f64,
    pub fps: usize
}
impl Timer {
    pub fn new(max_fps: usize) -> Self {
        Self {
            last_time: Instant::now(),
            min_secs: 1. / max_fps as f64,
            secs: 0.,
            fps: 0,
        }
    }
    pub fn update(&mut self) {
        let now = Instant::now();
        self.secs = (now - self.last_time).as_secs_f64();
        if self.secs < self.min_secs {
            std::thread::sleep(Duration::from_secs_f64(self.min_secs - self.secs));
        }
        self.fps = (1. / (Instant::now() - self.last_time).as_secs_f64()) as usize;
        self.last_time = Instant::now();
    }
    #[allow(unused)]
    pub fn show_fps(&self, app: &mut App) {
        app.print(&format!("fps: {:?}", self.fps), Color::WHITE, 5, 5);
    }
}