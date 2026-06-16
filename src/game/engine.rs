use macroquad::input::{is_key_pressed, KeyCode};
use macroquad::time::get_frame_time;
use macroquad::window::next_frame;

use crate::game::{
    config::GameConfig,
    input::handle_input,
    logic::GameLogic,
    renderer::render,
    state::{GameState, GameStatus},
};

// ---------------------------------------------------------------------------
// GameEngine
// ---------------------------------------------------------------------------

/// The backbone of the snake game.
///
/// `GameEngine` owns the [`GameState`] and a user-supplied [`GameLogic`]
/// implementation.  It drives the game loop: polling input, advancing the
/// simulation at a fixed tick rate, and rendering each frame.
///
/// # Usage
///
/// ```rust,ignore
/// #[macroquad::main(window_conf)]
/// async fn main() {
///     let config = GameConfig::default();
///     let logic  = DefaultGameLogic::new();   // or your own impl
///     let mut engine = GameEngine::new(config, logic);
///     engine.run().await;
/// }
/// ```
pub struct GameEngine<L: GameLogic> {
    state: GameState,
    logic: L,
    /// Seconds accumulated since the last game tick.
    tick_accumulator: f32,
}

impl<L: GameLogic> GameEngine<L> {
    /// Create a new engine with the given configuration and game-logic
    /// implementation.
    pub fn new(config: GameConfig, logic: L) -> Self {
        let state = GameState::new(config);
        Self {
            state,
            logic,
            tick_accumulator: 0.0,
        }
    }

    /// Run the game loop indefinitely (until the window is closed).
    ///
    /// This is `async` because `macroquad::window::next_frame` is `async`.
    pub async fn run(&mut self) {
        loop {
            let dt = get_frame_time();

            // Input is sampled every frame for responsiveness.
            handle_input(&mut self.state);

            // Restart when the player presses R on the game-over screen.
            if self.state.status == GameStatus::GameOver && is_key_pressed(KeyCode::R) {
                self.restart();
            }

            // Advance logic ticks (fixed time-step, may run multiple times
            // per frame if the frame was slow).
            if self.state.status == GameStatus::Running {
                self.tick_accumulator += dt;
                let interval = self.state.config.tick_interval();
                while self.tick_accumulator >= interval {
                    self.tick_accumulator -= interval;
                    self.logic.tick(&mut self.state);
                }
            }

            render(&self.state);
            next_frame().await;
        }
    }

    // -----------------------------------------------------------------------
    // Private helpers
    // -----------------------------------------------------------------------

    fn restart(&mut self) {
        let config = self.state.config.clone();
        self.state = GameState::new(config);
        self.tick_accumulator = 0.0;
    }
}
