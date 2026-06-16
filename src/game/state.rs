use std::collections::VecDeque;

use crate::game::config::GameConfig;

// ---------------------------------------------------------------------------
// Position
// ---------------------------------------------------------------------------

/// A discrete cell coordinate on the game grid.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

// ---------------------------------------------------------------------------
// Direction
// ---------------------------------------------------------------------------

/// The four cardinal directions the snake can travel.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    /// Returns `true` when `other` is the exact opposite direction.
    pub fn is_opposite(self, other: Direction) -> bool {
        matches!(
            (self, other),
            (Direction::Up, Direction::Down)
                | (Direction::Down, Direction::Up)
                | (Direction::Left, Direction::Right)
                | (Direction::Right, Direction::Left)
        )
    }

    /// The (dx, dy) unit vector for this direction.
    pub fn to_delta(self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

// ---------------------------------------------------------------------------
// GameStatus
// ---------------------------------------------------------------------------

/// Current phase of the game.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GameStatus {
    Running,
    Paused,
    GameOver,
}

// ---------------------------------------------------------------------------
// GameState
// ---------------------------------------------------------------------------

/// Everything the backbone and game logic need to know about the current game.
///
/// The backbone reads this struct for rendering and game-loop control.
/// The [`GameLogic`] implementation reads and mutates it during each tick.
///
/// [`GameLogic`]: crate::game::logic::GameLogic
pub struct GameState {
    /// Ordered body segments; index `0` is the head.
    pub snake: VecDeque<Position>,
    /// Current position of the food item.
    pub food: Position,
    /// Direction the snake is currently moving.
    pub direction: Direction,
    /// Buffered input direction applied at the start of the next tick.
    pub next_direction: Direction,
    /// Player score (incremented by game logic on food consumption).
    pub score: u32,
    /// Whether the game is running, paused, or over.
    pub status: GameStatus,
    /// Immutable grid / window configuration.
    pub config: GameConfig,
}

impl GameState {
    /// Creates a fresh game state with the snake centred on the grid.
    pub fn new(config: GameConfig) -> Self {
        let mid_x = (config.grid_cols / 2) as i32;
        let mid_y = (config.grid_rows / 2) as i32;

        let mut snake = VecDeque::new();
        snake.push_back(Position::new(mid_x, mid_y));
        snake.push_back(Position::new(mid_x - 1, mid_y));
        snake.push_back(Position::new(mid_x - 2, mid_y));

        Self {
            food: Position::new(mid_x + 5, mid_y),
            snake,
            direction: Direction::Right,
            next_direction: Direction::Right,
            score: 0,
            status: GameStatus::Running,
            config,
        }
    }

    /// Returns `true` when `pos` lies within the grid boundaries.
    pub fn is_in_bounds(&self, pos: Position) -> bool {
        pos.x >= 0
            && pos.y >= 0
            && pos.x < self.config.grid_cols as i32
            && pos.y < self.config.grid_rows as i32
    }

    /// Returns `true` when any snake segment occupies `pos`.
    pub fn snake_occupies(&self, pos: Position) -> bool {
        self.snake.contains(&pos)
    }

    /// The current head position.
    pub fn head(&self) -> Position {
        *self.snake.front().expect("snake must not be empty")
    }
}
