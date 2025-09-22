/// Game state and logic: input handling, updates, and rendering.
use piston_window::{Context, G2d, Key};
use rand::{thread_rng, Rng};

use crate::config::{
    BORDER_COLOR, FOOD_COLOR, GAMEOVER_COLOR, INITIAL_FOOD_X, INITIAL_FOOD_Y, MOVING_PERIOD,
    RESTART_TIME, START_SNAKE_X, START_SNAKE_Y,
};
use crate::draw::{draw_block, draw_rectangle};
use crate::snake::{Direction, Snake};

pub struct Game {
    snake: Snake,

    food_exists: bool,
    food_x: i32,
    food_y: i32,

    width: i32,
    height: i32,

    game_over: bool,
    waiting_time: f64,

    paused: bool,
    score: u32,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            snake: Snake::new(START_SNAKE_X, START_SNAKE_Y),
            waiting_time: 0.0,
            food_exists: true,
            food_x: INITIAL_FOOD_X,
            food_y: INITIAL_FOOD_Y,
            width,
            height,
            game_over: false,
            paused: false,
            score: 0,
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        // Allow toggling pause even when game over; restart logic handles reset separately.
        if key == Key::Space {
            self.paused = !self.paused;
            return;
        }

        if self.game_over {
            return;
        }

        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => None,
        };

        if let Some(d) = dir {
            if d == self.snake.head_direction().opposite() {
                return;
            }
        }

        if !self.paused {
            self.update_snake(dir);
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.snake.draw(con, g);

        if self.food_exists {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, con, g);
        }

        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);

        if self.game_over {
            draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g);
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }

        if self.paused {
            return;
        }

        if !self.food_exists {
            self.add_food();
        }

        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
        }
    }

    fn check_eating(&mut self) {
        let (head_x, head_y): (i32, i32) = self.snake.head_position();
        if self.food_exists && self.food_x == head_x && self.food_y == head_y {
            self.food_exists = false;
            self.snake.restore_tail();
            self.score += 1;
            println!("Score: {}", self.score);
        }
    }

    fn check_if_snake_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(dir);

        if self.snake.overlap_tail(next_x, next_y) {
            return false;
        }

        next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1
    }

    fn add_food(&mut self) {
        let mut rng = thread_rng();

        let mut new_x = rng.gen_range(1..self.width - 1);
        let mut new_y = rng.gen_range(1..self.height - 1);
        while self.snake.overlap_tail(new_x, new_y) {
            new_x = rng.gen_range(1..self.width - 1);
            new_y = rng.gen_range(1..self.height - 1);
        }

        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exists = true;
    }

    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_if_snake_alive(dir) {
            self.snake.move_forward(dir);
            self.check_eating();
        } else {
            self.game_over = true;
        }
        self.waiting_time = 0.0;
    }

    fn restart(&mut self) {
        self.snake = Snake::new(START_SNAKE_X, START_SNAKE_Y);
        self.waiting_time = 0.0;
        self.food_exists = true;
        self.food_x = INITIAL_FOOD_X;
        self.food_y = INITIAL_FOOD_Y;
        self.game_over = false;
        self.paused = false;
        self.score = 0;
    }
}

    fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.waiting_time = 0.0;
        self.food_exists = true;
        self.food_x = 6;
        self.food_y = 4;
        self.game_over = false;
    }
}