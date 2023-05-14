use macroquad::prelude::*;

struct Player {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    color: Color,
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
            color: RED,
        }
    }
    pub fn draw(&mut self) {
        draw_rectangle(self.x, self.y, self.w, self.h, self.color);
    }
}

struct Game {
    player: Player,
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
        game.player.draw();
        next_frame().await
    }
}
