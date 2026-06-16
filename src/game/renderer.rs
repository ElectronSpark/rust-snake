use macroquad::prelude::*;

use crate::game::state::{GameState, GameStatus};

// ---------------------------------------------------------------------------
// Colour palette
// ---------------------------------------------------------------------------

const COLOR_BACKGROUND: Color = Color::new(0.08, 0.08, 0.08, 1.0);
const COLOR_GRID: Color = Color::new(0.14, 0.14, 0.14, 1.0);
const COLOR_SNAKE_HEAD: Color = Color::new(0.22, 0.82, 0.22, 1.0);
const COLOR_SNAKE_BODY: Color = Color::new(0.10, 0.55, 0.10, 1.0);
const COLOR_FOOD: Color = Color::new(0.90, 0.20, 0.20, 1.0);
const COLOR_HUD: Color = WHITE;
const COLOR_OVERLAY: Color = Color::new(0.0, 0.0, 0.0, 0.65);

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

/// Draw the complete current frame from `state`.
///
/// Called once per rendered frame by the [`GameEngine`].
///
/// [`GameEngine`]: crate::game::engine::GameEngine
pub fn render(state: &GameState) {
    let cfg = &state.config;
    let cw = cfg.cell_width();
    let ch = cfg.cell_height();

    clear_background(COLOR_BACKGROUND);

    draw_grid(cfg.grid_cols, cfg.grid_rows, cw, ch, cfg.window_width, cfg.window_height);
    draw_food(state.food.x, state.food.y, cw, ch);
    draw_snake(&state.snake, cw, ch);
    draw_hud(state.score);

    match state.status {
        GameStatus::Paused => draw_overlay("PAUSED", "P / Esc  ·  resume"),
        GameStatus::GameOver => {
            let title = format!("GAME OVER  ·  Score: {}", state.score);
            draw_overlay(&title, "R  ·  restart");
        }
        GameStatus::Running => {}
    }
}

// ---------------------------------------------------------------------------
// Private helpers
// ---------------------------------------------------------------------------

fn draw_grid(cols: usize, rows: usize, cw: f32, ch: f32, win_w: f32, win_h: f32) {
    for col in 0..=cols {
        let x = col as f32 * cw;
        draw_line(x, 0.0, x, win_h, 1.0, COLOR_GRID);
    }
    for row in 0..=rows {
        let y = row as f32 * ch;
        draw_line(0.0, y, win_w, y, 1.0, COLOR_GRID);
    }
}

fn draw_food(fx: i32, fy: i32, cw: f32, ch: f32) {
    let margin = 3.0;
    draw_rectangle(
        fx as f32 * cw + margin,
        fy as f32 * ch + margin,
        cw - margin * 2.0,
        ch - margin * 2.0,
        COLOR_FOOD,
    );
}

fn draw_snake(snake: &std::collections::VecDeque<crate::game::state::Position>, cw: f32, ch: f32) {
    let margin = 1.0;
    for (i, seg) in snake.iter().enumerate() {
        let color = if i == 0 { COLOR_SNAKE_HEAD } else { COLOR_SNAKE_BODY };
        draw_rectangle(
            seg.x as f32 * cw + margin,
            seg.y as f32 * ch + margin,
            cw - margin * 2.0,
            ch - margin * 2.0,
            color,
        );
    }
}

fn draw_hud(score: u32) {
    let text = format!("Score: {score}");
    draw_text(&text, 8.0, 22.0, 26.0, COLOR_HUD);
}

fn draw_overlay(title: &str, subtitle: &str) {
    let sw = screen_width();
    let sh = screen_height();

    draw_rectangle(0.0, 0.0, sw, sh, COLOR_OVERLAY);

    let title_fs: u16 = 40;
    let sub_fs: u16 = 22;

    let tm = measure_text(title, None, title_fs, 1.0);
    let sm = measure_text(subtitle, None, sub_fs, 1.0);

    draw_text(
        title,
        (sw - tm.width) / 2.0,
        sh / 2.0 - 16.0,
        title_fs as f32,
        COLOR_HUD,
    );
    draw_text(
        subtitle,
        (sw - sm.width) / 2.0,
        sh / 2.0 + 22.0,
        sub_fs as f32,
        COLOR_HUD,
    );
}
