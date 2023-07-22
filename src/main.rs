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
    game.load_level(1);
    loop {
        clear_background(BLACK);
        game.event_handler();

        // Move the player and the ball x,y coordinates
        game.player.movement();
        game.ball.movement();

        // Draw player and ball
        game.player.draw();
        game.ball.draw();

        // Draw all the bricks
        game.bricks.iter_mut().for_each(|brick| brick.draw());

        next_frame().await
    }
}
