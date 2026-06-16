use macroquad::input::{is_key_pressed, KeyCode};

use crate::game::state::{Direction, GameState, GameStatus};

/// Poll keyboard input and update `state` accordingly.
///
/// Called every rendered frame so that direction changes are never missed.
/// The engine handles restarting so this module only needs to buffer directions
/// and toggle pause/resume.
pub fn handle_input(state: &mut GameState) {
    // --- Direction buffering (arrows and WASD) ---
    if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
        state.next_direction = Direction::Up;
    } else if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
        state.next_direction = Direction::Down;
    } else if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A) {
        state.next_direction = Direction::Left;
    } else if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D) {
        state.next_direction = Direction::Right;
    }

    // --- Pause / resume (P or Escape) ---
    if is_key_pressed(KeyCode::P) || is_key_pressed(KeyCode::Escape) {
        match state.status {
            GameStatus::Running => state.status = GameStatus::Paused,
            GameStatus::Paused => state.status = GameStatus::Running,
            GameStatus::GameOver => {}
        }
    }
}
