use macroquad::prelude::*;

pub struct Brick {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub color: Color,
}

impl Default for Brick {
    fn default() -> Self {
        Self {
            x: (screen_width() - 100.0) / 2.0,
            y: screen_height() - 10.0 - 50.0,
            w: 100.0,
            h: 10.0,
            color: RED,
        }
    }
}

impl Brick {
    pub fn new(w: f32, h: f32) -> Self {
        Self {
            x: (screen_width() - w) / 2.0,
            y: screen_height() - h - 50.0,
            w,
            h,
            color: WHITE,
        }
    }
    
    pub fn draw(&mut self) {
        draw_rectangle(self.x, self.y, self.w, self.h, self.color);
    }
}
