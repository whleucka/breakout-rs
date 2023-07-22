use macroquad::prelude::*;

pub struct Ball {
    pub active: bool,
    pub x: f32,
    pub y: f32,
    pub dx: i8,
    pub dy: i8,
    pub r: f32,
    pub color: Color,
    pub speed: f32,
}

impl Ball {
    pub fn new() -> Self {
        Self {
            active: true,
            x: 150.0,
            y: 50.0,
            r: 5.0,
            dx: 1,
            dy: -1,
            color: WHITE,
            speed: 5.0,
        }
    }
    pub fn movement(&mut self) {
        if self.x - self.r < 0.0 {
            self.dx = 1;
        } else if self.x + self.r > screen_width() {
            self.dx = -1;
        }
        if self.y - self.r < 0.0 {
            self.dy = 1;
        } else if self.y + self.r > screen_height() {
            self.dy = -1;
            // Ball is dead
            self.active = false;
        }
        self.x += self.speed * self.dx as f32;
        self.y += self.speed * self.dy as f32;
    }
    pub fn draw(&mut self) {
        if self.active {
            draw_circle(self.x, self.y, self.r, self.color);
        }
    }
}

