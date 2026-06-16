use macroquad::prelude::*;

mod game;

use game::{config::GameConfig, engine::GameEngine, logic::DefaultGameLogic};

// ---------------------------------------------------------------------------
// Window configuration
// ---------------------------------------------------------------------------

fn window_conf() -> Conf {
    let cfg = GameConfig::default();
    Conf {
        window_title: "Rust Snake".to_owned(),
        window_width: cfg.window_width as i32,
        window_height: cfg.window_height as i32,
        window_resizable: false,
        ..Default::default()
    }
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

#[macroquad::main(window_conf)]
async fn main() {
    let config = GameConfig::default();

    // Swap `DefaultGameLogic` for your own `GameLogic` implementation to
    // customise the snake rules.
    let logic = DefaultGameLogic::new();

    let mut engine = GameEngine::new(config, logic);
    engine.run().await;
}
