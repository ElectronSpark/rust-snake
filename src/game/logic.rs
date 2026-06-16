use rand::Rng;

use crate::game::state::{GameState, GameStatus, Position};

// ---------------------------------------------------------------------------
// Public trait
// ---------------------------------------------------------------------------

/// The interface between the game backbone and your snake game logic.
///
/// # How it works
///
/// [`GameEngine`] calls [`tick`] once per logical game tick while
/// [`GameStatus`] is [`Running`].  You read and mutate [`GameState`] freely:
/// move the snake, check collisions, consume food, update the score, or set
/// `state.status = GameStatus::GameOver`.
///
/// # Minimal example
///
/// ```rust,ignore
/// use rust_snake::game::{
///     logic::GameLogic,
///     state::{GameState, GameStatus},
/// };
///
/// struct MyLogic;
///
/// impl GameLogic for MyLogic {
///     fn tick(&mut self, state: &mut GameState) {
///         // your custom snake rules here
///     }
/// }
/// ```
///
/// [`GameEngine`]: crate::game::engine::GameEngine
/// [`tick`]: GameLogic::tick
/// [`Running`]: GameStatus::Running
pub trait GameLogic {
    /// Advance the game by one tick.
    ///
    /// The backbone has already updated `state.direction` from the buffered
    /// `state.next_direction` when the move is valid (not a 180° reversal),
    /// so you can rely on `state.direction` being correct.
    fn tick(&mut self, state: &mut GameState);
}

// ---------------------------------------------------------------------------
// Default implementation
// ---------------------------------------------------------------------------

/// A ready-to-use implementation of classic snake rules.
///
/// * The snake moves one cell per tick in `state.direction`.
/// * Hitting a wall or the snake's own body ends the game.
/// * Eating the food grows the snake by one segment and increments the score.
/// * New food is spawned at a random free cell after consumption.
///
/// You can replace this with your own [`GameLogic`] implementation to
/// customise any of the above behaviours.
pub struct DefaultGameLogic;

impl DefaultGameLogic {
    pub fn new() -> Self {
        Self
    }

    /// Compute the position the head would move to this tick.
    fn next_head(state: &GameState) -> Position {
        let head = state.head();
        let (dx, dy) = state.direction.to_delta();
        Position::new(head.x + dx, head.y + dy)
    }

    /// Pick a random grid cell not currently occupied by the snake.
    fn spawn_food(state: &GameState) -> Position {
        let mut rng = rand::thread_rng();
        loop {
            let x = rng.gen_range(0..state.config.grid_cols as i32);
            let y = rng.gen_range(0..state.config.grid_rows as i32);
            let pos = Position::new(x, y);
            if !state.snake_occupies(pos) {
                return pos;
            }
        }
    }
}

impl Default for DefaultGameLogic {
    fn default() -> Self {
        Self::new()
    }
}

impl GameLogic for DefaultGameLogic {
    fn tick(&mut self, state: &mut GameState) {
        // Commit the buffered direction only if it is not a direct reversal.
        if !state.direction.is_opposite(state.next_direction) {
            state.direction = state.next_direction;
        }

        let new_head = Self::next_head(state);

        // Out-of-bounds → game over.
        if !state.is_in_bounds(new_head) {
            state.status = GameStatus::GameOver;
            return;
        }

        // Self-collision: the tail will vacate its cell this tick, so only
        // treat it as a collision if the new head lands on a body segment
        // that is *not* the current tail.
        let tail = *state.snake.back().unwrap();
        if state.snake_occupies(new_head) && new_head != tail {
            state.status = GameStatus::GameOver;
            return;
        }

        // Advance the snake.
        state.snake.push_front(new_head);

        if new_head == state.food {
            // Food consumed: grow (do not remove tail), update score, respawn food.
            state.score += 1;
            state.food = Self::spawn_food(state);
        } else {
            state.snake.pop_back();
        }
    }
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::{
        config::GameConfig,
        state::{Direction, GameStatus, Position},
    };

    fn make_state() -> GameState {
        GameState::new(GameConfig::default())
    }

    #[test]
    fn snake_moves_forward() {
        let mut state = make_state();
        let old_head = state.head();
        let mut logic = DefaultGameLogic::new();
        logic.tick(&mut state);
        let new_head = state.head();
        assert_eq!(new_head.x, old_head.x + 1);
        assert_eq!(new_head.y, old_head.y);
    }

    #[test]
    fn game_over_on_wall_hit() {
        let mut state = make_state();
        // Drive the snake into the right wall.
        let mut logic = DefaultGameLogic::new();
        for _ in 0..state.config.grid_cols + 5 {
            if state.status != GameStatus::Running {
                break;
            }
            logic.tick(&mut state);
        }
        assert_eq!(state.status, GameStatus::GameOver);
    }

    #[test]
    fn direction_reversal_is_ignored() {
        let mut state = make_state();
        assert_eq!(state.direction, Direction::Right);
        state.next_direction = Direction::Left; // attempt 180°
        let mut logic = DefaultGameLogic::new();
        logic.tick(&mut state);
        // Direction must not have changed.
        assert_eq!(state.direction, Direction::Right);
    }

    #[test]
    fn eating_food_grows_snake_and_increments_score() {
        let config = GameConfig::default();
        let mid_x = (config.grid_cols / 2) as i32;
        let mid_y = (config.grid_rows / 2) as i32;

        let mut state = GameState::new(config);
        // Place food directly ahead of the head.
        state.food = Position::new(mid_x + 1, mid_y);
        let initial_len = state.snake.len();

        let mut logic = DefaultGameLogic::new();
        logic.tick(&mut state);

        assert_eq!(state.score, 1);
        assert_eq!(state.snake.len(), initial_len + 1);
    }

    #[test]
    fn no_food_does_not_grow_snake() {
        let mut state = make_state();
        // Move food far away so the first tick does not eat it.
        state.food = Position::new(0, 0);
        let initial_len = state.snake.len();

        let mut logic = DefaultGameLogic::new();
        logic.tick(&mut state);

        assert_eq!(state.score, 0);
        assert_eq!(state.snake.len(), initial_len);
    }

    #[test]
    fn self_collision_ends_game() {
        // Build a state where the snake immediately occupies the cell ahead.
        let config = GameConfig::default();
        let mut state = GameState::new(config);
        // Manually inject a segment right in front of the head.
        let head = state.head();
        let (dx, dy) = state.direction.to_delta();
        let in_front = Position::new(head.x + dx, head.y + dy);
        // Insert it as a body segment (not the tail so it won't vacate).
        let tail_index = state.snake.len() - 1;
        state.snake.insert(tail_index, in_front);

        let mut logic = DefaultGameLogic::new();
        logic.tick(&mut state);

        assert_eq!(state.status, GameStatus::GameOver);
    }

    #[test]
    fn spawn_food_not_on_snake() {
        let state = make_state();
        for _ in 0..50 {
            let food = DefaultGameLogic::spawn_food(&state);
            assert!(!state.snake_occupies(food));
        }
    }

    #[test]
    fn is_in_bounds() {
        let state = make_state();
        assert!(state.is_in_bounds(Position::new(0, 0)));
        assert!(!state.is_in_bounds(Position::new(-1, 0)));
        assert!(!state.is_in_bounds(Position::new(0, state.config.grid_rows as i32)));
    }
}
