use macroquad::prelude::*;

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub speed: f32,
    pub color: Color,
    pub direction: PlayerDirection,
}

pub enum PlayerDirection {
    IDLE,
    LEFT,
    RIGHT,
}

impl Player {
    pub fn new() -> Self {
        let w: f32 = 100.0;
        let h: f32 = 10.0;
        Self {
            x: (screen_width() - w) / 2.0,
            y: screen_height() - h - 50.0,
            w,
            h,
            speed: 15.0,
            color: RED,
            direction: PlayerDirection::IDLE,
        }
    }
    pub fn left(&mut self) {
        if self.x - self.speed < 0.0 {
            self.x = 0.0;
        } else {
            self.x = self.x - self.speed;
        }
    }
    pub fn right(&mut self) {
        if self.x + self.w + self.speed > screen_width() {
            self.x = screen_width() - self.w;
        } else {
            self.x = self.x + self.speed;
        }
    }
    pub fn idle(&mut self) {}
    pub fn movement(&mut self) {
        match self.direction {
            PlayerDirection::IDLE => self.idle(),
            PlayerDirection::LEFT => self.left(),
            PlayerDirection::RIGHT => self.right(),
        }
    }
    pub fn draw(&mut self) {
        draw_rectangle(self.x, self.y, self.w, self.h, self.color);
    }
}
