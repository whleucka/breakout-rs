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

impl Default for Player {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 150.0,
            w: 100.0,
            h: 10.0,
            speed: 15.0,
            color: RED,
            direction: PlayerDirection::IDLE,
        }
    }
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            ..Default::default()
        }
    }
    pub fn is_collision(&mut self, x: f32, y: f32, r: f32) -> bool {
        let top_left = self.x;
        let top_right = self.x + self.w;
        let top = self.y;
        let bottom = self.y + self.h;
        if x + r > top_left && x + r < top_right {
            if y + r > top && y + r < bottom {
                return true;
            }
        } 
        return false;
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
