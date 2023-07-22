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

impl Default for Ball {
    fn default() -> Self {
        Self {
            x: 150.0,
            y: 50.0,
            active: true,
            r: 5.0,
            dx: 1,
            dy: -1,
            color: WHITE,
            speed: 5.0,
        }
    }
}

impl Ball {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            ..Default::default()
        }
    }

    pub fn movement(&mut self) {
        if !self.active {
            return;
        }
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

    pub fn is_collision(&mut self, x: f32, y: f32, w: f32, h: f32) -> bool {
        // temp variables to set edges for testing
        let mut test_x: f32 = self.x;
        let mut test_y: f32 = self.y;

        // which edge is closest?
        if self.x < x {
            test_x = x; // test left edge
        } else if self.x > x + w {
            test_x = x + w; // right edge
        }
        if self.y < y {
            test_y = y; // top edge
        } else if self.y > y + h {
            test_y = y + h; // bottom edge
        }

        // get distance from closest edges
        let dist_x: f32 = self.x - test_x;
        let dist_y: f32 = self.y - test_y;
        let distance: f32 = f32::sqrt((dist_x * dist_x) + (dist_y * dist_y));

        // if the distance is less than the radius, collision!
        distance <= self.r
    }

    pub fn draw(&mut self) {
        if self.active {
            draw_circle(self.x, self.y, self.r, self.color);
        }
    }
}
