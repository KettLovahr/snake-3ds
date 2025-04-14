use ctru::{prelude::KeyPad, services::gfx::RawFrameBuffer};
use rand::random;
use crate::{color, kett_draw_handle::draw_rectangle};

static DEFAULT_WORLD: World = World{
        width: 40,
        height: 24,
        scale: 10,
        tick_delay: 6,
        food: Position { x: 12, y: 8 },
};

pub struct Game {
    world: World,
    snake: Snake
}

impl Game {
    pub fn step(&mut self, keys: &KeyPad) {
        self.snake.update(keys, &mut self.world);
    }

    pub fn draw(&self, fb: &RawFrameBuffer) {
        self.snake.draw(fb, &self.world);
    }

    pub fn new() -> Self {
        let default_snake = Snake::new(Position { x: 5, y: 5 }, 5, Direction::Right);
        Game {
            world: DEFAULT_WORLD.clone(),
            snake: default_snake,
        }
    }

    pub fn restart(&mut self) {
        let default_snake = Snake::new(Position { x: 5, y: 5 }, 5, Direction::Right);
        self.snake = default_snake;
        self.world = DEFAULT_WORLD.clone();
    }

    pub fn get_score(&self) -> u32 {
        self.snake.score
    }
}


#[derive(PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
struct Position {
    x: isize,
    y: isize,
}

impl std::ops::Sub for Position {
    type Output = Position;

    fn sub(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Clone)]
struct World {
    width: isize,
    height: isize,
    scale: isize,
    tick_delay: u32,
    food: Position,
}

#[derive(Clone)]
struct Snake {
    body: Vec<Position>,
    alive: bool,
    direction: Direction,
    ticker: u32,
    score: u32,
}

impl Snake {
    fn new(pos: Position, length: u32, dir: Direction) -> Self {
        Snake {
            body: (0..length)
                .map(|n| match dir {
                    Direction::Up => Position {
                        x: pos.x,
                        y: pos.y + n as isize,
                    },
                    Direction::Right => Position {
                        x: pos.x - n as isize,
                        y: pos.y,
                    },
                    Direction::Down => Position {
                        x: pos.x,
                        y: pos.y - n as isize,
                    },
                    Direction::Left => Position {
                        x: pos.x + n as isize,
                        y: pos.y,
                    },
                })
                .collect(),
            alive: true,
            direction: dir,
            ticker: 0,
            score: 0,
        }
    }

    fn update(self: &mut Self, keys: &KeyPad, world: &mut World) {
        self.ticker += 1;

        if self.ticker % world.tick_delay == 0 && self.alive {
            self.handle_input(keys);

            let new_body: Vec<Position> = self
                .body
                .iter()
                .enumerate()
                .map(|(i, val)| {
                    if i == 0 {
                        match self.direction {
                            Direction::Up => Position {
                                x: val.x,
                                y: emod(val.y + 1, world.height),
                            },
                            Direction::Right => Position {
                                x: emod(val.x + 1, world.width),
                                y: val.y,
                            },
                            Direction::Down => Position {
                                x: val.x,
                                y: emod(val.y - 1, world.height),
                            },
                            Direction::Left => Position {
                                x: emod(val.x - 1, world.width),
                                y: val.y,
                            },
                        }
                    } else {
                        if *val == self.body[0] {
                            self.alive = false;
                        }
                        self.body[i - 1]
                    }
                })
                .collect();

            if self.alive {
                self.body = new_body;
            }

            if self.body[0] == world.food {
                let new_body: Vec<Position> = (0..self.body.len() + 3)
                    .into_iter()
                    .map(|x| {
                        if x < self.body.len() {
                            self.body[x]
                        } else {
                            self.body[self.body.len() - 1]
                        }
                    })
                    .collect();
                self.body = new_body;
                self.score += 1;
                while self.body.contains(&world.food) {
                    world.food = Position {
                        x: (random::<i32>() as isize % world.width).abs(),
                        y: (random::<i32>() as isize % world.height).abs(),
                    };
                }
            }
        }
    }

    fn handle_input(&mut self, keys: &KeyPad) {
        let mut dir_queue = self.direction;
        if keys.contains(KeyPad::DPAD_UP) {
            dir_queue = Direction::Up;
        }
        if keys.contains(KeyPad::DPAD_DOWN) {
            dir_queue = Direction::Down;
        }
        if keys.contains(KeyPad::DPAD_LEFT) {
            dir_queue = Direction::Left;
        }
        if keys.contains(KeyPad::DPAD_RIGHT) {
            dir_queue = Direction::Right;
        }

        if dir_queue != self.direction.opposite() {
            self.direction = dir_queue;
        }
    }

    fn draw(&self, fb: &RawFrameBuffer, world: &World) {
        self.body.iter().enumerate().for_each(|(x, pos)| {
            if (x == 0 || x == self.body.len() - 1) && self.alive {
                let len = self.body.len() - 1;
                let ev = if x == 0 {
                    *pos - self.body[1]
                } else {
                    *pos - self.body[len - 1]
                };
                let op = if x == 0 {
                    (self.ticker % world.tick_delay) as f32 / world.tick_delay as f32
                } else {
                    1.0 - ((self.ticker % world.tick_delay) as f32 / world.tick_delay as f32)
                };
                match ev {
                    Position { x: -1, y: 0 } => {
                        draw_rectangle(
                            fb,
                            ((pos.x + 1) as isize * world.scale as isize) - (op * world.scale as f32) as isize,
                            pos.y * world.scale,
                            world.scale,
                            world.scale,
                            if self.alive { color::WHITE } else { color::RED },
                        );
                    }
                    Position { x: 0, y: -1 } => {
                        draw_rectangle(
                            fb,
                            pos.x * world.scale,
                            ((pos.y + 1) * world.scale) - (op * world.scale as f32) as isize,
                            world.scale,
                            world.scale,
                            if self.alive { color::WHITE } else { color::RED },
                        );
                    }
                    Position { x: 0, y: 1 } => {
                        draw_rectangle(
                            fb,
                            pos.x * world.scale,
                            pos.y * world.scale,
                            world.scale,
                            (world.scale as f32 * op) as isize,
                            if self.alive { color::WHITE } else { color::RED },
                        );
                    }
                    Position { x: 1, y: 0 } => {
                        draw_rectangle(
                            fb,
                            pos.x * world.scale,
                            pos.y * world.scale,
                            (world.scale as f32 * op) as isize,
                            world.scale,
                            if self.alive { color::WHITE } else { color::RED },
                        );
                    }
                    _ => {}
                }
            } else {
                draw_rectangle(
                    fb,
                    pos.x * world.scale,
                    pos.y * world.scale,
                    world.scale,
                    world.scale,
                    if self.alive { color::WHITE } else { color::RED },
                );
            }
        });

        draw_rectangle(
            fb,
            world.food.x * world.scale,
            world.food.y * world.scale,
            world.scale,
            world.scale,
            color::ORANGE,
        );

        // let b = format!("Score: {:0>3}", self.score);
        // handle.draw_text(&b, 0, 0, 20, color::GREEN);
    }
}

const fn emod(l: isize, r: isize) -> isize {
    ((l % r) + r) % r
}

