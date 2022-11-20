mod utils;

use fish_engine::{self, Announcement, Card, Cards, Game, GameStage, Player, Random};

use log::info;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Engine {
    game: Game,
}

#[wasm_bindgen]
impl Engine {
    pub fn new() -> Engine {
        console_error_panic_hook::set_once();
        console_log::init_with_level(log::Level::Debug);

        let mut game = Game::new(
            5,
            vec![
                Player::new(Box::new(Random::default())),
                Player::new(Box::new(Random::default())),
                Player::new(Box::new(Random::default())),
                Player::new(Box::new(Random::default())),
                Player::new(Box::new(Random::default())),
                Player::new(Box::new(Random::default())),
            ],
        );

        Engine { game }
    }

    pub fn step(&mut self) {
        let pre = self.game.announcements.len();

        match &self.game.stage {
            GameStage::Dealing { .. } => {
                self.game.step();
            }
            GameStage::Playing { .. } => {
                for i in 0..100 {
                    self.game.step();
                    if self.game.announcements.len() != pre {
                        break;
                    }
                }
            }
            GameStage::Done { .. } => {}
        }
    }

    pub fn game_state(&self) -> String {
        serde_json::to_string(&self.game).unwrap()
    }
}

// #[wasm_bindgen]
// pub fn greet() {
//     console_error_panic_hook::set_once();
//     console_log::init_with_level(log::Level::Debug);

//     let mut game = Game::new(10, vec![Player::new(Box::new(Random::default()))]);

//     info!("Hello, fish-engine :) {game:?}");
// }
