/// Configuration for the snake game.
///
/// Pass a `GameConfig` to [`GameEngine::new`] to control the window size,
/// grid dimensions, and game speed.  All fields are public so you can adjust
/// them freely; call [`GameConfig::default`] to get sensible starting values.
#[derive(Clone, Debug)]
pub struct GameConfig {
    /// Number of columns in the grid.
    pub grid_cols: usize,
    /// Number of rows in the grid.
    pub grid_rows: usize,
    /// How many game-logic ticks happen per second (controls snake speed).
    pub ticks_per_second: f32,
    /// Width of the window in pixels.
    pub window_width: f32,
    /// Height of the window in pixels.
    pub window_height: f32,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            grid_cols: 20,
            grid_rows: 20,
            ticks_per_second: 8.0,
            window_width: 600.0,
            window_height: 600.0,
        }
    }
}

impl GameConfig {
    /// Width of a single grid cell in pixels.
    pub fn cell_width(&self) -> f32 {
        self.window_width / self.grid_cols as f32
    }

    /// Height of a single grid cell in pixels.
    pub fn cell_height(&self) -> f32 {
        self.window_height / self.grid_rows as f32
    }

    /// Seconds between consecutive game ticks.
    pub fn tick_interval(&self) -> f32 {
        1.0 / self.ticks_per_second
    }
}
