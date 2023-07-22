mod breakout;
use macroquad::prelude::*;

use crate::breakout::Game;

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
    let mut game = Game::new();
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
