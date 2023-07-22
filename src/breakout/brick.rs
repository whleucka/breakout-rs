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
            x: 0.0,
            y: 0.0,
            w: 40.0,
            h: 10.0,
            color: WHITE,
        }
    }
}

impl Brick {
    // The new method will spread the defaults
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            ..Default::default()
        }
    }

    pub fn draw(&mut self) {
        draw_rectangle(self.x, self.y, self.w, self.h, self.color);
    }
}
