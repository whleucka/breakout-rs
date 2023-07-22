pub mod ball;
pub mod player;

use macroquad::prelude::*;

use crate::breakout::ball::Ball;
use crate::breakout::player::{Player, PlayerDirection};

pub struct Game {
    pub ball: Ball,
    pub player: Player,
}

impl Game {
    pub fn new() -> Self {
        Self {
           ball: Ball::new(),
           player: Player::new(),
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
