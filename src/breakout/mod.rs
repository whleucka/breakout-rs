pub mod ball;
pub mod brick;
pub mod player;

use macroquad::prelude::*;

use crate::breakout::ball::Ball;
use crate::breakout::brick::Brick;
use crate::breakout::player::{Player, PlayerDirection};

pub struct Game {
    pub ball: Ball,
    pub player: Player,
    pub bricks: Vec<Brick>,
    pub level: i32,
}

impl Game {
    pub fn new() -> Self {
        let player = Player::default();
        let x = (screen_width() - player.w) / 2.0;
        let y = screen_height() - player.h - 50.0;
        Self {
            ball: Ball::new(x + player.w / 2.0, y),
            player: Player::new(x, y),
            // Vector of bricks
            bricks: Vec::new(),
            level: 0,
        }
    }

    pub async fn run(&mut self) {
        self.load_level(1);
        loop {
            clear_background(BLACK);
            self.event_handler();

            // Move the player and the ball x,y coordinates
            self.player.movement();
            self.ball.movement();

            self.detect_collision();

            // Draw player and ball
            self.player.draw();
            self.ball.draw();

            // Draw all the bricks
            self.bricks
                .iter_mut()
                .filter(|brick| brick.active)
                .for_each(|brick| brick.draw());

            next_frame().await
        }
    }

    pub fn detect_collision(&mut self) {
        self.bricks
            .iter_mut()
            .filter(|brick| brick.active)
            .for_each(|brick| {
                if self.ball.is_collision(brick.x, brick.y, brick.w, brick.h) {
                    brick.active = false;
                    self.ball.dx = -1 * self.ball.dx;
                    self.ball.dy = -1 * self.ball.dy;
                }
            })
    }

    pub fn load_level(&mut self, level: i32) {
        self.level = level;
        self.load_bricks();
    }

    pub fn load_bricks(&mut self) {
        let defaults = Brick::default();
        let x_offset: f32 = 30.0;
        let y_offest: f32 = 20.0;
        let rows: i32 = self.level + 5;
        let cols: i32 = (screen_width() / defaults.w) as i32 - 1;
        for h in 0..rows {
            for i in 0..cols {
                let x: f32 = x_offset + defaults.x + defaults.w * i as f32;
                let y: f32 = y_offest + defaults.y + defaults.h * h as f32;
                let brick = Brick::new(x, y);
                self.bricks.push(brick);
            }
        }
    }

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
