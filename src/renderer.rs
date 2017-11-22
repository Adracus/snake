use logic::*;
use geometry::*;
use ncurses::*;
use std;

pub trait Renderer {
    fn render(&mut self, renderable: &Renderable);

    fn get_direction(&mut self) -> Option<Direction>;

    fn get_direction_nonblocking(&mut self) -> Option<Direction>;
}

pub trait Renderable {
    fn state(&self) -> GameState;
}

pub struct GameState<'a> {
    pub bounds: &'a Bounds,
    pub snake: &'a Snake,
    pub food: &'a Food,
}

pub struct NCurses {}

impl NCurses {
    pub fn new() -> NCurses {
        initscr();
        noecho();
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        raw();
        keypad(stdscr(), true);
        NCurses{}
    }
}

impl NCurses {
    fn draw_border(&mut self, bounds: &Bounds) {
        let horizontal_line =
            std::iter::repeat("X")
                .take((bounds.width + 1) as usize).collect::<String>();
        mv(0, 0);
        printw(&horizontal_line);
        mv(bounds.height as i32 + 1, 0);
        printw(&horizontal_line);
        for y in 0..(bounds.height as i32 + 1) {
            mv(y, 0);
            printw("X");
            mv(y, bounds.width as i32 + 1);
            printw("X");
        }
    }

    fn draw_snake(&mut self, snake: &Snake) {
        let segments = snake.get_segments();
        for segment in segments {
            mv(segment.y + 1, segment.x + 1);
            printw("s");
        }
    }

    fn draw_food(&mut self, food: &Food) {
        let pos = &food.pos;
        mv(pos.y + 1, pos.x + 1);
        printw(&food.value.to_string());
    }

    fn get_direction_internal(&mut self) -> Option<Direction> {
        match getch() {
            KEY_UP => Some(Direction::Up),
            KEY_RIGHT => Some(Direction::Right),
            KEY_DOWN => Some(Direction::Down),
            KEY_LEFT => Some(Direction::Left),
            _ => None,
        }
    }
}

impl Drop for NCurses {
    fn drop(&mut self) {
        endwin();
    }
}

impl Renderer for NCurses {
    fn render(&mut self, renderable: &Renderable) {
        let state = renderable.state();
        clear();
        self.draw_border(state.bounds);
        self.draw_snake(state.snake);
        self.draw_food(state.food);
        refresh();
    }

    fn get_direction(&mut self) -> Option<Direction> {
        timeout(-1);
        self.get_direction_internal()
    }

    fn get_direction_nonblocking(&mut self) -> Option<Direction> {
        timeout(0);
        self.get_direction_internal()
    }
}

