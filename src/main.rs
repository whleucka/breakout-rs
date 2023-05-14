use macroquad::prelude::*;

struct Ball {
    active: bool,
    x: f32,
    y: f32,
    dx: i8,
    dy: i8,
    r: f32,
    color: Color,
    speed: f32,
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

struct Player {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    speed: f32,
    color: Color,
    direction: PlayerDirection,
}

enum PlayerDirection {
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

struct Game {
    ball: Ball,
    player: Player,
}

impl Game {
    pub fn event_handler(&mut self) {
        if is_key_down(KeyCode::H) || is_key_down(KeyCode::Left) {
            self.player.direction = PlayerDirection::LEFT;
        } else if is_key_down(KeyCode::L) || is_key_down(KeyCode::Right) {
            self.player.direction = PlayerDirection::RIGHT;
        } else {
            self.player.direction = PlayerDirection::IDLE;
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
        ball: Ball::new(),
        player: Player::new(),
    };
    loop {
        clear_background(BLACK);
        game.event_handler();
        game.player.movement();
        game.ball.movement();
        game.player.draw();
        game.ball.draw();
        next_frame().await
    }
}
