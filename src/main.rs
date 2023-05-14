use macroquad::prelude::*;

struct Player {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    speed: f32,
    color: Color,
    direction: Direction,
}

enum Direction {
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
            y: screen_height() - h - 100.0,
            w,
            h,
            speed: 15.0,
            color: RED,
            direction: Direction::IDLE,
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
            Direction::IDLE => self.idle(),
            Direction::LEFT => self.left(),
            Direction::RIGHT => self.right(),
        }
    }
    pub fn draw(&mut self) {
        draw_rectangle(self.x, self.y, self.w, self.h, self.color);
    }
}

struct Game {
    player: Player,
}

impl Game {
    pub fn event_handler(&mut self) {
        if is_key_down(KeyCode::H) || is_key_down(KeyCode::Left) {
            self.player.direction = Direction::LEFT;
        } else if is_key_down(KeyCode::L) || is_key_down(KeyCode::Right) {
            self.player.direction = Direction::RIGHT;
        } else {
            self.player.direction = Direction::IDLE;
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "breakout-rs".to_owned(),
        fullscreen: false,
        window_width: 1024,
        window_height: 768,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game {
        player: Player::new(),
    };
    loop {
        clear_background(BLACK);
        game.event_handler();
        game.player.movement();
        game.player.draw();
        next_frame().await
    }
}
