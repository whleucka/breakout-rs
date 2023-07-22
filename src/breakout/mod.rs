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
