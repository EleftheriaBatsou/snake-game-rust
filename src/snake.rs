/// Snake entity and movement logic.
use std::collections::LinkedList;
use piston_window::{Context, G2d};
use piston_window::types::Color;

use crate::draw::draw_block;

const SNAKE_COLOR: Color = [0.00, 0.80, 0.00, 1.0];

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Debug, Clone)]
struct Block {
    x: i32,
    y: i32,
}

pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>,
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();
        body.push_back(Block { x: x + 2, y });
        body.push_back(Block { x: x + 1, y });
        body.push_back(Block { x, y });

        Snake {
            direction: Direction::Right,
            body,
            tail: None,
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for block in &self.body {
            draw_block(SNAKE_COLOR, block.x, block.y, con, g);
        }
    }

    pub fn head_position(&self) -> (i32, i32) {
        let head_block = self
            .body
            .front()
            .expect("Snake body should never be empty");
        (head_block.x, head_block.y)
    }

    pub fn move_forward(&mut self, dir: Option<Direction>) {
        if let Some(d) = dir {
            self.direction = d;
        }

        let (last_x, last_y) = self.head_position();

        let new_block = match self.direction {
            Direction::Up => Block { x: last_x, y: last_y - 1 },
            Direction::Down => Block { x: last_x, y: last_y + 1 },
            Direction::Left => Block { x: last_x - 1, y: last_y },
            Direction::Right => Block { x: last_x + 1, y: last_y },
        };

        self.body.push_front(new_block);
        if let Some(removed_block) = self.body.pop_back() {
            self.tail = Some(removed_block);
        }
    }

    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
        let (head_x, head_y) = self.head_position();
        let moving_dir = dir.unwrap_or(self.direction);

        match moving_dir {
            Direction::Up => (head_x, head_y - 1),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x - 1, head_y),
            Direction::Right => (head_x + 1, head_y),
        }
    }

    pub fn restore_tail(&mut self) {
        if let Some(blk) = self.tail.take() {
            self.body.push_back(blk);
        }
    }

    pub fn overlap_tail(&self, x: i32, y: i32) -> bool {
        let len = self.body.len();
        if len == 0 {
            return false;
        }

        self.body
            .iter()
            .take(len - 1)
            .any(|block| block.x == x && block.y == y)
    }
}

#[cfg(test)]
mod tests {
    use super::{Direction, Snake};

    #[test]
    fn next_head_default_direction_is_right() {
        let snake = Snake::new(2, 2);
        let (nx, ny) = snake.next_head(None);
        assert_eq!((nx, ny), (3, 2));
    }

    #[test]
    fn overlap_tail_excludes_current_tail() {
        let snake = Snake::new(2, 2);
        // Initial tail is at the starting x,y (2,2)
        assert_eq!(snake.overlap_tail(2, 2), false);
        // But overlapping one of the body blocks returns true
        assert!(snake.overlap_tail(3, 2));
        assert!(snake.overlap_tail(4, 2));
    }

    #[test]
    fn move_forward_updates_head() {
        let mut snake = Snake::new(2, 2);
        snake.move_forward(Some(Direction::Down));
        let (hx, hy) = snake.head_position();
        assert_eq!((hx, hy), (4, 3)); // moved from (4,2) to (4,3)
    }
}

            ch += 1;
            if ch == self.body.len() - 1 {
                break;
            }
        }
        return false;
    }
}