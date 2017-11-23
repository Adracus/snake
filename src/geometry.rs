extern crate rand;

use rand::Rng;

#[derive(Clone, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn translate(&mut self, xdiff: i32, ydiff: i32) {
        self.x += xdiff;
        self.y += ydiff;
    }

    pub fn random_in_bounds<R: Rng>(rng: &mut R, bounds: &Bounds) -> Point {
        let x = rng.gen_range(0, bounds.width) as i32;
        let y = rng.gen_range(0, bounds.height) as i32;
        Point { x, y }
    }
}

pub struct Bounds {
    pub width: u8,
    pub height: u8,
}

impl Bounds {
    pub fn new(width: u8, height: u8) -> Bounds {
        Bounds { width, height }
    }

    pub fn contains(&self, &Point { x, y }: &Point) -> bool {
        x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32
    }

    pub fn area(&self) -> i32 {
        self.width as i32 * self.height as i32
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn diff(&self) -> (i32, i32) {
        match self {
            &Direction::Up => (0, -1),
            &Direction::Right => (1, 0),
            &Direction::Down => (0, 1),
            &Direction::Left => (-1, 0),
        }
    }

    pub fn is_opposite(&self, other: &Direction) -> bool {
        match self {
            &Direction::Up => other == &Direction::Down,
            &Direction::Right => other == &Direction::Left,
            &Direction::Down => other == &Direction::Up,
            &Direction::Left => other == &Direction::Right,
        }
    }
}

