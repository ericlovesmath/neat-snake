use macroquad::prelude::*;

use std::collections::VecDeque;

type Point = (i16, i16);

#[derive(PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

struct SnakeGame {
    head: Point,
    body: VecDeque<Point>,
    dir: Direction,
    fruit: Point,
    score: i64,
    size: i16,
}

impl SnakeGame {
    pub fn new(size: i16) -> SnakeGame {
        SnakeGame {
            head: (0, 0),
            dir: Direction::Right,
            body: VecDeque::new(),
            fruit: (rand::gen_range(0, SQUARES), rand::gen_range(0, SQUARES)),
            score: 0,
            size,
        }
    }

    pub fn over(&self) -> bool {
        self.head.0 < 0
            || self.head.1 < 0
            || self.head.0 >= self.size
            || self.head.1 >= self.size
            || self
                .body
                .iter()
                .any(|(x, y)| *x == self.head.0 && *y == self.head.1)
    }

    pub fn advance(&mut self) {
        self.body.push_front(self.head);
        self.head = match self.dir {
            Direction::Up => (self.head.0, self.head.1 - 1),
            Direction::Down => (self.head.0, self.head.1 + 1),
            Direction::Right => (self.head.0 + 1, self.head.1),
            Direction::Left => (self.head.0 - 1, self.head.1),
        };
        if self.head == self.fruit {
            self.score += 1;
            self.fruit = (rand::gen_range(0, SQUARES), rand::gen_range(0, SQUARES));
        } else {
            self.body.pop_back();
        }
    }
}

const SQUARES: i16 = 16;
const SPEED: f64 = 0.3;

#[macroquad::main("Snake")]
async fn main() {
    let mut game = SnakeGame::new(SQUARES);
    let mut last_update = get_time();
    let mut dir_lock = false;

    loop {
        if !game.over() {
            if is_key_down(KeyCode::Right) && game.dir != Direction::Left && !dir_lock {
                game.dir = Direction::Right;
                dir_lock = true;
            } else if is_key_down(KeyCode::Left) && game.dir != Direction::Right && !dir_lock {
                game.dir = Direction::Left;
                dir_lock = true;
            } else if is_key_down(KeyCode::Up) && game.dir != Direction::Down && !dir_lock {
                game.dir = Direction::Up;
                dir_lock = true;
            } else if is_key_down(KeyCode::Down) && game.dir != Direction::Up && !dir_lock {
                game.dir = Direction::Down;
                dir_lock = true;
            }

            if get_time() - last_update > SPEED {
                last_update = get_time();
                game.advance();
                dir_lock = false;
            }
        }

        if !game.over() {
            clear_background(LIGHTGRAY);

            let game_size = screen_width().min(screen_height());
            let offset_x = (screen_width() - game_size) / 2. + 10.;
            let offset_y = (screen_height() - game_size) / 2. + 10.;
            let sq_size = (screen_height() - offset_y * 2.) / SQUARES as f32;

            draw_rectangle(offset_x, offset_y, game_size - 20., game_size - 20., WHITE);

            for i in 1..SQUARES {
                draw_line(
                    offset_x,
                    offset_y + sq_size * i as f32,
                    screen_width() - offset_x,
                    offset_y + sq_size * i as f32,
                    2.,
                    LIGHTGRAY,
                );
            }

            for i in 1..SQUARES {
                draw_line(
                    offset_x + sq_size * i as f32,
                    offset_y,
                    offset_x + sq_size * i as f32,
                    screen_height() - offset_y,
                    2.,
                    LIGHTGRAY,
                );
            }

            draw_rectangle(
                offset_x + game.head.0 as f32 * sq_size,
                offset_y + game.head.1 as f32 * sq_size,
                sq_size,
                sq_size,
                DARKGREEN,
            );

            for (x, y) in &game.body {
                draw_rectangle(
                    offset_x + *x as f32 * sq_size,
                    offset_y + *y as f32 * sq_size,
                    sq_size,
                    sq_size,
                    LIME,
                );
            }

            draw_rectangle(
                offset_x + game.fruit.0 as f32 * sq_size,
                offset_y + game.fruit.1 as f32 * sq_size,
                sq_size,
                sq_size,
                GOLD,
            );

            draw_text(format!("SCORE: {}", game.score).as_str(), 10., 20., 20., DARKGRAY);
        } else {
            clear_background(WHITE);
            let text = "Game Over. Press [enter] to play again.";
            let font_size = 30.;
            let text_size = measure_text(text, None, font_size as _, 1.0);

            draw_text(
                text,
                screen_width() / 2. - text_size.width / 2.,
                screen_height() / 2. + text_size.height / 2.,
                font_size,
                DARKGRAY,
            );

            if is_key_down(KeyCode::Enter) {
                game = SnakeGame::new(SQUARES);
                last_update = get_time();
            }
        }
        next_frame().await;
    }
}
