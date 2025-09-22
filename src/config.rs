/// Game configuration constants.
///
/// Centralizing constants helps keep behavior consistent and makes it easier
/// to tune gameplay without hunting through the codebase.
use piston_window::types::Color;

// Colors
pub const FOOD_COLOR: Color = [0.80, 0.00, 0.00, 1.0];
pub const BORDER_COLOR: Color = [0.00, 0.00, 0.00, 1.0];
pub const GAMEOVER_COLOR: Color = [0.90, 0.00, 0.00, 0.5];

// Timing
pub const MOVING_PERIOD: f64 = 0.1;
pub const RESTART_TIME: f64 = 1.0;

// Starting positions
pub const START_SNAKE_X: i32 = 2;
pub const START_SNAKE_Y: i32 = 2;

// Initial food position
pub const INITIAL_FOOD_X: i32 = 6;
pub const INITIAL_FOOD_Y: i32 = 4;