use rand::Rng;
use geometry::*;
use renderer::{Renderable, GameState};

pub struct Snake {
    food: u8,
    segments: Vec<Point>,
}

impl Snake {
    fn random_in_bounds<R: Rng>(r: &mut R, bounds: &Bounds) -> Snake {
        let head = Point::random_in_bounds(r, bounds);
        return Snake { food: 0, segments: vec![head] }
    }

    pub fn get_segments(&self) -> &Vec<Point> {
        &self.segments
    }

    fn next_head(&self, direction: &Direction) -> Point {
        let (xdiff, ydiff) = direction.diff();
        let mut head = self.segments[0].clone();
        head.translate(xdiff, ydiff);
        head
    }

    fn move_direction(&mut self, direction: &Direction) {
        let new_head = self.next_head(direction);
        self.segments.insert(0, new_head);
        if 0 == self.food {
            let last_index = self.segments.len() - 1;
            self.segments.remove(last_index);
        } else {
            self.food = self.food - 1;
        }
    }

    fn feed(&mut self, value: u8) {
        self.food += value;
    }

    fn contains(&self, p: &Point) -> bool {
        self.segments.contains(p)
    }
}

pub struct Food {
    pub pos: Point,
    pub value: u8,
}

impl Food {
    fn random_nonconflicting<R: Rng>(r: &mut R, bounds: &Bounds, snake: &Snake) -> Food {
        while {
            let p = Point::random_in_bounds(r, bounds);
            if !snake.contains(&p) {
                return Food { pos: p, value: 1 };
            }
            true
        } {}
        panic!("unreachable code")
    }
}

pub struct NonInitializedGame<R: Rng> {
    rng: R,
    bounds: Bounds,
    snake: Snake,
    food: Food,
}

impl <R: Rng> NonInitializedGame<R> {
    pub fn new(mut rng: R, bounds: Bounds) -> NonInitializedGame<R> {
        let snake = Snake::random_in_bounds(&mut rng, &bounds);
        let food = Food::random_nonconflicting(&mut rng, &bounds, &snake);
        NonInitializedGame { rng, bounds, snake, food }
    }

    pub fn start(self, direction: Direction) -> Game<R> {
        Game {
            rng: self.rng,
            bounds: self.bounds,
            snake: self.snake,
            food: self.food,
            direction,
        }
    }
}

impl <R: Rng> Renderable for NonInitializedGame<R> {
    fn state(&self) -> GameState {
        GameState {
            bounds: &self.bounds,
            food: &self.food,
            snake: &self.snake,
        }
    }
}

pub struct Game<R : Rng> {
    rng: R,
    bounds: Bounds,

    pub direction: Direction,
    snake: Snake,

    food: Food,
}

impl <R : Rng> Game<R> {
    pub fn tick(&mut self) -> bool {
        let next_head = self.snake.next_head(&self.direction);
        if !self.bounds.contains(&next_head) || self.snake.contains(&next_head) {
            return false;
        }
        self.snake.move_direction(&self.direction);
        if self.food.pos == next_head {
            self.snake.feed(self.food.value);
            self.food = Food::random_nonconflicting(&mut self.rng, &self.bounds, &self.snake);
        }
        true
    }
}

impl <R : Rng> Renderable for Game<R> {
    fn state(&self) -> GameState {
        GameState {
            bounds: &self.bounds,
            food: &self.food,
            snake: &self.snake,
        }
    }
}

